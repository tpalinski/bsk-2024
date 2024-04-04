use leptos::*;

#[component]
pub fn HomePage() -> impl IntoView {

    view! {
        <div class="bg-gray-600 h-screen w-screen flex flex-col gap-4 items-center">
            <h1 class="text-violet-300 text-6xl">"Professional RSA encryption app"</h1>
            <p class="bold text-xl text-center"> Choose one of the options below </p>
            <a href="/signature" class="p-4 bg-violet-300 rounded-xl"> Sign a document utilizing electronic signature </a>
        </div>
    }
}
