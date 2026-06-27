use chrono::Local;
use leptos::{prelude::*, task::spawn_local};
use leptos_icons::Icon;
use serde::Serialize;

use crate::{
    data_models::{contact::Contact, message::Message},
    services::web::base_url,
    store::auth::get_current_user,
};

fn order_messages_by_timestamp(messages: &mut Vec<Message>) {
    messages.sort_by_key(|message| message.timestamp);
}

async fn get_messages_with_contact(user_id: String, contact_id: String) -> Vec<Message> {
    let client = reqwest::Client::new();
    let mut messages = match client
        .get(
            base_url() + "/api/chats/" + user_id.as_str() + "/" + contact_id.as_str() + "/messages",
        )
        .send()
        .await
        .ok()
    {
        Some(response) => {
            if response.status().is_success() {
                match response.json::<Vec<Message>>().await.ok() {
                    Some(messages) => messages,
                    None => vec![],
                }
            } else {
                vec![]
            }
        }
        None => vec![],
    };

    order_messages_by_timestamp(&mut messages);
    messages
}

#[derive(Debug, Serialize)]
pub struct CreateMessage {
    pub sender_id: String,
    pub receiver_id: String,
    pub content: String,
}
async fn send_message_to_contact(
    user_id: String,
    contact_id: String,
    message: String,
) -> Result<(), String> {
    let payload = CreateMessage {
        sender_id: user_id,
        receiver_id: contact_id,
        content: message,
    };
    let client = reqwest::Client::new();
    match client
        .post(base_url() + "/api/messages")
        .json(&payload)
        .send()
        .await
        .ok()
    {
        Some(response) => {
            if response.status().is_success() {
                return Ok(());
            } else {
                return Err("Something wrong happened".to_string());
            }
        }
        None => return Err("Something wrong happened".to_string()),
    };
}

#[derive(Debug, Serialize)]
pub struct CreateFirstMessage {
    pub sender_id: String,
    pub receiver_pseudo: String,
    pub content: String,
}
async fn send_first_message_to_contact(
    user_id: String,
    pseudo: String,
    message: String,
) -> Result<(), String> {
    let payload = CreateFirstMessage {
        sender_id: user_id,
        receiver_pseudo: pseudo,
        content: message,
    };
    let client = reqwest::Client::new();
    match client
        .post(base_url() + "/api/messages")
        .json(&payload)
        .send()
        .await
        .ok()
    {
        Some(response) => {
            if response.status().is_success() {
                return Ok(());
            } else {
                return Err("Something wrong happened".to_string());
            }
        }
        None => return Err("Something wrong happened".to_string()),
    };
}

async fn get_contacts(user_id: String) -> Vec<Contact> {
    let client = reqwest::Client::new();
    let contacts = match client
        .get(base_url() + "/api/contacts/" + user_id.as_str())
        .send()
        .await
        .ok()
    {
        Some(response) => {
            if response.status().is_success() {
                match response.json::<Vec<Contact>>().await.ok() {
                    Some(contacts) => contacts,
                    None => vec![],
                }
            } else {
                vec![]
            }
        }
        None => vec![],
    };
    contacts
}

#[component]
pub fn Chat() -> impl IntoView {
    // Contacts
    let my_user_id = get_current_user().get().expect("No user logged in").id;
    let message_user_id = StoredValue::new(my_user_id.clone());
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

    let handle_input_change = move |ev| {
        set_input_text.set(event_target_value(&ev));
    };

    view! {
        <div class="flex h-full w-full">

            // Contacts list
            <aside class="w-72 min-w-[18rem] max-w-[22rem] flex flex-col bg-base-100 border-r border-base-300 shadow-lg">
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
            <main class="flex-1 flex flex-col bg-base-100 relative">
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
                        <div class="flex items-center gap-3">
                            <div>
                                <h2 class="font-bold text-base">
                                    {move || active_contact().map(|c| c.username).unwrap_or_default()}
                                </h2>
                            </div>
                        </div>
                    </header>

                    // Messages zone
                    <div class="flex-1 overflow-y-auto p-4 space-y-4 chat-scroll bg-base-100">
                        // Example day separator
                        // <div class="text-center my-4">
                        //     <span class="badge badge-ghost badge-sm">"Today"</span>
                        // </div>

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
                                            let sender_id = message_user_id.get_value();
                                            let current_text = input_text.get();

                                            spawn_local(async move {
                                                let _ = send_message_to_contact(
                                                    sender_id.clone(),
                                                    contact.id.clone(),
                                                    current_text.clone(),
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
                                                on:input=handle_input_change
                                                prop:value=move || input_text.get()
                                            ></textarea>
                                        </div>

                                        <button
                                            class="btn btn-primary mt-2"
                                            prop:disabled=move || new_pseudo.get().trim().is_empty() || input_text.get().trim().is_empty()
                                            on:click=move |_| {
                                                let sender_id = message_user_id.get_value();
                                                let target_pseudo = new_pseudo.get();
                                                let msg_text = input_text.get();

                                                let sender_id_clone = sender_id.clone();
                                                let my_user_id_clone = sender_id.clone();

                                                set_new_pseudo.set(String::new());
                                                set_input_text.set(String::new());
                                                set_is_adding_contact.set(false);

                                                spawn_local(async move {
                                                    let _ = send_first_message_to_contact(
                                                        sender_id_clone,
                                                        target_pseudo,
                                                        msg_text
                                                    ).await;

                                                    set_contacts.set(get_contacts(my_user_id_clone).await);
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
