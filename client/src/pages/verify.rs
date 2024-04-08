use leptos::*;

#[server(VerifyFile, "/api")]
async fn verify_file(path: String) -> Result<String, ServerFnError> {
    #[cfg(feature="ssr")]
    {
        use crate::filesystem::get_file_contents;
        use std::path::PathBuf;
        use crate::rsa::model::Signature;

        // load files
        let file_name = path.rsplit('\\').collect::<Vec<&str>>()[0].to_owned();
        let mut path = dirs::home_dir().unwrap();
        path.push(&file_name);
        let data_file_name = path.file_stem().unwrap();
        let xades_data = get_file_contents(path.clone());
        let mut data_path = dirs::home_dir().unwrap();
        data_path.push(data_file_name);
        let _file_data = get_file_contents(dbg!(PathBuf::from(data_path)));

        //Verify
        let xades = dbg!(Signature::from_xml(xades_data));
        Ok("Good".to_owned())
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
            Sign file
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
                    {verify_data.get().unwrap()}
            </Show>
        </div>
    }
}
