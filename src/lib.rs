use leptos::{component, create_rw_signal, create_signal, IntoView, provide_context, ReadSignal, RwSignal, SignalGet, SignalUpdate, use_context, view, WriteSignal};
use leptos_meta::{Link, Meta, provide_meta_context, Stylesheet, Title};
use leptos_router::{A, Route, Router, Routes};

use crate::css::CssClass::{AboutContainer, AboutH1, AboutP, Container, HomeButton, HomeContainer, HomeH2, HomeP, MyFooter, Nav, NavLink};
use crate::css::Theme;
use crate::pages::not_found::NotFoundPage;
use crate::state::GlobalState;

mod components;
mod pages;
mod css;
mod state;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    let state = create_rw_signal(GlobalState::default());
    provide_context(state);

    view! {
        <Stylesheet id="leptos" href="/style/output.css"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        // sets the document title
        <Title text="Welcome to Leptos CSR"/>
        // injects metadata in the <head> of the page
        <Meta charset="UTF-8"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>

        <Router>
             <div class={move || Container.get_css(state.get().theme)}>
                <NavBar/>
                <main class="flex-grow">
                    <Routes>
                        <Route path="/" view=Home/>
                        <Route path="/about" view=About/>
                        <Route path="/*any" view=NotFoundPage/>
                    </Routes>
                </main>
                <Footer/>
            </div>
        </Router>
    }
}


#[component]
fn NavBar() -> impl IntoView {
    let state = use_context::<RwSignal<GlobalState>>().unwrap_or_default();
    view! {
        <nav class={move || Nav.get_css(state.get().theme)}>
            <ul class="flex justify-center space-x-10 py-4">
                <li>
                    <A href="/" class={move || NavLink.get_css(state.get().theme)}>
                        {"Home"}
                    </A>
                </li>
                <li>
                    <A href="/about" class={move || NavLink.get_css(state.get().theme)}>
                        {"About"}
                    </A>
                </li>
            </ul>
        </nav>
    }
}

#[component]
fn Footer() -> impl IntoView {
    let state = use_context::<RwSignal<GlobalState>>().unwrap_or_default();
    view! {
        <footer class={move || MyFooter.get_css(state.get().theme)}>
            <p class="font-bold">{"© 2024 Company Name. All rights reserved."}</p>
        </footer>
    }
}

#[component]
fn About() -> impl IntoView {
    let state = use_context::<RwSignal<GlobalState>>().unwrap_or_default();
    view! {
        <div class={move || AboutContainer.get_css(state.get().theme)}>
            <h1 class={move || AboutH1.get_css(state.get().theme)}>{"About Us"}</h1>
            <p class={move || AboutP.get_css(state.get().theme)}>{"Here you can learn more about us."}</p>
        </div>
    }
}


#[component]
pub fn Home() -> impl IntoView {
    let state = use_context::<RwSignal<GlobalState>>().unwrap_or_default();
    let (count, set_count) = create_signal(1);
    view! {
        <div class={move || HomeContainer.get_css(state.get().theme)}>
            <h2 class={move || HomeH2.get_css(state.get().theme)}>"Welcome to Leptos with Tailwind"</h2>
            <p class={move || HomeP.get_css(state.get().theme)}>"Tailwind will scan your Rust files for Tailwind class names and compile them into a CSS file."</p>
            <button
                class={move || HomeButton.get_css(state.get().theme)}
                on:click=move |_| home_button_on_click(count,set_count,state)
            >
                "Something's here | "
                {move || if count.get() == 0 {
                    "Click me!".to_string()
                } else {
                    count.get().to_string()
                }}
                " | Some more text"
            </button>
        </div>
    }
}

/// 增加计数器 + 切换主题
fn home_button_on_click(count: ReadSignal<i32>, set_count: WriteSignal<i32>, state: RwSignal<GlobalState>) {
    set_count.update(|count| *count += 1);
    let current_count = count.get();
    if current_count % 2 == 0 {
        state.update(|state| state.theme = Theme::Twitter);
    } else {
        state.update(|state| state.theme = Theme::Ig);
    }
}
