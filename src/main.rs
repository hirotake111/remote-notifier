use axum::{extract::Json, routing::post, Router};
use serde::Deserialize;
use std::net::SocketAddr;
use tokio::process::Command;

#[derive(Debug, Deserialize)]
struct Notification {
    message: String,
}

async fn notify(Json(payload): Json<Notification>) -> &'static str {
    let message = &payload.message;

    #[cfg(target_os = "macos")]
    {
        let script = format!(
            "display notification \"{}\" with title \"Remote Notifier\"",
            message.replace('"', "\\\"")
        );
        let _ = Command::new("osascript")
            .arg("-e")
            .arg(&script)
            .output()
            .await;
    }

    #[cfg(target_os = "linux")]
    {
        let _ = Command::new("notify-send")
            .arg("Remote Notifier")
            .arg(message)
            .output()
            .await;
    }

    "OK"
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", post(notify));

    let addr = SocketAddr::from(([0, 0, 0, 0], 9000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("SERVER running on http://{}", addr);

    axum::serve(listener, app).await.unwrap();
}
