use chrono::Local;
use chrono::{DateTime, Utc};
use gloo_timers::future;
use leptos::wasm_bindgen::JsCast;
use leptos::wasm_bindgen::closure::Closure;
use leptos::{prelude::*, task::spawn_local};
use leptos_icons::Icon;
use serde::{Deserialize, Serialize};
use web_sys::{MessageEvent, WebSocket};

use crate::services::web::base_hostname;
use crate::{
    data_models::{contact::Contact, message::Message},
    services::{
        keys::{decrypt_message, encrypt_message, get_key_securely},
        web::base_url,
    },
    store::auth::get_current_user,
};

fn order_messages_by_timestamp(messages: &mut Vec<Message>) {
    messages.sort_by_key(|message| message.timestamp);
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct InputMessage {
    pub id: String,
    pub sender_id: String,
    pub receiver_id: String,
    pub sender_content: String,
    pub receiver_content: String,
    #[serde(rename = "sent_at")]
    pub timestamp: DateTime<Utc>,
}

async fn get_messages_with_contact(user_id: String, contact_id: String) -> Vec<Message> {
    let client = reqwest::Client::new();
    let messages = match client
        .get(
            base_url() + "/api/chats/" + user_id.as_str() + "/" + contact_id.as_str() + "/messages",
        )
        .send()
        .await
        .ok()
    {
        Some(response) => {
            if response.status().is_success() {
                match response.json::<Vec<InputMessage>>().await.ok() {
                    Some(messages) => messages,
                    None => vec![],
                }
            } else {
                vec![]
            }
        }
        None => vec![],
    };

    let my_priv_key = get_key_securely().await.unwrap();

    // 3. Transform the messages
    let mut decrypted_list = Vec::new();
    for msg in messages {
        let decrypted_content = if msg.sender_id == user_id {
            decrypt_message(&my_priv_key, &msg.sender_content)
                .await
                .unwrap_or_else(|_| "Error decrypting".to_string())
        } else {
            decrypt_message(&my_priv_key, &msg.receiver_content)
                .await
                .unwrap_or_else(|_| "Error decrypting".to_string())
        };

        decrypted_list.push(Message {
            id: msg.id,
            sender_id: msg.sender_id,
            receiver_id: msg.receiver_id,
            content: decrypted_content,
            timestamp: msg.timestamp,
        });
    }

    order_messages_by_timestamp(&mut decrypted_list);
    decrypted_list
}

#[derive(Debug, Serialize)]
pub struct CreateMessage {
    pub sender_id: String,
    pub receiver_id: String,
    pub sender_content: String,
    pub receiver_content: String,
}
async fn send_message_to_contact(
    user_id: String,
    contact_id: String,
    message: String,
    contact_pub_key: String,
    self_pub_key: String,
) -> Result<(), String> {
    let encrypted_text = match encrypt_message(contact_pub_key.as_str(), message.as_str()).await {
        Ok(cipher) => cipher,
        Err(e) => {
            leptos::logging::log!("ERROR: Failed to encrypt message! {:?}", e);
            return Err("Failed to encrypt message".to_string());
        }
    };

    let self_encrypted_text = match encrypt_message(self_pub_key.as_str(), message.as_str()).await {
        Ok(cipher) => cipher,
        Err(e) => {
            leptos::logging::log!("ERROR: Failed to encrypt message! {:?}", e);
            return Err("Failed to encrypt message".to_string());
        }
    };
    let payload = CreateMessage {
        sender_id: user_id,
        receiver_id: contact_id,
        receiver_content: encrypted_text,
        sender_content: self_encrypted_text,
    };
    let client = reqwest::Client::new();
    match client
        .post(base_url() + "/api/messages")
        .json(&payload)
        .send()
        .await
    {
        Ok(response) => {
            if response.status().is_success() {
                return Ok(());
            } else {
                leptos::logging::log!(
                    "ERROR: Message send failed with status: {}",
                    response.status()
                );
                return Err(format!("Message send failed: {}", response.status()));
            }
        }
        Err(e) => {
            leptos::logging::log!("ERROR: Failed to send message request: {:?}", e);
            return Err(format!("Network error: {}", e));
        }
    };
}

#[derive(Clone, Debug, Deserialize)]
struct InputContact {
    pub id: String,
    pub username: String,
    pub public_key: String,
    pub last_message: Option<InputMessage>,
}

async fn get_contacts(user_id: String) -> Vec<Contact> {
    let client = reqwest::Client::new();
    let input_contacts = match client
        .get(base_url() + "/api/contacts/" + user_id.as_str())
        .send()
        .await
        .ok()
    {
        Some(response) => {
            if response.status().is_success() {
                // Parse using InputContact instead of Contact
                match response.json::<Vec<InputContact>>().await.ok() {
                    Some(contacts) => contacts,
                    None => {
                        leptos::logging::log!("ERROR: Failed to parse contacts JSON");
                        vec![]
                    }
                }
            } else {
                vec![]
            }
        }
        None => vec![],
    };

    let my_priv_key = get_key_securely().await.unwrap();
    let mut final_contacts = Vec::new();

    // Loop through and decrypt the last message for the sidebar preview
    for ic in input_contacts {
        let last_message = if let Some(msg) = ic.last_message {
            let decrypted_content = if msg.sender_id == user_id {
                decrypt_message(&my_priv_key, &msg.sender_content)
                    .await
                    .unwrap_or_else(|_| "Error decrypting".to_string())
            } else {
                decrypt_message(&my_priv_key, &msg.receiver_content)
                    .await
                    .unwrap_or_else(|_| "Error decrypting".to_string())
            };

            Some(Message {
                id: msg.id,
                sender_id: msg.sender_id,
                receiver_id: msg.receiver_id,
                content: decrypted_content,
                timestamp: msg.timestamp,
            })
        } else {
            None
        };

        final_contacts.push(Contact {
            id: ic.id,
            username: ic.username,
            public_key: ic.public_key,
            last_message,
        });
    }

    final_contacts
}

#[component]
pub fn Chat() -> impl IntoView {
    // Contacts
    let my_user = get_current_user().get().expect("No user logged in");
    let my_user_id = my_user.id.clone();
    let message_user_id = StoredValue::new(my_user_id.clone());
    let my_pub_key = StoredValue::new(my_user.public_key.clone());
    let (messages, set_messages): (ReadSignal<Vec<Message>>, WriteSignal<Vec<Message>>) =
        signal(Vec::new());
    let (contacts, set_contacts): (ReadSignal<Vec<Contact>>, WriteSignal<Vec<Contact>>) =
        signal(Vec::new());
    let (selected_contact, set_selected_contact): (
        ReadSignal<Option<String>>,
        WriteSignal<Option<String>>,
    ) = signal(None);
    let (is_adding_contact, set_is_adding_contact) = signal(false);
    let (new_pseudo, set_new_pseudo) = signal(String::new());

    let active_contact = move || {
        selected_contact.get().and_then(|selected_id| {
            contacts.with(|list| list.iter().find(|c| c.id == selected_id).cloned())
        })
    };

    let my_user_id_clone = my_user_id.clone();
    spawn_local(async move {
        set_contacts.set(get_contacts(my_user_id_clone).await);
    });

    // Message writing
    let (input_text, set_input_text) = signal(String::new());
    let (error_message, set_error_message) = signal(Option::<String>::None);

    // Auto-clear error messages after 5 seconds
    Effect::new(move |_| {
        if error_message.get().is_some() {
            let set_error_message_clone = set_error_message.clone();
            spawn_local(async move {
                future::TimeoutFuture::new(5000).await;
                set_error_message_clone.set(None);
            });
        }
    });

    let handle_input_change = move |ev| {
        set_input_text.set(event_target_value(&ev));
    };

    // Web socket for messages
    let ws_active_contact = active_contact.clone();
    let ws_user_id = my_user_id.clone();
    let ws_set_messages = set_messages.clone();

    Effect::new(move |_| {
        let ws_user_id_clone = ws_user_id.clone();
        let window = web_sys::window().unwrap();
        let protocol = window.location().protocol().unwrap();
        let ws_protocol = if protocol == "https:" { "wss:" } else { "ws:" };
        let ws_url = format!("{}//{}/ws", ws_protocol, base_hostname());
        let ws = WebSocket::new(&ws_url).unwrap();

        let onmessage_callback = Closure::wrap(Box::new(move |_e: MessageEvent| {
            if let Some(contact) = ws_active_contact() {
                let fetch_user_id = ws_user_id_clone.clone();
                let fetch_contact_id = contact.id.clone();

                spawn_local(async move {
                    let fresh_messages =
                        get_messages_with_contact(fetch_user_id, fetch_contact_id).await;
                    ws_set_messages.set(fresh_messages);
                });
            }
        }) as Box<dyn FnMut(MessageEvent)>);

        ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));

        onmessage_callback.forget();
    });

    //Auto scroll messages :
    let bottom_ref = NodeRef::<leptos::html::Div>::new();

    Effect::new(move |_| {
        // Read the signal without cloning the whole array just to track it
        messages.with(|_| ());

        // request_animation_frame waits for the DOM to finish updating!
        request_animation_frame(move || {
            if let Some(el) = bottom_ref.get() {
                el.scroll_into_view();
            }
        });
    });

    view! {
        <div class="flex h-full w-full">

            // Contacts list
            <aside
                class="flex-col bg-base-100 border-r border-base-300 shadow-lg w-full md:w-72 md:min-w-[18rem] md:max-w-[22rem] md:flex"
                class:flex=move || selected_contact.get().is_none() && !is_adding_contact.get()
                class:hidden=move || selected_contact.get().is_some() || is_adding_contact.get()
            >
                <div class="p-4 border-b border-base-300 flex justify-between items-center">
                    <h1 class="text-xl font-bold tracking-tight">"Discussions"</h1>
                    <button class="btn btn-circle btn-ghost btn-sm"
                        on:click=move |_| {
                            set_is_adding_contact.set(true);
                            set_selected_contact.set(None);
                        }
                    >
                        <Icon icon=icondata::BiPlusRegular />
                    </button>
                </div>
                <div class="flex-1 overflow-y-auto p-2 space-y-1">

                    <For
                        each=move || contacts.get()
                        key=|contact| contact.id.clone()
                        children = move |contact: Contact| {
                            let click_id = contact.id.clone();
                            let class_id = contact.id.clone();

                            let click_user_id = my_user_id.clone();

                            view!{
                                <div
                                    on:click=move |_| {
                                        set_is_adding_contact.set(false);
                                        set_selected_contact.set(Some(click_id.clone()));

                                        let click_user_id_clone = click_user_id.clone();
                                        let click_id_clone = click_id.clone();

                                        spawn_local(async move {
                                            set_messages.set(get_messages_with_contact(click_user_id_clone, click_id_clone).await);
                                        });
                                    }
                                    class:bg-primary-focus=move || selected_contact.get() == Some(class_id.clone())
                                    class="cursor-pointer p-3 rounded-lg bg-primary text-primary-content shadow-md transition-all hover:bg-primary-focus group"
                                >
                                    <div class="flex justify-between items-start">
                                        <span class="font-semibold text-sm">{contact.username.clone()}</span>
                                        <span class="text-[10px] opacity-70">{if contact.last_message.is_some() {contact.last_message.clone().unwrap().timestamp.with_timezone(&Local).format("%H:%M").to_string()} else {"".to_string()}}</span>
                                    </div>
                                    <p class="text-xs mt-1 opacity-90 truncate">{if contact.last_message.is_some() {contact.last_message.clone().unwrap().content.clone()} else {"".to_string()}}</p>
                                </div>
                            }
                        }
                        />
                </div>
            </aside>

            // Selected contact messaging zone
            <main
                class="flex-1 flex-col bg-base-100 relative w-full md:flex"
                class:flex=move || selected_contact.get().is_some() || is_adding_contact.get()
                class:hidden=move || selected_contact.get().is_none() && !is_adding_contact.get()
            >
                <Show
                    when=move || is_adding_contact.get()
                    fallback=move || view! {
                        <Show
                            when=move || selected_contact.get().is_some()
                            fallback=|| view! {
                                <div class="flex-1 flex flex-col items-center justify-center opacity-50 select-none">
                                    <Icon icon=icondata::BiMessageSquareDetailRegular attr:class="size-16 mb-4" />
                                    <p class="text-lg font-medium">"Select a discussion to start chatting"</p>
                                </div>
                            }
                        >
                <Show
                    when=move || selected_contact.get().is_some()
                    fallback=|| view! {
                                <div class="flex-1 flex flex-col items-center justify-center opacity-50 select-none">
                                    <Icon icon=icondata::BiMessageSquareDetailRegular attr:class="size-16 mb-4" />
                                    <p class="text-lg font-medium">"Select a discussion to start chatting"</p>
                                </div>
                            }
                >
                    // Contact info
                    <header class="border-b border-base-300 p-4 bg-base-100 flex justify-between items-center shadow-sm z-10 shrink-0">
                        <div class="flex items-center gap-2">
                            // MOBILE BACK BUTTON
                            <button
                                class="btn btn-ghost btn-circle btn-sm md:hidden"
                                on:click=move |_| {
                                    set_selected_contact.set(None);
                                    set_is_adding_contact.set(false);
                                }
                            >
                                <Icon icon=icondata::BiArrowBackRegular attr:class="size-5".to_string() />
                            </button>

                            <div>
                                <h2 class="font-bold text-base">
                                    {move || active_contact().map(|c| c.username).unwrap_or_default()}
                                </h2>
                            </div>
                        </div>
                    </header>

                    // Messages zone
                    <div class="flex-1 overflow-y-auto p-4 space-y-4 chat-scroll bg-base-100">
                        <For
                            each=move || messages.get()

                            key=|message| message.id.clone()

                            children=move |message: Message| {
                                let is_mine = message.sender_id == message_user_id.get_value();

                                let time = message.timestamp.with_timezone(&Local).format("%H:%M").to_string();
                                let content = message.content.clone();

                                let chat_class = if is_mine { "chat chat-end" } else { "chat chat-start" };
                                let header_class = if is_mine { "chat-header text-xs opacity-50 mb-1 text-right" } else { "chat-header text-xs opacity-50 mb-1" };
                                let bubble_class = if is_mine { "chat-bubble chat-bubble-primary text-white" } else { "chat-bubble chat-bubble-accent" };
                                let author = if is_mine { "You" } else { "Contact" };

                                view! {
                                    <div class=chat_class>
                                        <div class=header_class>
                                            {author} <time class="text-xs opacity-50 ml-2">{time}</time>
                                        </div>
                                        <div class=bubble_class>{content}</div>

                                        {if is_mine {
                                            Some(view! { <div class="chat-footer opacity-50 text-xs text-right">"sent"</div> })
                                        } else {
                                            None
                                        }}
                                    </div>
                                }
                            }
                        />
                        <div node_ref=bottom_ref></div>
                    </div>

                    // Writing zone
                    <div class="p-4 border-t border-base-300 bg-base-100 shrink-0">
                        <div class="flex gap-2 items-end">

                            <textarea
                                id="message-input"
                                class="textarea textarea-bordered flex-1 max-h-32 focus:outline-none focus:ring-2 focus:ring-primary/50 resize-none"
                                rows="1"
                                placeholder="Type a message..."
                                on:input=handle_input_change
                                prop:value=move || input_text.get()
                            ></textarea>

                            <button
                                class="btn btn-primary btn-square"
                                on:click=move |_| {
                                        if let Some(contact) = active_contact() {
                                            // Get the values from StoredValue cleanly!
                                            let sender_id = message_user_id.get_value();
                                            let sender_public_key = my_pub_key.get_value();
                                            let current_text = input_text.get();

                                            spawn_local(async move {
                                                let _ = send_message_to_contact(
                                                    sender_id.clone(),
                                                    contact.id.clone(),
                                                    current_text.clone(),
                                                    contact.public_key.clone(),
                                                    sender_public_key, // Passed directly
                                                ).await;
                                                set_messages.set(get_messages_with_contact(
                                                    sender_id,
                                                    contact.id,
                                                ).await);
                                            });

                                            set_input_text.set(String::new())
                                        }
                                    }
                            >
                                <Icon icon=icondata::BiSendSolid />
                            </button>

                        </div>
                    </div>
                </Show>
                        </Show>
                                }
                            >
                                <div class="flex-1 flex flex-col p-8 items-center justify-center">
                                    <div class="w-full max-w-md bg-base-200 p-6 rounded-xl shadow-md flex flex-col gap-4">
                                        <button
                                            class="btn btn-ghost btn-circle btn-sm md:hidden"
                                            on:click=move |_| set_is_adding_contact.set(false)
                                        >
                                            <Icon icon=icondata::BiArrowBackRegular attr:class="size-5".to_string() />
                                        </button>
                                        <h2 class="text-xl font-bold">"Start a new discussion"</h2>

                                        <div class="form-control">
                                            <label class="label"><span class="label-text">"Contact Pseudo"</span></label>
                                            <input
                                                type="text"
                                                placeholder="Who do you want to message?"
                                                class="input input-bordered w-full"
                                                on:input=move |ev| set_new_pseudo.set(event_target_value(&ev))
                                                prop:value=move || new_pseudo.get()
                                            />
                                        </div>

                                        <div class="form-control">
                                            <label class="label"><span class="label-text">"Message"</span></label>
                                            <textarea
                                                class="textarea textarea-bordered h-24 resize-none"
                                                placeholder="Type your first message..."
                                                on:input=move |ev| set_input_text.set(event_target_value(&ev))
                                                prop:value=move || input_text.get()
                                            ></textarea>
                                        </div>

                                        <Show when=move || error_message.get().is_some()>
                                            <div class="alert alert-error mt-2">
                                                <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>
                                                <span>{move || error_message.get().unwrap_or_default()}</span>
                                            </div>
                                        </Show>

                                        <button
                                            class="btn btn-primary mt-2"
                                            prop:disabled=move || new_pseudo.get().trim().is_empty() || input_text.get().trim().is_empty()
                                            on:click=move |_| {

                                                let sender_id = message_user_id.get_value();
                                                let sender_public_key = my_pub_key.get_value();
                                                let target_pseudo = new_pseudo.get();
                                                let message_content = input_text.get();

                                                #[derive(Debug, Clone, Deserialize)]
                                                struct FirstContact {
                                                    pub id: String,
                                                    pub username: String,
                                                    pub public_key: String,
                                                }

                                                spawn_local(async move {
                                                    let res = reqwest::Client::new()
                                                        .get(format!("{}/api/users/pseudo/{}", base_url(), target_pseudo))
                                                        .send().await;

                                                    match res {
                                                        Ok(response) => {
                                                            if response.status().is_success() {
                                                                match response.json::<FirstContact>().await {
                                                                    Ok(contact) => {
                                                                        if let Err(e) = send_message_to_contact(
                                                                            sender_id,
                                                                            contact.id,
                                                                            message_content,
                                                                            contact.public_key,
                                                                            sender_public_key.clone()
                                                                        ).await {
                                                                            set_error_message.set(Some(format!("Failed to send message: {}", e)));
                                                                        }

                                                                        // Clear the input fields on success
                                                                        set_new_pseudo.set(String::new());
                                                                        set_input_text.set(String::new());
                                                                    }
                                                                    Err(e) => {
                                                                        set_error_message.set(Some(format!("Failed to parse user data: {}", e)));
                                                                    }
                                                                }
                                                            } else {
                                                                set_error_message.set(Some(format!("User '{}' not found.", target_pseudo)));
                                                            }
                                                        }
                                                        Err(e) => {
                                                            set_error_message.set(Some(format!("Network error: {}", e)));
                                                        }
                                                    }
                                                });
                                            }
                                        >
                                            "Send Message"
                                            <Icon icon=icondata::BiSendSolid attr:class="ml-2" />
                                        </button>
                                    </div>
                                </div>
                            </Show>
            </main>
        </div>
    }
}
