use leptos::{leptos_dom::logging::console_log, *};

#[component]
pub fn SignaturePage() -> impl IntoView {
    // Creates a reactive value to update the button

    let input_ref: NodeRef<html::Input> = create_node_ref();

    view! {
        <div class="bg-gray-600 h-screen w-screen flex flex-col gap-4 items-center">
            <input type="file" 
                node_ref=input_ref
            />
            <button on:click=move |_| {
                console_log(&input_ref().expect("should never happen").value())
            }>
            Sign file
            </button>
        </div>
    }
}
