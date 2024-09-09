use sycamore::prelude::*;

#[component]
fn App() -> View {
    let counter = create_signal(0);
    let increment = move |_| counter.set(counter.get() + 1);
    view! {
        div {
            h1 { "Hello Sycamore!" }
        }

        div {
            button(on:click=increment, r#type="button") { "+1" }
            p { "Value: " (counter.get()) }
        }

        div {
            p { "Try modifying src/main.rs and see your changes updated automatically." }
        }
    }
}

fn main() {
    console_error_panic_hook::set_once();
    sycamore::render(App);
    console_log!("App mounted!");
}
