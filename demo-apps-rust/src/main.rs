use axum::{response::Html, routing::get, Json, Router};
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::net::SocketAddr;

// JSON shape returned by /api/health.
// /api/health が返す JSON の形。
#[derive(Serialize)]
struct Health {
    status: &'static str,
    #[serde(rename = "secretLoaded")]
    secret_loaded: bool,
    // Truncated hash, present only when the secret was read successfully.
    // 切り詰めたハッシュ値。シークレットの読み込みに成功した場合のみ入る。
    #[serde(rename = "secretFingerprint", skip_serializing_if = "Option::is_none")]
    secret_fingerprint: Option<String>,
}

// Runs server-side only — the secret file is read here, but only a hash of
// it ever leaves this function. Even a single-binary Rust server still needs
// its secret hidden from the AI container the same way securenote-api/secrets
// is (see ../secrets/README.md).
//
// サーバー側のみで実行される — シークレットファイルはここで読み込むが、
// 外に出るのはそのハッシュ値のみ。単一バイナリの Rust サーバーであっても、
// securenote-api/secrets と同じくシークレットは AI コンテナから隠す必要が
// ある（詳細は ../secrets/README.md 参照）。
async fn health() -> Json<Health> {
    let path = std::env::var("DEMO_SECRET_PATH")
        .unwrap_or_else(|_| "./secrets/demo-secret.key".to_string());

    match tokio::fs::read_to_string(&path).await {
        Ok(secret) => {
            let mut hasher = Sha256::new();
            hasher.update(secret.trim().as_bytes());
            // Only the first 12 hex chars — enough to eyeball "did this
            // change", not enough to reconstruct the secret.
            // 先頭12文字（16進数）のみ使用 — 「変化したか」を目視確認できれば
            // 十分で、シークレットを復元できる情報量は持たせない。
            let fingerprint = format!("{:x}", hasher.finalize())[..12].to_string();
            Json(Health {
                status: "ok",
                secret_loaded: true,
                secret_fingerprint: Some(fingerprint),
            })
        }
        Err(_) => Json(Health {
            status: "ok",
            secret_loaded: false,
            secret_fingerprint: None,
        }),
    }
}

// Serves the static demo page; embedded into the binary at compile time so
// no separate static-file directory needs to ship with the release image.
// 静的なデモページを返す。コンパイル時にバイナリへ埋め込むため、
// リリースイメージに別途 static ファイル用ディレクトリを持たせる必要がない。
async fn index() -> Html<&'static str> {
    Html(include_str!("../static/index.html"))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .route("/api/health", get(health));

    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3000);
    // 0.0.0.0, not localhost/127.0.0.1 — inside the Docker container this
    // process must accept connections forwarded from outside the container,
    // not just from within it.
    // localhost/127.0.0.1 ではなく 0.0.0.0 を指定する — Docker コンテナ内では、
    // コンテナ外から転送されてくる接続を受け付ける必要があるため。
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("listening on {addr}");
    axum::serve(listener, app).await.unwrap();
}
