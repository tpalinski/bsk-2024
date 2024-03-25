use leptos::*;

use crate::{app::GlobalState, user_api::RegisterUserRequest};

#[component]
pub fn RegistrationPage() -> impl IntoView {

    let register_user = create_server_action::<RegisterUserRequest>();
    let (email, set_email) = create_signal(String::new());
    let (password, set_password) = create_signal(String::new());
    let (name, set_name) = create_signal(String::new());

    let register_response = Signal::derive(move || {
        let res = register_user.value().get();
        match res {
            Some(e) => match e {
                Ok(s) => s.token,
                Err(_) => String::new()
            },
            None => String::new()
        }
    });

    create_effect(move |_| {
        let token = register_response.get();
        if !token.is_empty() {
            let state = expect_context::<RwSignal<GlobalState>>();
            state.set(GlobalState{email: email.get(), token});
            let navigate = leptos_router::use_navigate();
            navigate("/generate", Default::default());
        } 
    });

    let on_submit = move |_| {
        let user = RegisterUserRequest{email: email.get(), password: password.get(), user_name: name.get()};
        register_user.dispatch(user);
    };

    view! {
        <div class="bg-gray-600 h-screen w-screen flex flex-col gap-4 items-center">
            <h1 class="text-violet-300 text-6xl"> "BSK2024 - TTP server (real)" </h1>
            <p class="bold text-xl text-center"> "To use our encryption services, please create an account in our system" </p>
            <label>
                "Display name"
                <input type="text" name="name"
                    on:input=move |ev| {
                        set_name(event_target_value(&ev));
                    }
                    prop:value=name
                />
            </label>
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
                when=move || {!register_user.pending().get()}
                fallback=|| view! {<div class="p-4 bg-violet-300 rounded-xl"> Loading... </div>}
            >
                <button class="p-4 bg-violet-300 rounded-xl" on:click=on_submit> Register </button>
            </Show> 
        </div>
    }
}
