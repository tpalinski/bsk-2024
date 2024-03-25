use leptos::*;

#[component]
pub fn HomePage() -> impl IntoView {

    view! {
        <div class="bg-gray-600 h-screen w-screen flex flex-col gap-4 items-center">
            <h1 class="text-violet-300 text-6xl"> "BSK2024 - TTP server (real)" </h1>
            <p class="bold text-xl text-center"> "This site plays the role of a Trusted Third Party (TTP). It provides the users with encryption keys, stores them and automatically provides them to users during encryption process." </p>
            <a href="/register" class="p-4 bg-violet-300 rounded-xl"> "Register an account" </a>
        </div>
    }
}
