use leptos::*;

use crate::app::GlobalState;

#[server(GenerateUserKeys, "/api")]
pub async fn generate_user_keys(pin: String, email: String, token: String) -> Result<String, ServerFnError> {
    #[cfg(feature="ssr")]
    {
        use crate::crypto_server::generate_keys;
        use crate::user_repository::{insert_keys, replace_token};
        use crate::api_utils::validate_user_token;

        let _ = validate_user_token(token, email.clone()).await?;
        let keys = generate_keys(&pin);
        let _db_res = insert_keys(email.clone(), &keys).await;
        let new_token = match replace_token(email).await {
            Ok(t) => t,
            Err(_)=> String::new()
        };
        Ok(new_token)
    }
}

pub fn GenerateKeys() -> impl IntoView {

    let global_state = expect_context::<RwSignal<GlobalState>>();

    if global_state.get().token.is_empty() {
        let navigate = leptos_router::use_navigate();
        navigate("/", Default::default());
    }

    let (user_pin, set_user_pin) = create_signal("".to_owned());
    let generate_action = create_server_action::<GenerateUserKeys>();
    let on_click = move |_| {
        let state_snap = global_state.get();
        generate_action.dispatch(GenerateUserKeys{pin: user_pin.get(), email: state_snap.email, token: state_snap.token});
    };
    let token = Signal::derive(move ||{
        let res = generate_action.value().get();
        match res {
            Some(e) => {
                match e {
                    Ok(s) => s,
                    Err(_) => String::new()
                }
            },
            None => String::new()
        }
    });

    let error = Signal::derive(move || {
        let res = generate_action.value().get();
        match res {
            Some(val) => {
                match val {
                    Ok(_) => None,
                    Err(e) => Some(e.to_string())
                }
            },
            None => None
        }
    });

    create_effect(move |_| {
        let token = token.get();
        if !token.is_empty() {
            let state_snap = global_state.get();
            global_state.set(GlobalState{email: state_snap.email, token});
            let navigate = leptos_router::use_navigate();
            navigate("/", Default::default());
        } 
    });

    view! {
        <div class="flex flex-center items-center justify-center flex-col gap-1 text-center h-screen w-screen bg-gray-600">
            <h1 class="text-violet-300 text-3xl fixed top-4"> Generate your encryption keys </h1>
            <Show
                when=move || {!generate_action.pending().get()}
                fallback=|| view! {"Generating keys. This process may take a while. \n After your keys have successfully generated, you will be redirected to the home page"}
            >
                <h2 class="bold text-xl text-center"> Please enter your unique password. It will be used during encryption process, so please store it somewhere securely, as you will be required to provide it while signing documents</h2>
                <input class="border border-violet-700" type="text"
                    on:input=move |ev| {
                        set_user_pin(event_target_value(&ev));
                    }
                prop:value=user_pin
                />
                <button class="p-4 bg-violet-300 rounded-xl" on:click=on_click> Generate keys </button>
            </Show>
            <Show
                when=move || {error.get().is_some()}
                fallback=|| {view!{}}
            >
                {error.get().unwrap()}
            </Show>
        </div>
    }
}

