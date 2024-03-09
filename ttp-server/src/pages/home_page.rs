use leptos::*;

#[component]
pub fn HomePage() -> impl IntoView {

    view! {
        <div class="bg-gray-600 h-screen w-screen flex flex-col gap-4 items-center">
            <h1 class="text-violet-300 text-6xl"> "BSK2024 - TTP server (real)" </h1>
            <a href="/generate" class="pd-6 rounded-xl bg-grey-800"> "Generate keys" </a>
        </div>
    }
}
