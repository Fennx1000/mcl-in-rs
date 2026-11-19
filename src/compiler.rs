use crate::op::Op;
use anyhow::{Result, anyhow};

#[derive(Debug)]
enum Token {
    Op(Op),
    Value(i64),
    Unknown(String),
}

fn tokenize(content: &str) -> Result<Vec<Token>> {
    let result: Vec<Token> = content
        .split_whitespace()
        .map(|raw_token| match raw_token {
            "push" => Token::Op(Op::Push),
            "pop" => Token::Op(Op::Pop),
            "print" => Token::Op(Op::Print),
            val => match val.parse::<i64>() {
                Ok(int) => Token::Value(int),
                Err(_) => Token::Unknown(val.to_string()),
            },
        })
        .collect();

    for token in &result {
        if let Token::Unknown(val) = token {
            return Err(anyhow!("Unexpected token: '{}'", val));
        }
    }
    Ok(result)
}

pub fn compile(content: &str) -> Result<Vec<u64>> {
    let tokens = tokenize(content)?;
    println!("{:#?}", tokens);
    Ok(vec![])
}
