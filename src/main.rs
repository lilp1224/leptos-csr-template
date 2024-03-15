use leptos::{mount_to_body, view};
#[cfg(feature = "dev-tracing")]
// use tracing::info;
use {{crate_name}}::App;
use project_name::App;

fn main() {

    #[cfg(feature = "dev-tracing")]
    {
        console_error_panic_hook::set_once();
        tracing_wasm::set_as_global_default();
        info!("csr mode - mounting to body!!");
    }

    mount_to_body(|| {
        view! {<App />}
    });
}
