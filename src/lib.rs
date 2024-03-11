use leptos::{component, view, IntoView, create_signal, SignalUpdate, SignalGet};
use leptos_meta::{provide_meta_context, Link, Meta, Stylesheet, Title};
use leptos_router::{Route, Router, Routes, A};
use crate::pages::not_found::NotFound;


mod components;
mod pages;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    view! {
        <Stylesheet id="leptos" href="/style/output.css"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        // sets the document title
        <Title text="Welcome to Leptos CSR"/>
        // injects metadata in the <head> of the page
        <Meta charset="UTF-8"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>

        <Router>
        <div class="flex justify-center items-center">
            <nav class="bg-gray-800 p-4">
                <A href="/" class="text-white">"Home"</A>
                <span class="text-white">" | "</span>
                <A href="/about" class="text-white">"About"</A>
            </nav>
        </div>
            <Routes>
                <Route path="/*" view=  move || view! { <NotFound/> }/>
                <Route path="/" view=  move || view! { <Home/> }/>
                <Route path="/about" view=  move || view! { <About/> }/>
            </Routes>
        </Router>
    }
}



#[component]
pub fn Home() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
        <div class="my-0 mx-auto max-w-3xl text-center">
            <h2 class="p-6 text-4xl">"Welcome to Leptos with Tailwind"</h2>
            <p class="px-10 pb-10 text-left">"Tailwind will scan your Rust files for Tailwind class names and compile them into a CSS file."</p>
            <button
                class="bg-amber-600 hover:bg-sky-700 px-5 py-3 text-white rounded-lg"
                on:click=move |_| set_count.update(|count| *count += 1)
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


#[component]
pub fn About() -> impl IntoView {
    view! {
        <div class="my-0 mx-auto max-w-3xl text-center">
            <h2 class="p-6 text-4xl">"About"</h2>
            <p class="px-10 pb-10 text-left">"This is a simple example of a Leptos app with Tailwind CSS.=========="</p>
        </div>
    }
}