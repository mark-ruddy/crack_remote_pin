use axum::extract::Query;
use axum::response::Html;
use axum::Extension;
use axum::{
    http::StatusCode,
    routing::{get, post},
    Router,
};
use rand::Rng;
use serde::Deserialize;
use std::error::Error;
use std::net::SocketAddr;
use std::sync::Arc;

#[derive(Deserialize)]
struct State {
    pin: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = SocketAddr::from(([0, 0, 0, 0], 9000));
    println!("listening on {}", addr);

    let mut rng = rand::thread_rng();
    let pin_unparsed = rng.gen_range(0000..9999);
    let pin = format!("{:0>4}", pin_unparsed.to_string());
    println!("Pin is: {}", pin);
    let state = Arc::new(State { pin });

    let app = Router::new()
        .route("/", get(root))
        .route("/try_pin", post(try_pin))
        .layer(Extension(state));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}

async fn root(Extension(state): Extension<Arc<State>>) -> Result<String, StatusCode> {
    Ok(format!("The pin is: {}", state.pin))
}

async fn try_pin(
    Extension(state): Extension<Arc<State>>,
    params_pin: Query<State>,
) -> Result<Html<String>, StatusCode> {
    let passed_pin: State = params_pin.0;
    println!("Received request for pin: {}", passed_pin.pin);
    let mut pin_message = "Incorrect pin";
    if passed_pin.pin == state.pin {
        pin_message = "Correct pin";
    }

    Ok(Html(format!(
        r#"
        <!doctype html>
        <html>
        <head>
        <body>
            <p>{}</p>
        </body>
        </head>
        </html>
        "#,
        pin_message
    )))
}
