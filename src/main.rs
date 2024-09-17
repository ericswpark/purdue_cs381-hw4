use std::cmp::min;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::post;
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

fn dp_best_cost(a: &[u32], b: &[u32], n: usize) -> u32 {
    let mut t: Vec<u32> = Vec::new();

    for (i, cost) in a.iter().enumerate().take(n) {
        if i == 0 {
            t.push(*cost);
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

        t.push(*cost + max_prev_t.unwrap());
    }

    *t.iter().max().unwrap()
}

#[derive(Deserialize)]
struct QuestionOne {
    a: Vec<u32>,
    b: Vec<u32>,
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

#[derive(Deserialize)]
struct QuestionThree {
    n: u32,
}

#[derive(Serialize)]
struct QuestionThreeAnswer {
    answer: u32,
}

#[derive(Error, Debug)]
enum QuestionThreeError {
    #[error("Please keep n below 50")]
    NTooBig,
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
        Ok(result) => (StatusCode::OK, Json(QuestionThreeAnswer { answer: result })).into_response(),
        Err(e) => e.into_response(),
    }
}

fn most_keypresses(n: u32) -> u32 {
    // Possible clipboard values range from 0 to n, so we need to have n + 1 spaces
    // To prevent overflowing, leave some headroom
    let mut t: Vec<Vec<u32>> = vec![vec![u32::MAX - 100; (n + 1) as usize]; n as usize];

    for i in 0..n {
        if i == 0 {
            t[0][0] = 1; // Base case, typing 1 with 0 in the clipboard
            t[0][1] = 4; // Base case, typing 1 with 1 in the clipboard (a - Ctrl A - Ctrl C)
            continue;
        }

        for j in 0..=i + 1 {    // From 1 to i
            // Adding 'a' with 1 keypress from previous row
            let add_a = t[(i - 1) as usize][j as usize] + 1;

            // Pasting - find i and j that sum up to the current value
            let mut paste_case = u32::MAX;
            for i_p in 0..i {
                if (i_p + 1) + j == (i + 1) {
                    paste_case = min(paste_case, t[i_p as usize][j as usize] + 2);
                }
            }

            // Copying
            let mut copy_case = u32::MAX;
            for j_p in 0..=i + 1 {
                copy_case = min(copy_case, t[i as usize][j_p as usize] + 3);
            }

            t[i as usize][j as usize] = *[add_a, copy_case, paste_case].iter().min().unwrap();
        }
    }

    *t[(n - 1) as usize].iter().min().unwrap()
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
