use leptos::*;

use crate::app::GlobalState;

#[server(DecryptFile, "/api")]
async fn decrypt_file(path: String) -> Result<String, ServerFnError> {
    #[cfg(feature="ssr")]
    {
        todo!()
    }
}

#[component]
pub fn DecryptPage() -> impl IntoView {
    let input_ref: NodeRef<html::Input> = create_node_ref();
    let decrypt_action = create_server_action::<DecryptFile>();

    let global_state = expect_context::<RwSignal<GlobalState>>();
    let (token, set_token) = create_slice(global_state, |state| state.token.clone(), |state, token| state.token = token);

    if token.get().is_empty() {
        let navigate = leptos_router::use_navigate();
        let mut options = leptos_router::NavigateOptions::default();
        options.replace = true;
        navigate("/login?redirect=decrypt", options);
    }

    let decrypt_error = Signal::derive(move || {
        let val = decrypt_action.value().get();
        if val.is_some() {
            match val.unwrap() {
                Ok(_) => None,
                Err(e) => Some(e.to_string())
            }
        } else {
            None
        }
    });

    let decrypt_data = Signal::derive(move || {
        let action_data = decrypt_action.value().get();
        if action_data.is_some() {
            match action_data.unwrap() {
                Ok(res) => Some(res),
                Err(_) => None
            }
        } else {
            None
        }
    });


    view! {
        <div class="bg-gray-600 h-screen w-screen flex flex-col gap-4 items-center">
            <h1 class="text-violet-300 text-6xl">"Professional RSA encryption app"</h1>
            <a href="/" class="p-4 bg-violet-300 rounded-xl"> Homepage </a>
            <p class="bold text-xl text-center"> decrypt signature</p>
            <label>
                "Signature file"
                <input type="file" accept=".xades"
                    node_ref=input_ref
                />
            </label>
            <button class="p-4 bg-violet-300 rounded-xl" on:click=move |_| {
                let path = input_ref().expect("should never happen").value();
                decrypt_action.dispatch(DecryptFile {path});
            }>
            decrypt
            </button>
            <Show 
                when=move || {decrypt_error.get().is_some()}
                fallback=|| view! {}
            >
                {decrypt_error.get().unwrap()}
            </Show>
            <Show 
                when=move || {decrypt_data.get().is_some()}
                fallback=|| view! {}
            >
                    <p class="text-violet-300 text 4-xl"> {decrypt_data.get().unwrap()} </p>
            </Show>
        </div>
    }
}
