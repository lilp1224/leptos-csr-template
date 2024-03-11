use leptos::{logging, mount_to_body, view};
use {{crate_name}}::App;
// use project_name::App;

fn main() {

    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    logging::log!("csr mode - mounting to body");

    mount_to_body(|| {
        view! {<App />}
    });
}
