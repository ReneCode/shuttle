use axum::{extract::Path, routing::get, Json, Router};
use serde_json::{json, Value};

async fn hello_world() -> &'static str {
    "root-level"
}

async fn get_json() -> Json<Value> {
    Json(json!({"name": 42}))
}

async fn get_day01(path: Option<Path<(String, String)>>) -> String {
    if let Some(Path((a, b))) = path {
        let val_a: i32 = a.parse().unwrap();
        let val_b: i32 = b.parse().unwrap();
        let val_result = (val_a ^ val_b).pow(3);
        let result = format!("{val_result}");
        result
    } else {
        "ok".to_string()
    }
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/json", get(get_json))
        .route("/1/:a/:b", get(get_day01));

    Ok(router.into())
}
