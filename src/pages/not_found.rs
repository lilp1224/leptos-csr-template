use leptos::{component, IntoView, view};
use leptos_router::A;

/// 404 Not Found Page
#[component]
pub fn NotFoundPage() -> impl IntoView {
    view! {
        <div class="flex h-screen">
            <div class="m-auto text-center">
                <h1 class="text-6xl font-bold mb-8">"404"</h1>
                <p class="text-xl mb-8">"抱歉，我们找不到您请求的页面。"</p>
                <A href="/" class="px-6 py-2 text-sm font-semibold rounded-full bg-blue-500 text-white hover:bg-blue-700">"返回首页"</A>
            </div>
        </div>
    }
}
