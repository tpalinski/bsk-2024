use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

use crate::pages::{decrypt::DecryptPage, encrypt::EncryptPage, home::HomePage, login::LoginPage, signature::SignaturePage, verify::VerifyPage};


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GlobalState {
    pub email: String,
    pub token: String,
    pub name: String
}

impl GlobalState {
    pub fn new() -> Self {
        GlobalState { email: String::new(), token: String::new(), name: String::new() }
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    provide_context(create_rw_signal(GlobalState::new()));

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/client.css"/>

        // sets the document title
        <Title text="RSA encryption"/>

        <Router>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/login" view=LoginPage/>
                    <Route path="/signature" view=SignaturePage/>
                    <Route path="/verify" view=VerifyPage/>
                    <Route path="/encrypt" view=EncryptPage/>
                    <Route path="/decrypt" view=DecryptPage/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <h1>"Not Found"</h1>
    }
}
