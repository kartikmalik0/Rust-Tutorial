use axum::{ routing::get, Json, Router };
use serde::Serialize;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let router1 = Router::new().route("/v", get(v_get));

    //  define the ip and port listner
    let address = "0.0.0.0:3000";
    let listner = tokio::net::TcpListener::bind(address).await.unwrap();

    // axum server to launcyh the web server

    axum::serve(listner, router1).await.unwrap();
}

#[derive(Serialize)]
struct V {
    manu: String,
    model: String,
    year: u32,
    id: String,
}

async fn v_get() -> Json<V> {
    Json::from(V {
        manu: "Maruti".to_string(),
        model: "Maruti 800".to_string(),
        year: 2020,
        id: Uuid::new_v4().to_string(),
    })
}
