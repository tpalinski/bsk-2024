use leptos::*;

#[server(VerifyFile, "/api")]
async fn verify_file(path: String) -> Result<String, ServerFnError> {
    #[cfg(feature="ssr")]
    {
        use crate::{filesystem::get_verify_data, rsa::verify_signature};

        // load files
        let (xades_data, file_data) = get_verify_data(path);
        match verify_signature(&file_data, xades_data).await {
            Ok(res) => {
                if res {Ok("Signature is valid".to_owned())}
                else {Ok("Signature validation failed - not genuine!!!".to_owned())}
            },
            Err(e) => {
                let resp = expect_context::<leptos_actix::ResponseOptions>();
                resp.set_status(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR);
                return Err(ServerFnError::ServerError(e))
            }
        }

    }
}

#[component]
pub fn VerifyPage() -> impl IntoView {
    let input_ref: NodeRef<html::Input> = create_node_ref();
    let verify_action = create_server_action::<VerifyFile>();

    let verify_error = Signal::derive(move || {
        let val = verify_action.value().get();
        if val.is_some() {
            match val.unwrap() {
                Ok(_) => None,
                Err(e) => Some(e.to_string())
            }
        } else {
            None
        }
    });

    let verify_data = Signal::derive(move || {
        let action_data = verify_action.value().get();
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
            <p class="bold text-xl text-center"> Verify signature</p>
            <label>
                "Signature file"
                <input type="file" accept=".xades"
                    node_ref=input_ref
                />
            </label>
            <button class="p-4 bg-violet-300 rounded-xl" on:click=move |_| {
                let path = input_ref().expect("should never happen").value();
                verify_action.dispatch(VerifyFile {path});
            }>
            Verify
            </button>
            <Show 
                when=move || {verify_error.get().is_some()}
                fallback=|| view! {}
            >
                {verify_error.get().unwrap()}
            </Show>
            <Show 
                when=move || {verify_data.get().is_some()}
                fallback=|| view! {}
            >
                    <p class="text-violet-300 text 4-xl"> {verify_data.get().unwrap()} </p>
            </Show>
        </div>
    }
}
