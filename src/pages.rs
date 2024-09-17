use crate::structs::*;
use cs381_hw4::*;

use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;

fn do_question_one(a: Vec<u32>, b: Vec<u32>) -> Result<u32, QuestionOneError> {
    if a.len() != b.len() {
        return Err(QuestionOneError::LengthMismatch);
    }

    let result = dp_best_cost(&a, &b, a.len());
    Ok(result)
}
pub async fn question_one(Json(payload): Json<QuestionOne>) -> impl IntoResponse {
    match do_question_one(payload.a, payload.b) {
        Ok(result) => (StatusCode::OK, Json(QuestionOneAnswer { answer: result })).into_response(),
        Err(e) => e.into_response(),
    }
}

fn do_question_two(s: Vec<u32>, l: Vec<u32>) -> Result<u32, QuestionTwoError> {
    if s.len() != l.len() {
        return Err(QuestionTwoError::LengthMismatch);
    }

    let result = dj(&s, &l);
    Ok(result)
}
pub async fn question_two(Json(payload): Json<QuestionTwo>) -> impl IntoResponse {
    match do_question_two(payload.s, payload.l) {
        Ok(result) => (StatusCode::OK, Json(QuestionTwoAnswer { answer: result })).into_response(),
        Err(e) => e.into_response(),
    }
}

fn do_question_three(n: u32) -> Result<u32, QuestionThreeError> {
    if n > 50 {
        return Err(QuestionThreeError::NTooBig);
    }

    let result = most_keypresses(n);
    Ok(result)
}

pub async fn question_three(Json(payload): Json<QuestionThree>) -> impl IntoResponse {
    match do_question_three(payload.n) {
        Ok(result) => {
            (StatusCode::OK, Json(QuestionThreeAnswer { answer: result })).into_response()
        }
        Err(e) => e.into_response(),
    }
}
