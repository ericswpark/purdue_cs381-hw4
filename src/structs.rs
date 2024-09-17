use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Deserialize)]
pub struct QuestionOne {
    pub(crate) a: Vec<u32>,
    pub(crate) b: Vec<u32>,
}

#[derive(Serialize)]
pub struct QuestionOneAnswer {
    pub(crate) answer: u32,
}

#[derive(Error, Debug)]
pub enum QuestionOneError {
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

#[derive(Deserialize)]
pub struct QuestionTwo {
    pub(crate) s: Vec<u32>,
    pub(crate) l: Vec<u32>,
}

#[derive(Serialize)]
pub struct QuestionTwoAnswer {
    pub(crate) answer: u32,
}

#[derive(Error, Debug)]
pub enum QuestionTwoError {
    #[error("Array lengths do not match")]
    LengthMismatch,
}

impl IntoResponse for QuestionTwoError {
    fn into_response(self) -> Response {
        let (status, body) = match self {
            QuestionTwoError::LengthMismatch => (StatusCode::BAD_REQUEST, self.to_string()),
        };
        (status, Json(serde_json::json!({ "error": body }))).into_response()
    }
}

#[derive(Deserialize)]
pub struct QuestionThree {
    pub(crate) n: u32,
}

#[derive(Serialize)]
pub struct QuestionThreeAnswer {
    pub(crate) answer: u32,
}

#[derive(Error, Debug)]
pub enum QuestionThreeError {
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
