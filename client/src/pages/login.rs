use leptos::{leptos_dom::logging::console_log, *};

use crate::{app::GlobalState, http_client::login, model::LoginResponse};

// I only do it this way to avoid cors shenanigans
#[server(ServerLogin, "/api")]
async fn server_login(email: String, password: String) -> Result<LoginResponse, ServerFnError> {
    let res = login(email, password).await;
    match res {
        Ok(data) => Ok(data),
        Err(status) => {
            let resp = expect_context::<leptos_actix::ResponseOptions>();
            resp.set_status(actix_web::http::StatusCode::from_u16(status.as_u16()).unwrap());
            return Err(ServerFnError::ServerError("Wrong credentials were provided".to_owned()))
        }
    }
}

#[component]
pub fn LoginPage() -> impl IntoView {

    let (email, set_email) = create_signal(String::new());
    let (password, set_password) = create_signal(String::new());

    let login_action = create_server_action::<ServerLogin>();
    let handle_click = move |_| {
        login_action.dispatch(ServerLogin{email: email.get(), password: password.get()});
    };

    create_effect(move |_| {
        let action_data = login_action.value().get();
        if action_data.is_some() {
            match action_data.unwrap() {
                Ok(res) => {
                    let state = expect_context::<RwSignal<GlobalState>>();
                    state.set(GlobalState{email: res.email, token: res.token, name: res.name});
                    let navigate = leptos_router::use_navigate();
                    navigate("/signature", Default::default());
                },
                Err(e) => {
                    console_log(&e.to_string())
                }
            }
        }
    });

    let login_error = Signal::derive(move || {
        let val = login_action.value().get();
        if val.is_some() {
            match val.unwrap() {
                Ok(_) => None,
                Err(e) => Some(e.to_string())
            }
        } else {
            None
        }
    });

    view! {
        <div class="bg-gray-600 h-screen w-screen flex flex-col gap-4 items-center">
            <h1 class="text-violet-300 text-6xl">"Professional RSA encryption app - login page"</h1>
            <a href="/" class="p-4 bg-violet-300 rounded-xl"> Homepage </a>
            <p class="bold text-xl text-center"> Please log in to access desired functionality</p>
            <label>
                "Email"
                <input type="email" name="email"
                    on:input=move |ev| {
                        set_email(event_target_value(&ev));
                    }
                    prop:value=email
                />
            </label>
            <label>
                "Password"
                <input type="password" name="password"
                    on:input=move |ev| {
                        set_password(event_target_value(&ev));
                    }
                    prop:value=password

                />
            </label>
            <Show 
                when=move || {login_error.get().is_some()}
                fallback=|| view! {}
            >
                {login_error.get().unwrap()}
            </Show>
            <Show
                when=move || {!login_action.pending().get()}
                fallback=|| view! {<div class="p-4 bg-violet-300 rounded-xl"> Loading... </div>}
            >
                <button on:click=handle_click class="p-4 bg-violet-300 rounded-xl"> Log in </button>
            </Show> 
        </div>
    }
}
