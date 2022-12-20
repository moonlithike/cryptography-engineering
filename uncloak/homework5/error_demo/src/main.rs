use anyhow::{Result, anyhow};
use thiserror::Error;
use rand::prelude::*;

fn main() {
    println!("Hello, world!");
    random_error();
}

fn random_error() -> anyhow::Error {
    if rand::random() {
        return Err(CustomErrorOne::CaseA);
    } else {
        return Err(CustomErrorTwo::CaseB);
    }
}


#[derive(Error, Debug)]
pub enum CustomErrorOne {
    #[error("error case A")]
    CaseA,
    #[error("error case B")]
    CaseB,
}

#[derive(Error, Debug)]
pub enum CustomErrorTwo {
    #[error("error case A")]
    CaseA,
    #[error("error case B")]
    CaseB,
}
