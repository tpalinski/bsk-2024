use std::time::Instant;

use leptos::*;

use crate::app::GlobalState;

#[server(DecryptFile, "/api")]
async fn decrypt_file(path: String, token: String, user: String, pin: String) -> Result<(String, String), ServerFnError> {
    #[cfg(feature="ssr")]
    {
        use crate::{filesystem::{get_data_from_fake_path, save_to_fake_path}, rsa::decrypt_data};

        let data = get_data_from_fake_path(path.clone());
        println!("Encrypting RSA file, size: ");
        dbg!(&data.len());
        let start = Instant::now();
        let (plaintext, new_token) = match decrypt_data(&data, user, token, pin).await {
            Ok(p) => p,
            Err(e) => {
                let resp = expect_context::<leptos_actix::ResponseOptions>();
                resp.set_status(actix_web::http::StatusCode::BAD_REQUEST);
                return Err(ServerFnError::ServerError(format!("Error while decrypting data: {e}")))
            }
        };
        save_to_fake_path(plaintext, path, ".plain");
        let elapsed = Instant::now() - start;
        println!("Done in {:?}", elapsed);
        Ok((format!("File decrypted successfully"), new_token))
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

    let (pin, set_pin) = create_signal(String::new());

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

    create_effect(move |_| {
        let action_data = decrypt_data.get();
        if action_data.is_some() {
            let res = action_data.unwrap();
            set_token(res.1);
        }
    });

    view! {
        <div class="bg-gray-600 h-screen w-screen flex flex-col gap-4 items-center">
            <h1 class="text-violet-300 text-6xl">"Professional RSA encryption app"</h1>
            <a href="/" class="p-4 bg-violet-300 rounded-xl"> Homepage </a>
            <p class="bold text-xl text-center"> decrypt signature</p>
            <label>
                "Encrypted file"
                <input type="file" accept=".ciphertext"
                    node_ref=input_ref
                />
            </label>
            <label>
                "Enter your user pin"
                <input type="password" name="pin" 
                    on:input=move |ev| {
                        set_pin(event_target_value(&ev));
                    }
                    prop:value=pin
                />
            </label>
            <button class="p-4 bg-violet-300 rounded-xl" on:click=move |_| {
                let path = input_ref().expect("should never happen").value();
                decrypt_action.dispatch(DecryptFile {path, user: global_state.get().email, token: token.get(), pin: pin.get()});
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
                    <p class="text-violet-300 text 4-xl"> {decrypt_data.get().unwrap().0} </p>
            </Show>
        </div>
    }
}
