use axum::{extract::Json, routing::post, Router};
use daemonize::Daemonize;
use serde::Deserialize;
use std::net::SocketAddr;
use tokio::process::Command;

const TUNNEL_PID_FILE: &str = "/tmp/remote-notifier-tunnel.pid";
const SERVER_PID_FILE: &str = "/tmp/remote-notifier.pid";
const LOG_FILE: &str = "/tmp/remote-notifier.log";

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

fn start_tunnel(user_host: &str) -> Result<u32, String> {
    eprintln!("DEBUG: Starting tunnel to {}", user_host);
    
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        eprintln!("DEBUG: Testing SSH connection...");
        
        let output = Command::new("ssh")
            .args([
                "-o", "BatchMode=yes",
                "-o", "ConnectTimeout=10",
                "-o", "ExitOnForwardFailure=yes",
                "-N",
                "-R", "9000:localhost:9000",
                "-f",
                user_host,
            ])
            .output()
            .await;

        eprintln!("DEBUG: SSH test result: {:?}", output);

        match output {
            Ok(output) if output.status.success() => {
                eprintln!("DEBUG: SSH test succeeded, spawning background tunnel...");
                
                let child = Command::new("ssh")
                    .args(["-f", "-N", "-R", "9000:localhost:9000", user_host])
                    .spawn()
                    .map_err(|e| e.to_string())?;

                let pid = child.id().ok_or("Failed to get tunnel PID")?;
                std::fs::write(TUNNEL_PID_FILE, pid.to_string()).map_err(|e| e.to_string())?;
                println!("Tunnel started (PID: {})", pid);
                Ok(pid)
            }
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let stderr = String::from_utf8_lossy(&output.stderr);
                eprintln!("DEBUG: SSH failed. stdout: {}, stderr: {}", stdout, stderr);
                Err(format!("SSH failed: {}", stderr))
            }
            Err(e) => {
                eprintln!("DEBUG: Failed to run SSH: {}", e);
                Err(format!("Failed to run SSH: {}", e))
            }
        }
    })
}

fn kill_tunnel() {
    if let Ok(pid_str) = std::fs::read_to_string(TUNNEL_PID_FILE) {
        if let Ok(pid) = pid_str.trim().parse::<u32>() {
            let _ = Command::new("kill")
                .arg("-9")
                .arg(pid.to_string())
                .spawn()
                .map(|_| ());
            let _ = std::fs::remove_file(TUNNEL_PID_FILE);
            println!("Tunnel stopped");
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    eprintln!("DEBUG: args: {:?}", args);

    let daemon_mode = args.iter().any(|a| a == "--daemon");
    let tunnel_arg = args.iter().position(|a| a == "--tunnel");
    let kill_tunnel_flag = args.iter().any(|a| a == "--kill-tunnel");

    eprintln!("DEBUG: daemon_mode: {}, tunnel_arg: {:?}, kill_tunnel_flag: {}", daemon_mode, tunnel_arg, kill_tunnel_flag);

    if kill_tunnel_flag {
        kill_tunnel();
        return;
    }

    if let Some(pos) = tunnel_arg {
        if let Some(user_host) = args.get(pos + 1) {
            match start_tunnel(user_host) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Error starting tunnel: {}", e);
                    std::process::exit(1);
                }
            }
        } else {
            eprintln!("Error: --tunnel requires a user@host argument");
            std::process::exit(1);
        }
    }

    if daemon_mode {
        use daemonize::Stdio;
        use std::fs::File;

        let stdout = File::create(LOG_FILE).unwrap();
        let stderr = stdout.try_clone().unwrap();

        let daemon = Daemonize::new()
            .pid_file(SERVER_PID_FILE)
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

