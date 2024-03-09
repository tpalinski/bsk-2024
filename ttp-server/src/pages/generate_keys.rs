use leptos::*;

use crate::model::user_keys::UserKeys;


#[server(GenerateUserKeys, "/api")]
pub async fn generate_user_keys(pin: String) -> Result<UserKeys, ServerFnError> {
    #[cfg(feature="ssr")]
    {
        use crate::crypto_server::generate_keys;
        use crate::user_repository::insert_keys;
        println!("geneating keys");
        let keys = generate_keys(&pin);
        println!("finished generating keys");
        println!("Saving user keys...");
        let _db_res = insert_keys("based@mail.com".to_owned(), &keys).await;
        println!("Finished ssaving keys");
        Ok(keys)
    }
}

#[component]
fn KeyDisplay(keys: Signal<UserKeys>) -> impl IntoView {
    view! {
        <p> Public key: {move || keys.get().keys().0} </p>
        <p> Private key: {move || keys.get().keys().1} </p>
    }
}

pub fn GenerateKeys() -> impl IntoView {

    let (user_pin, set_user_pin) = create_signal("".to_owned());
    let generate_action = create_server_action::<GenerateUserKeys>();
    let on_click = move |_| {
        generate_action.dispatch(GenerateUserKeys{pin: user_pin.get()});
    };
    let keys = Signal::derive(move ||{
        let res = generate_action.value().get();
        match res {
            Some(value) => value.unwrap(),
            None => UserKeys::new(String::new(), String::new())
        }
    });

    view! {
        <div class="flex flex-center items-center justify-center flex-col gap-1 text-center h-screen w-screen bg-gray-600">
            <h1 class="text-violet-300 text-3xl fixed top-4"> Generate your encryption keys </h1>
            <Show
                when=move || {!generate_action.pending().get()}
                fallback=|| view! {"Generating keys. This process may take a while"}
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
                when=move || {(!generate_action.pending().get()) && generate_action.version().get() > 0}
                fallback=|| view! {}
            >
                <KeyDisplay keys=keys/>
            </Show>
        </div>
    }
}

