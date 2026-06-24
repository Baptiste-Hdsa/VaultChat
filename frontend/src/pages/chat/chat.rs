use chrono::Local;
use leptos::{logging::log, prelude::*};
use leptos_icons::Icon;

use crate::{
    data_models::{contact::Contact, message::Message},
    store::auth::get_current_user,
};

fn order_messages_by_timestamp(messages: &mut Vec<Message>) {
    messages.sort_by_key(|message| message.timestamp);
}

fn get_messages_with_contact(user_id: String, contact_id: String) -> Vec<Message> {
    log!(
        "Calling backend to get messages of : {} and {}",
        user_id,
        contact_id
    );
    let mut messages = vec![];
    messages.push(Message {
        id: "1".to_string(),
        sender_id: contact_id.clone(),
        receiver_id: user_id.clone(),
        content: "Test received".to_string(),
        timestamp: Local::now().to_utc(),
    });
    messages.push(Message {
        id: "2".to_string(),
        sender_id: user_id.clone(),
        receiver_id: contact_id.clone(),
        content: "Test sent".to_string(),
        timestamp: Local::now().to_utc(),
    });

    order_messages_by_timestamp(&mut messages);
    messages
}

fn send_message_to_contact(
    user_id: String,
    contact_id: String,
    message: String,
) -> Result<(), String> {
    log!(
        "Calling backend to send {} from {} to {}",
        message,
        user_id,
        contact_id
    );

    Ok(())
}

fn get_contacts(user_id: String) -> Vec<Contact> {
    log!("Calling backend to get contacts of : {}", user_id);

    vec![Contact {
        pseudo: "Contact".to_string(),
        id: "contact".to_string(),
        last_message: Message {
            id: "3".to_string(),
            sender_id: "contact".to_string(),
            receiver_id: user_id.clone(),
            content: "last_message".to_string(),
            timestamp: Local::now().to_utc(),
        },
    }]
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

    let active_contact = move || {
        selected_contact.get().and_then(|selected_id| {
            contacts.with(|list| list.iter().find(|c| c.id == selected_id).cloned())
        })
    };

    set_contacts.set(get_contacts(my_user_id.clone()));

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
                    <button class="btn btn-circle btn-ghost btn-sm">
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
                                // Example contact template
                                <div
                                    on:click=move |_| {
                                        set_selected_contact.set(Some(click_id.clone()));
                                        set_messages.set(get_messages_with_contact(click_user_id.clone(), click_id.clone()));
                                    }
                                    class:bg-primary-focus=move || selected_contact.get() == Some(class_id.clone())
                                    class="cursor-pointer p-3 rounded-lg bg-primary text-primary-content shadow-md transition-all hover:bg-primary-focus group"
                                >
                                    <div class="flex justify-between items-start">
                                        <span class="font-semibold text-sm">{contact.pseudo.clone()}</span>
                                        <span class="text-[10px] opacity-70">{contact.last_message.timestamp.with_timezone(&Local).format("%H:%M").to_string()}</span>
                                    </div>
                                    <p class="text-xs mt-1 opacity-90 truncate">{contact.last_message.content.clone()}</p>
                                </div>
                            }
                        }
                        />
                </div>
            </aside>

            // Selected contact messaging zone
            <main class="flex-1 flex flex-col bg-base-100 relative">
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
                                    {move || active_contact().map(|c| c.pseudo).unwrap_or_default()}
                                </h2>
                            </div>
                        </div>
                    </header>

                    // Messages zone
                    <div class="flex-1 overflow-y-auto p-4 space-y-4 chat-scroll bg-base-100">
                        // Example day separator
                        <div class="text-center my-4">
                            <span class="badge badge-ghost badge-sm">"Today"</span>
                        </div>

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

                                            let _ = send_message_to_contact(
                                                sender_id.clone(),
                                                contact.id.clone(),
                                                current_text.clone(),
                                            );

                                            // set_messages.set(get_messages_with_contact(
                                            //     sender_id,
                                            //     contact.id,
                                            // ));

                                            // Temp for testing
                                            set_messages.update(|msgs| {
                                                    msgs.push(Message {
                                                        id: "caca".to_string(),
                                                        sender_id,
                                                        receiver_id: contact.id,
                                                        content: current_text.clone(),
                                                        timestamp: Local::now().to_utc(),
                                                    });
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
            </main>
        </div>
    }
}
