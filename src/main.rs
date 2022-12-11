use sycamore::prelude::*;

#[component]
fn App<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        div {
            p { "Hello, world!" }
        }
    }
}

fn main() {
    console_error_panic_hook::set_once();
    console_log::init().unwrap();

    sycamore::render(App);
}
