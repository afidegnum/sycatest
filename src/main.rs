use gloo_timers::future::TimeoutFuture;
use rand::Rng;
use std::error::Error as SysError;
use std::fmt::Display;
use sycamore::prelude::*;
use sycamore::suspense::{use_transition, Suspense};
#[derive(Debug, Clone, Copy)]
enum Block {
    One,
    Two,
    Three,
}

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Block {
    fn content<G: Html>(self, cx: Scope) -> Result<View<G>, Box<dyn SysError>> {
        match self {
            Block::One => Ok(view! {cx, div{"content one"}}),
            Block::Two => Ok(view! {cx, div{"content Two"}}),
            Block::Three => Ok(view! {cx, div{"content Three"}}),
        }
    }
}

#[component]
async fn Child<G: Html>(cx: Scope<'_>, block: Block) -> View<G> {
    let delay_ms = rand::thread_rng().gen_range(200..500);
    TimeoutFuture::new(delay_ms).await;

    view! { cx,
        div {
            p { "Content loaded after " (delay_ms) "ms" }

            p { (tab.content(cx)) }
        }
    }
}

#[component]
fn App<G: Html>(cx: Scope) -> View<G> {
    let tab = create_signal(cx, Block::One);
    let transition = use_transition(cx);
    let update = move |x| transition.start(move || block.set(x), || ());

    view! { cx,
        div {
            p { "Suspense + Transitions" }
            p { "Transition state: " (transition.is_pending().then_some("pending").unwrap_or("done")) }
            button(on:click=move |_| update(Block::One)) { "One" }
            button(on:click=move |_| update(Block::Two)) { "Two" }
            button(on:click=move |_| update(Block::Three)) { "Three" }
            Suspense(fallback=view! { cx, p { "Loading..." } }) {
                ({
                    let tab = *tab.get();
                    view! { cx, Child(tab) }
                })
            }
        }
    }
}

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap();

    sycamore::render(|cx| view! { cx, App {} });
}
