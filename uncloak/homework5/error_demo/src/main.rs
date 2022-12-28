use anyhow::{Result, anyhow};
use thiserror::Error;
use rand::prelude::*;

fn main() {
    println!("Hello, world!");
    random_error();
}

fn random_error() -> anyhow::Error {
    if rand::random() {
        return CustomErrorOne::CaseA("test".to_string());
    } else {
        return CustomErrorTwo::CaseB("test".to_string());
    }
}


#[derive(Error, Debug)]
pub enum CustomErrorOne {
    #[error("error case A")]
    CaseA(String),
    #[error("error case B")]
    CaseB(String),
}

#[derive(Error, Debug)]
pub enum CustomErrorTwo {
    #[error("error case A")]
    CaseA(String),
    #[error("error case B")]
    CaseB(String),
}
