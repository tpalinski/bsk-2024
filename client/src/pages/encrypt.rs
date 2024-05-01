use leptos::*;

#[server(EncryptFile, "/api")]
async fn encrypt_file(path: String, user: String) -> Result<String, ServerFnError> {
    #[cfg(feature="ssr")]
    {
        use crate::{filesystem::{get_data_from_fake_path, save_to_fake_path}, rsa::encrypt_data};

        // load files
        let data = get_data_from_fake_path(path.clone());
        let ciphertext = match encrypt_data(&data, user.clone()).await {
            Ok(res) => res,
            Err(e) => {
                let resp = expect_context::<leptos_actix::ResponseOptions>();
                resp.set_status(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR);
                return Err(ServerFnError::ServerError(e))
            }
        };
        save_to_fake_path(ciphertext.into_bytes(), path, ".ciphertext");
        Ok(format!("File encrypted successfully. You can now send it to {user}"))

    }
}

#[component]
pub fn EncryptPage() -> impl IntoView {
    let input_ref: NodeRef<html::Input> = create_node_ref();
    let encrypt_action = create_server_action::<EncryptFile>();
    let (email, set_email) = create_signal(String::new());

    let encrypt_error = Signal::derive(move || {
        let val = encrypt_action.value().get();
        if val.is_some() {
            match val.unwrap() {
                Ok(_) => None,
                Err(e) => Some(e.to_string())
            }
        } else {
            None
        }
    });

    let encrypt_data = Signal::derive(move || {
        let action_data = encrypt_action.value().get();
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
            <p class="bold text-xl text-center"> Encrypt a file using RSA</p>
            <label>
                "File to encrypt"
                <input type="file" accept=".txt,.pdf"
                    node_ref=input_ref
                />
            </label>
            <label>
                "Recepient's registered email"
                <input type="email" name="email"
                    on:input=move |ev| {
                        set_email(event_target_value(&ev));
                    }
                    prop:value=email
                />
            </label>

            <Show 
                when=move || {encrypt_action.pending().get() != true}
                fallback=|| view! {<p class="p-4 bg-violet-300 rounded-xl"> Loading... </p>}
            >
                <button class="p-4 bg-violet-300 rounded-xl" on:click=move |_| {
                    let path = input_ref().expect("should never happen").value();
                    encrypt_action.dispatch(EncryptFile{path, user: email.get()});
                }>
                encrypt
                </button>
            </Show>
            <Show 
                when=move || {encrypt_error.get().is_some()}
                fallback=|| view! {}
            >
                {encrypt_error.get().unwrap()}
            </Show>
            <Show 
                when=move || {encrypt_data.get().is_some()}
                fallback=|| view! {}
            >
                    <p class="text-violet-300 text 4-xl"> {encrypt_data.get().unwrap()} </p>
            </Show>
        </div>
    }
}
