use leptos::{component, IntoView, view};
use leptos_router::A;

#[component]
pub fn NotFoundPage() -> impl IntoView {
    view! {
        <div class="m-auto text-center p-20 bg-gradient-to-r from-indigo-500 via-purple-500 to-pink-500 rounded-lg shadow-xl">
            <h1 class="text-9xl font-bold text-white mb-8">"404"</h1>
            <p class="text-2xl text-white mb-8">"抱歉，我们找不到您请求的页面。"</p>
            <A href="/" class="px-8 py-3 text-lg font-semibold rounded-full bg-white text-indigo-500 hover:bg-gray-200 transition-colors duration-300">"返回首页"</A>
        </div>
    }
}
