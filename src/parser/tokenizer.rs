use std::cmp::min;

use bson::de;
use log::{debug, info};
use regex::Regex;
use serde::Serialize;

pub struct Tokenizer<'a> {
    string: &'a str,
    cursor: usize,
}

#[derive(PartialEq, Debug, Clone, Copy, Serialize)]
pub enum TokenTypes {
    NUMBER,
    STRING,
    WHITESPACE,
    PLUS,
    EQUALS,
    MINUS,
    MULTIPLY,
    DIVIDE,
    EOF,
    COMMENT,
    NEWLINE,
}
const SPEC: [(&str, TokenTypes); 11] = [
    (r"^ ", TokenTypes::WHITESPACE),
    (r"^//.*", TokenTypes::COMMENT),
    (r"^\n+", TokenTypes::NEWLINE),
    (r"^(\d+)", TokenTypes::NUMBER),
    (r"^'([^'\\]*(?:\\.[^'\\]*)*)'", TokenTypes::STRING),
    (r#"^"[^"]*""#, TokenTypes::STRING),
    (r"^\+", TokenTypes::PLUS),
    (r"^-", TokenTypes::MINUS),
    (r"^=", TokenTypes::EQUALS),
    (r"^\*", TokenTypes::MULTIPLY),
    (r"^\/", TokenTypes::DIVIDE),
];
const SKIPPABLES: [TokenTypes; 3] = [
    TokenTypes::WHITESPACE,
    TokenTypes::COMMENT,
    TokenTypes::NEWLINE,
];
#[derive(PartialEq, Debug, Clone, Serialize)]
pub struct Token {
    pub token_type: TokenTypes,
    pub value: String,
    pub start: usize,
    pub end: usize,
}

impl<'a> Tokenizer<'a> {
    pub fn new() -> Self {
        Tokenizer {
            string: &"",
            cursor: 0,
        }
    }
    fn is_eof(&self) -> bool {
        self.cursor >= self.string.len()
    }
    // Constructor should be an associated function that creates an instance of Tokenizer
    pub fn init(&mut self, input: &'a str) {
        self.string = input;
        self.cursor = 0;
    }

    // Check if more tokens are available
    fn has_more_tokens(&self) -> bool {
        self.cursor < self.string.len()
    }

    pub fn get_next_token(&mut self) -> Option<Token> {
        if !self.has_more_tokens() {
            return None;
        }

        for (re_str, token_type) in SPEC.iter() {
            debug!(
                "Trying to match regex: {} for {:?} type",
                re_str, token_type
            );
            let re = Regex::new(re_str).unwrap();
            let text_slice = &self.string[self.cursor..];
            debug!("=====================================================");
            debug!("Text slice:[{}]", text_slice);
            if let Some(mat) = re.find(text_slice) {
                debug!(
                    "Matched token: {:?} and type is : {:?}",
                    mat.as_str(),
                    token_type
                );
                let start = self.cursor + mat.start();
                let end = self.cursor + mat.end();
                if SKIPPABLES.contains(token_type) {
                    self.cursor = end;
                    debug!("Skipping: {:?}", token_type);
                    return self.get_next_token();
                }

                let token = Token {
                    token_type: *token_type,
                    value: mat.as_str().to_string(),
                    start,
                    end,
                };

                self.cursor = end; // Move cursor past the current numeric token

                return Some(token);
            }
        }

        None
    }
}
