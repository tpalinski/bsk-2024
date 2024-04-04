use leptos::*;

use crate::app::GlobalState;

#[server(SignFile, "/api")]
async fn sign_file(path: String) -> Result<(), ServerFnError> {
    #[cfg(feature="ssr")]
    {
        use std::path::PathBuf;
        use crate::filesystem::{get_file_contents, save_to_file};
        use crate::rsa::sign_data;

        // TODO - move it to some top level fn
        let mut file_name = path.rsplit('\\').collect::<Vec<&str>>()[0].to_owned();
        let path = PathBuf::from(file_name.clone());
        let data = get_file_contents(dbg!(path.clone()));
        let signature = sign_data(data);
        file_name.push_str(".sign");
        let path = PathBuf::from(file_name);
        save_to_file(signature.into_bytes(), path);
    }
    Ok(())
}

#[component]
pub fn SignaturePage() -> impl IntoView {
    // Creates a reactive value to update the button

    let input_ref: NodeRef<html::Input> = create_node_ref();
    let sign_action = create_server_action::<SignFile>();

    let global_state = expect_context::<RwSignal<GlobalState>>();

    if global_state.get().token.is_empty() {
        let navigate = leptos_router::use_navigate();
        navigate("/login", Default::default());
    }

    view! {
        <div class="bg-gray-600 h-screen w-screen flex flex-col gap-4 items-center">
            <h1 class="text-violet-300 text-6xl">"Professional RSA encryption app"</h1>
            <p class="bold text-xl text-center"> Welcome {global_state.get().name}. Please choose the file you want to sign</p>
            <input type="file" 
                node_ref=input_ref
            />
            <button on:click=move |_| {
                let path = input_ref().expect("should never happen").value();
                sign_action.dispatch(SignFile {path});
            }>
            Sign file
            </button>
        </div>
    }
}
