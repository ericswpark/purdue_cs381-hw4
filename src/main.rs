use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::post;
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use thiserror::Error;

fn dp_best_cost(a: &[u8], b: &[u8], n: usize) -> u32 {
    let mut t: Vec<u32> = Vec::new();

    for (i, cost) in a.iter().enumerate().take(n) {
        if i == 0 {
            t.push(*cost as u32);
            continue;
        }

        let mut max_prev_t = None;

        // Find the maximum amount of money we could make previously given enough rest days from i
        for j in 0..i {
            // If the current day is over the previous day + amount of rest days needed
            if i > j + b[j] as usize {
                if max_prev_t.is_none() {
                    max_prev_t = Some(t[j]);
                } else if let Some(max_prev_t_val) = max_prev_t {
                    if max_prev_t_val < t[j] {
                        max_prev_t = Some(t[j]);
                    }
                }
            }
        }

        if max_prev_t.is_none() {
            max_prev_t = Some(0);
        }

        t.push(*cost as u32 + max_prev_t.unwrap());
    }

    *t.iter().max().unwrap()
}

#[derive(Deserialize)]
struct QuestionOne {
    a: Vec<u8>,
    b: Vec<u8>,
}

#[derive(Serialize)]
struct QuestionOneAnswer {
    answer: u32,
}

#[derive(Error, Debug)]
enum QuestionOneError {
    #[error("Array lengths do not match")]
    LengthMismatch,
}

impl IntoResponse for QuestionOneError {
    fn into_response(self) -> Response {
        let (status, body) = match self {
            QuestionOneError::LengthMismatch => (StatusCode::BAD_REQUEST, self.to_string()),
        };
        (status, Json(serde_json::json!({ "error": body }))).into_response()
    }
}

fn do_question_one(a: Vec<u8>, b: Vec<u8>) -> Result<u32, QuestionOneError> {
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

#[tokio::main]
async fn main() {
    let app = Router::new().route("/1", post(question_one));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:10000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
