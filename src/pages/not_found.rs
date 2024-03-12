use leptos::{component, IntoView, RwSignal, SignalGet, use_context, view};
use leptos_router::A;
use crate::css::CssClass::{NotFoundPageA, NotFoundPageContainer, NotFoundPageH1, NotFoundPageP};
use crate::state::GlobalState;

#[component]
pub fn NotFoundPage() -> impl IntoView {
    let state = use_context::<RwSignal<GlobalState>>().unwrap_or_default();
    view! {
        <div class={move || NotFoundPageContainer.get_css(state.get().theme)}>
            <h1 class={move || NotFoundPageH1.get_css(state.get().theme)}>"404"</h1>
            <p class={move || NotFoundPageP.get_css(state.get().theme)}>"抱歉，我们找不到您请求的页面。"</p>
            <A href="/" class={move || NotFoundPageA.get_css(state.get().theme)}>"返回首页"</A>
        </div>
    }
}
