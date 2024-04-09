use leptos::*;

use crate::app::GlobalState;

#[server(SignFile, "/api")]
async fn sign_file(path: String, token: String, name: String, pin: String) -> Result<String, ServerFnError> {
    #[cfg(feature="ssr")]
    {
        use crate::filesystem::{get_data_from_fake_path, save_to_fake_path};
        use crate::rsa::sign_data;

        // TODO - move it to some top level fn
        let data = get_data_from_fake_path(path.clone());
        let (signature, new_token) = match sign_data(&data, name, token, pin).await {
            Ok(s) => s,
            Err(e) => {
                let resp = expect_context::<leptos_actix::ResponseOptions>();
                resp.set_status(actix_web::http::StatusCode::BAD_REQUEST);
                return Err(ServerFnError::ServerError(e))
            }
        };
        save_to_fake_path(signature.into_bytes(), path, ".xades");
        Ok(new_token)
    }
}

#[component]
pub fn SignaturePage() -> impl IntoView {
    let input_ref: NodeRef<html::Input> = create_node_ref();
    let sign_action = create_server_action::<SignFile>();

    let global_state = expect_context::<RwSignal<GlobalState>>();
    let (token, set_token) = create_slice(global_state, |state| state.token.clone(), |state, token| state.token = token);

    if token.get().is_empty() {
        let navigate = leptos_router::use_navigate();
        let mut options = leptos_router::NavigateOptions::default();
        options.replace = true;
        navigate("/login", options);
    }

    let (pin, set_pin) = create_signal(String::new());


    let sign_error = Signal::derive(move || {
        let val = sign_action.value().get();
        if val.is_some() {
            match val.unwrap() {
                Ok(_) => None,
                Err(e) => Some(e.to_string())
            }
        } else {
            None
        }
    });

    let sign_data = Signal::derive(move || {
        let action_data = sign_action.value().get();
        if action_data.is_some() {
            match action_data.unwrap() {
                Ok(res) => Some(res),
                Err(_) => None
            }
        } else {
            None
        }
    });

    create_effect(move |_| {
        let action_data = sign_data.get();
        if action_data.is_some() {
            let res = action_data.unwrap();
            set_token(res);
        }
    });

    view! {
        <div class="bg-gray-600 h-screen w-screen flex flex-col gap-4 items-center">
            <h1 class="text-violet-300 text-6xl">"Professional RSA encryption app"</h1>
            <a href="/" class="p-4 bg-violet-300 rounded-xl"> Homepage </a>
            <p class="bold text-xl text-center"> Welcome {global_state.get().name}. Please choose the file you want to sign</p>
            <label>
                "Pick a file"
                <input type="file" accept=".txt,.pdf"
                    node_ref=input_ref
                />
            </label>
            <label>
                "Enter your user pin"
                <input type="text" name="text" 
                    on:input=move |ev| {
                        set_pin(event_target_value(&ev));
                    }
                    prop:value=pin
                />
            </label>
            <button class="p-4 bg-violet-300 rounded-xl" on:click=move |_| {
                let path = input_ref().expect("should never happen").value();
                sign_action.dispatch(SignFile {path, name: global_state.get().email, token: global_state.get().token, pin: pin.get()});
            }>
            Sign file
            </button>
            <Show 
                when=move || {sign_error.get().is_some()}
                fallback=|| view! {}
            >
                {sign_error.get().unwrap()} {"\n To try and sign the file again, please log in once again"}
            </Show>
            <Show 
                when=move || {sign_data.get().is_some()}
                fallback=|| view! {}
            >
                Document signed successfully
            </Show>
        </div>
    }
}
