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
