use maud::{html, DOCTYPE};
use spin_sdk::{
    http::{IntoResponse, Request, Response},
    http_component,
    //sqlite::Connection,
};

/// A simple Spin HTTP component.
#[http_component]
fn handle_app(req: Request) -> anyhow::Result<impl IntoResponse> {
    println!("Handling request to {:?}", req.header("spin-full-url"));

    let markup = html! {
        (DOCTYPE)
        html {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                title { "Rust Shared Data" }
            }
            body {
                script type="module" src="/client/js/main.js" {}
                h1 { "Rust Shared Data" }
            }
        }
    };

    // TODO: api routes

    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(markup.into_string())
        .build())
}
