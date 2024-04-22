use axum::{
    body::Bytes,
    extract::Multipart,
    http::{Method, StatusCode},
    routing::{get, post},
    Router,
};
use eyre::Result;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};

// use serde::{Deserialize, Serialize};
use tokio::{fs, io::AsyncWriteExt, process::Command};

const PROJECT_DIR: &str = "/tmp/remotelab";

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new()
        .route("/", get(root))
        .route("/upload-program", post(upload_program)
        .layer(
            ServiceBuilder::new().layer(
                CorsLayer::new()
                    .allow_origin(Any)
                    .allow_methods([Method::GET, Method::POST])
            ),
        ),
    );

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

async fn upload_program(payload: Multipart) -> (StatusCode, String) {
    if let Ok(fields) = get_fields(payload).await {
        println!("Files uploaded:");
        for (name, content) in &fields {
            println!("{} ({} bytes)", name, content.len());
        }

        if let Err(e) = save_files(fields).await {
            println!("Error saving files: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error saving files".to_owned(),
            );
        }

        if let Ok(output) = compile_program().await {
            println!("Compilation output: {}", output);
            return (StatusCode::OK, output);
        } else {
            println!("Error compiling program");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error compiling program".to_owned(),
            );
        }
    } else {
        return (
            StatusCode::BAD_REQUEST,
            "Error parsing multipart data".to_owned(),
        );
    }
}

async fn get_fields(mut payload: Multipart) -> Result<Vec<(String, Bytes)>> {
    let mut fields: Vec<(String, Bytes)> = vec![];
    while let Some(field) = payload.next_field().await? {
        let name = match field.name() {
            Some(name) => name.to_string(),
            None => {
                continue;
            }
        };
        let content = field.bytes().await?;

        fields.push((name, content));
    }

    return Ok(fields);
}

async fn save_files(files: Vec<(String, Bytes)>) -> Result<()> {
    if !fs::try_exists(PROJECT_DIR).await? {
        fs::create_dir(PROJECT_DIR).await?;
    }

    let path = PROJECT_DIR.to_owned() + "/main";
    if fs::try_exists(&path).await? {
        fs::remove_dir_all(&path).await?;
    }

    fs::create_dir(&path).await?;
    for (name, content) in files {
        let mut file = fs::File::create(format!("{}/{}", &path, name)).await?;
        file.write_all(&content).await?;
    }

    return Ok(());
}

async fn compile_program() -> Result<String, std::io::Error> {
    let port: String = std::env::var("ARDUINO_PORT").unwrap_or("/dev/ttyACM0".to_string());
    let fqbn: String =
        std::env::var("ARDUINO_FQBN").unwrap_or("arduino:samd:arduino_zero_native".to_string());

    return Command::new("arduino-cli")
        .current_dir(PROJECT_DIR.to_owned() + "/main")
        .args(&["compile", "--port", &port, "--fqbn", &fqbn, "--upload"])
        .output()
        .await
        .map(|output| String::from_utf8(output.stdout).unwrap_or("".to_owned()));
}
