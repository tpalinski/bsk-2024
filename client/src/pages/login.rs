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
            return Err(ServerFnError::ServerError("Error while logging in".to_owned()))
        }
    }
}

#[component]
pub fn LoginPage() -> impl IntoView {

    let login_action = create_server_action::<ServerLogin>();

    let handle_click = move |_| {
        login_action.dispatch(ServerLogin{email: "testowy2@mail.com".to_owned(), password: "haslomaslo".to_owned()});
    };

    create_effect(move |_| {
        let action_data = login_action.value().get();
        if action_data.is_some() {
            match action_data.unwrap() {
                Ok(res) => {
                    let state = expect_context::<RwSignal<GlobalState>>();
                    state.set(GlobalState{email: res.email, token: res.token, name: res.name});
                    let navigate = leptos_router::use_navigate();
                    navigate("/", Default::default());
                },
                Err(e) => {
                    console_log(&e.to_string())
                }
            }
        }
    });

    view! {
        <div class="bg-gray-600 h-screen w-screen flex flex-col gap-4 items-center">
            <h1 class="text-violet-300 text-6xl">"Professional RSA encryption app - login page"</h1>
            <p class="bold text-xl text-center"> Press the button to log in</p>
            <button on:click=handle_click class="p-4 bg-violet-300 rounded-xl"> Sign a document utilizing electronic signature </button>
        </div>
    }
}
