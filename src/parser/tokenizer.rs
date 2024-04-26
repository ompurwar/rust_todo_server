use std::cmp::min;

use bson::de;
use log::{debug, info};
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
    DIV,
    EOF,
}

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

    // Extract the next token based on some delimiters, here assuming whitespace
    pub fn get_next_token(&mut self) -> Option<Token> {
        if !self.has_more_tokens() {
            return None;
        }

        // Numeric token
        // if the current charater is numeric then return the numeric token
        if self.string[self.cursor..]
            .chars()
            .next()
            .unwrap()
            .is_numeric()
        {
            let start = self.cursor;
            debug!("Numeric token found At {}", start);
            // find the end of the numeric token
            let end = self.string[start..]
                .find(|c: char| !c.is_numeric())
                .map(|i| i + start)
                .unwrap_or(self.string.len());

            let token = &self.string[start..end];

            debug!(
                "string: {} | Token: {:?} ended at {}",
                self.string, token, end
            );

            self.cursor = end; // move the cursor to the end of the token where first non-numeric character is found
            return Some(Token {
                token_type: TokenTypes::NUMBER,
                value: String::from(token),
                start,
                end,
            });
        }
        //String token

        if self.string[self.cursor..].chars().next().unwrap() == '\'' {
            debug!("String token found At {}", self.cursor);
            // find the end of the numeric token
            let start = self.cursor;
            let start_idx = self.cursor + 1;
            let end: usize = self.string[start_idx..]
                .find(|c: char| c == '\'')
                .map(|i| start_idx + i)
                .unwrap_or(self.string.len());
            info!(
                "STRING: {:#?} | START: {:?} |  End: {:?}",
                self.string[self.cursor..].to_string(),
                self.cursor,
                end
            );
            let token = &self.string[self.cursor..min(end + 1, self.string.len())]; // end + 1 since end index is exclusive
            info!("Token: {:?}", token);
            self.cursor = end + 1; // move the cursor to the end of the token where closing " is found
            return Some(Token {
                token_type: TokenTypes::STRING,
                value: String::from(token),
                start,
                end,
            });
        }
        // WHitespace ' ' token
        if self.string[self.cursor..].chars().next().unwrap() == '\"' {
            debug!("String token found At {}", self.cursor);
            // find the end of the numeric token
            let start = self.cursor;
            let start_idx = self.cursor + 1;
            let end: usize = self.string[start_idx..]
                .find(|c: char| c == '\"')
                .map(|i| start_idx + i)
                .unwrap_or(self.string.len());
            info!(
                "STRING: {:#?} | START: {:?} |  End: {:?}",
                self.string[self.cursor..].to_string(),
                self.cursor,
                end
            );
            let token = &self.string[self.cursor..min(end + 1, self.string.len())]; // end + 1 since end index is exclusive
            info!("Token: {:?}", token);
            self.cursor = end + 1; // move the cursor to the end of the token where closing " is found
            return Some(Token {
                token_type: TokenTypes::STRING,
                value: String::from(token),
                start,
                end,
            });
        }
        // WHitespace ' ' token
        if self.string[self.cursor..].chars().next().unwrap() == ' ' {
            // find the end of the numeric token
            let start = self.cursor;
            let end = 1 + self.cursor;
            let token = &self.string[self.cursor..end];
            self.cursor = end;
            return Some(Token {
                token_type: TokenTypes::WHITESPACE,
                value: String::from(token),
                start,
                end,
            });
        }
        // PLUS '+' token
        if self.string[self.cursor..].chars().next().unwrap() == '+' {
            // find the end of the numeric token
            let start = self.cursor;
            let end = 1 + self.cursor;
            let token = &self.string[self.cursor..end];
            self.cursor = end;
            return Some(Token {
                token_type: TokenTypes::PLUS,
                value: String::from(token),
                start,
                end,
            });
        }
        // PLUS '-' token
        if self.string[self.cursor..].chars().next().unwrap() == '-' {
            // find the end of the numeric token
            let start = self.cursor;
            let end = 1 + self.cursor;
            let token = &self.string[self.cursor..end];
            self.cursor = end;
            return Some(Token {
                token_type: TokenTypes::MINUS,
                value: String::from(token),
                start,
                end,
            });
        }
        // PLUS '=' token
        if self.string[self.cursor..].chars().next().unwrap() == '=' {
            // find the end of the numeric token
            let start = self.cursor;
            let end = 1 + self.cursor;
            let token = &self.string[self.cursor..end];
            self.cursor = end;
            return Some(Token {
                token_type: TokenTypes::EQUALS,
                value: String::from(token),
                start,
                end,
            });
        }
        // PLUS '*' token
        if self.string[self.cursor..].chars().next().unwrap() == '*' {
            // find the end of the numeric token
            let start = self.cursor;
            let end = 1 + self.cursor;
            let token = &self.string[self.cursor..end];
            self.cursor = end;
            return Some(Token {
                token_type: TokenTypes::MULTIPLY,
                value: String::from(token),
                start,
                end,
            });
        }
        None
    }
}
