use axum::{extract::Json, routing::post, Router};
use daemonize::Daemonize;
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
            "display notification \"{}\" with title \"Remote Notifier\" sound name \"Ping\"",
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
            .arg("-u")
            .arg("critical")
            .arg("Remote Notifier")
            .arg(message)
            .output()
            .await;

        let _ = Command::new("paplay")
            .arg("/usr/share/sounds/freedesktop/stereo/complete.ogg")
            .output()
            .await;
    }

    "OK"
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let daemon_mode = args.iter().any(|a| a == "--daemon");

    if daemon_mode {
        use daemonize::Stdio;
        use std::fs::File;

        let stdout = File::create("/tmp/remote-notifier.log").unwrap();
        let stderr = stdout.try_clone().unwrap();

        let daemon = Daemonize::new()
            .pid_file("/tmp/remote-notifier.pid")
            .working_directory("/tmp")
            .stdout(Stdio::from(stdout))
            .stderr(Stdio::from(stderr));

        match daemon.start() {
            Ok(_) => println!("Daemon started"),
            Err(e) => eprintln!("Error starting daemon: {}", e),
        }
    }

    run_server();
}

fn run_server() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let app = Router::new().route("/", post(notify));

        let addr = SocketAddr::from(([0, 0, 0, 0], 9000));
        let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

        println!("SERVER running on http://{}", addr);

        axum::serve(listener, app).await.unwrap();
    });
}

