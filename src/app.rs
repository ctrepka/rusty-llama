use crate::api::converse;
use crate::model::Message;
use leptos::html::Input;
use leptos::html::Section;
use leptos::*;
use leptos_meta::*;

use crate::model::Conversation;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let (conversation, set_conversation) = create_signal(Conversation::new());
    let send = create_action(move |new_message: &String| {
        let user_message = Message {
            text: new_message.clone(),
            user: true,
        };
        set_conversation.update(move |c| c.messages.push(user_message));

        converse(conversation.get())
    });

    create_effect(move |_| {
        if let Some(_) = send.input().get() {
            let model_message = Message {
                text: String::from("..."),
                user: false,
            };

            set_conversation.update(move |c| {
                c.messages.push(model_message);
            });
        }
    });

    create_effect(move |_| {
        if let Some(Ok(response)) = send.value().get() {
            set_conversation.update(move |c| {
                c.messages.last_mut().unwrap().text = response;
            });
        }
    });

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>
        <main id="main-container">
            <ChatArea conversation />
            <TypeArea send />
        </main>

    }
}

#[component]
pub fn TypeArea(send: Action<String, Result<String, ServerFnError>>) -> impl IntoView {
    let input_ref = create_node_ref::<Input>();

    view! {
        <section id="prompt-area">
            <form on:submit=move |ev| {
                ev.prevent_default();
                let input = input_ref.get().expect("input to exist");
                send.dispatch(input.value());
                input.set_value("");
            }>
                <input type="text" node_ref=input_ref />
                <button>Submit</button>
            </form>
        </section>
    }
}

#[component]
pub fn ChatArea(conversation: ReadSignal<Conversation>) -> impl IntoView {
    let chat_area_ref = create_node_ref::<Section>();

    create_effect(move |_| {
        conversation.get();
        if let Some(section) = chat_area_ref.get() {
            section.set_scroll_top(section.scroll_height());
        }
    });

    view! {
        <section id="chat-area" node_ref=chat_area_ref >
            {
                move || conversation.get().messages.iter().map(move |message| {
                    let style_str = if message.user {String::from("user-chat")} else {String::from("ai-chat")};
                    view! {
                        <div
                            class={style_str}
                        >{message.text.clone()}</div>
                    }
                }).collect::<Vec<_>>()
            }
        </section>
    }
}
