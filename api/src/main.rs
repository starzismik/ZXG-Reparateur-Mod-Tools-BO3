use axum::{routing::get, Json, Router};
use serde::Serialize;
use std::net::SocketAddr;

const APP_NAME: &str = "[ZXG] Réparateur Mod Tools BO3";
const VERSION: &str = "1.0.0";
const RELEASE_DATE: &str = "2026-08-21";
const DOWNLOAD_URL: &str = "https://github.com/starzismik/ZXG-Reparateur-Mod-Tools-BO3/releases/latest";

#[derive(Serialize)]
struct ServiceStatus {
    application: &'static str,
    status: &'static str,
}

#[derive(Serialize)]
struct UpdateMetadata {
    application: &'static str,
    version: &'static str,
    release_date: &'static str,
    download_url: &'static str,
}

async fn index() -> Json<ServiceStatus> {
    Json(ServiceStatus {
        application: APP_NAME,
        status: "online",
    })
}

async fn health() -> Json<ServiceStatus> {
    Json(ServiceStatus {
        application: APP_NAME,
        status: "healthy",
    })
}

async fn update() -> Json<UpdateMetadata> {
    Json(UpdateMetadata {
        application: APP_NAME,
        version: VERSION,
        release_date: RELEASE_DATE,
        download_url: DOWNLOAD_URL,
    })
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .route("/health", get(health))
        .route("/update", get(update));

    let address = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = tokio::net::TcpListener::bind(address)
        .await
        .expect("unable to bind the update API");

    axum::serve(listener, app)
        .await
        .expect("update API stopped unexpectedly");
}
