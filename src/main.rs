mod structs;

use crate::structs::*;
use cs381_hw4::*;

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::post;
use axum::{Json, Router};
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

impl IntoResponse for QuestionOneError {
    fn into_response(self) -> Response {
        let (status, body) = match self {
            QuestionOneError::LengthMismatch => (StatusCode::BAD_REQUEST, self.to_string()),
        };
        (status, Json(serde_json::json!({ "error": body }))).into_response()
    }
}

fn do_question_one(a: Vec<u32>, b: Vec<u32>) -> Result<u32, QuestionOneError> {
    if a.len() != b.len() {
        return Err(QuestionOneError::LengthMismatch);
    }

    let result = dp_best_cost(&a, &b, a.len());
    Ok(result)
}
async fn question_one(Json(payload): Json<QuestionOne>) -> impl IntoResponse {
    match do_question_one(payload.a, payload.b) {
        Ok(result) => (StatusCode::OK, Json(QuestionOneAnswer { answer: result })).into_response(),
        Err(e) => e.into_response(),
    }
}

impl IntoResponse for QuestionThreeError {
    fn into_response(self) -> Response {
        let (status, body) = match self {
            QuestionThreeError::NTooBig => (StatusCode::BAD_REQUEST, self.to_string()),
        };
        (status, Json(serde_json::json!({ "error": body }))).into_response()
    }
}

fn do_question_three(n: u32) -> Result<u32, QuestionThreeError> {
    if n > 50 {
        return Err(QuestionThreeError::NTooBig);
    }

    let result = most_keypresses(n);
    Ok(result)
}

async fn question_three(Json(payload): Json<QuestionThree>) -> impl IntoResponse {
    match do_question_three(payload.n) {
        Ok(result) => {
            (StatusCode::OK, Json(QuestionThreeAnswer { answer: result })).into_response()
        }
        Err(e) => e.into_response(),
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/1", post(question_one))
        .route("/3", post(question_three))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http());

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:10000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
