use leptos::{component, view, IntoView, create_signal, SignalUpdate, SignalGet};
use leptos_meta::{provide_meta_context, Link, Meta, Stylesheet, Title};
use leptos_router::{Route, Router, Routes, A};
use crate::pages::not_found::NotFoundPage;


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
             <div class="flex flex-col min-h-screen bg-gradient-to-r from-indigo-100 via-purple-100 to-pink-100">
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
    view! {
        <nav class="bg-gradient-to-r from-indigo-500 via-purple-500 to-pink-500 text-white">
            <ul class="flex justify-center space-x-10 py-4">
                <li>
                    <A href="/" class="nav-link font-bold text-lg hover:bg-indigo-700 p-3 rounded-lg transition-all duration-300">
                        {"Home"}
                    </A>
                </li>
                <li>
                    <A href="/about" class="nav-link font-bold text-lg hover:bg-indigo-700 p-3 rounded-lg transition-all duration-300">
                        {"About"}
                    </A>
                </li>
            </ul>
        </nav>
    }
}

#[component]
fn Footer() -> impl IntoView {
    view! {
        <footer class="mt-auto bg-gradient-to-r from-indigo-500 via-purple-500 to-pink-500 text-white text-center py-4">
            <p class="font-bold">{"© 2024 Company Name. All rights reserved."}</p>
        </footer>
    }
}

#[component]
fn About() -> impl IntoView {
    view! {
        <div class="text-center p-10">
            <h1 class="text-3xl font-bold my-4">{"About Us"}</h1>
            <p class="mt-2 text-lg">{"Here you can learn more about us."}</p>
        </div>
    }
}


#[component]
pub fn Home() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
        <div class="my-0 mx-auto max-w-3xl text-center">
            <h2 class="p-6 text-4xl font-bold">"Welcome to Leptos with Tailwind"</h2>
            <p class="px-10 pb-10 text-left text-lg">"Tailwind will scan your Rust files for Tailwind class names and compile them into a CSS file."</p>
            <button
                class="bg-gradient-to-r from-pink-500 to-yellow-500 hover:from-pink-600 hover:to-yellow-600 active:to-pink-700 active:from-yellow-500 px-6 py-3 text-white rounded-full shadow-md transform active:scale-95 transition-all duration-300"
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

