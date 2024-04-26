use super::tokenizer::{self, Token, TokenTypes};
use bson::de;
use log::{debug, error, info};

use serde::{Deserialize, Serialize};
use tokio::sync::oneshot::error;

pub struct Parser<'a> {
    string: &'a str,
    tokenizer: tokenizer::Tokenizer<'a>,
    lookahead: Option<Token>,
}

impl<'a> Parser<'a> {
    pub fn new() -> Self {
        let initial_string: &'a str = "";
        Parser {
            string: initial_string, /* initialize fields if there are any */
            tokenizer: tokenizer::Tokenizer::new(),
            lookahead: None,
        }
    }

    pub fn parse(&mut self, input: &'a str) -> Result<ASTNode<ASTNode<Token>>, String> {
        // Logic to parse the input string

        self.string = input;
        self.tokenizer.init(input);

        // prime the tokenizer to obtain the first token
        // which is our lookahead token, the lookahead is used for predictive parsing
        self.lookahead = self.tokenizer.get_next_token();

        // Parse recursively starting from the main entry point, the Program:
        self.program()
    }

    fn program(&mut self) -> Result<ASTNode<ASTNode<Token>>, String> {
        // Implementation of what a program method should do
        Ok(ASTNode {
            node_type: NodeTypes::Program,
            value: self.literal()?,
        })
    }
    fn eat(&mut self, expected_token_type: TokenTypes) -> Option<Token> {
        // Check if the current token is of the expected type
        // If it is, consume it and move to the next token
        // If it is not, raise an error
        let lookahead = self.lookahead.clone();

        match lookahead {
            None => {
                let panic = format!(
                    "Unexpected end of input, expected: {:?}",
                    expected_token_type
                );
                error!("{}", panic);
                panic!("{}", panic);
            }
            Some(token) => {
                if token.token_type != expected_token_type {
                    let a = format!(
                        "Unexpected token: {:?},expected: {:?}",
                        token.value, expected_token_type
                    );
                    error!("{}", a);
                    panic!("{}", a);
                } else {
                    self.lookahead = self.tokenizer.get_next_token();
                    Some(token)
                }
            }
        }
    }

    /**
     * Literal
     * : NumericLiteral
     * | StringLiteral
     */
    fn literal(&mut self) -> Result<ASTNode<Token>, String> {
        match &self.lookahead {
            Some(token) => match token.token_type {
                TokenTypes::NUMBER => Ok(self.numeric_literal()),
                TokenTypes::STRING => Ok(self.string_literal()),
                TokenTypes::WHITESPACE => {
                    debug!("Skipping white space {}", token.value);
                    self.eat(TokenTypes::WHITESPACE);
                    self.literal()
                }
                _ => {
                    let msg = format!("Unexpected Literal Production: {:?}", token);
                    error!("{}", msg);
                    Err(msg)
                }
            },
            None => {
                let msg = format!("Unexpected end of input");
                error!("{}", msg);
                Err(msg)
            }
        }
    }
    fn white_space(&mut self) {
        // Consume white space
        while let Some(token) = &self.lookahead {
            if token.token_type == TokenTypes::WHITESPACE {
                self.eat(TokenTypes::WHITESPACE);
            } else {
                break;
            }
        }
    }
    /***
     * NUmericLiteral
     *  : NUMBER
     *  ;
     */
    fn numeric_literal(&mut self) -> ASTNode<Token> {
        let token = self.eat(TokenTypes::NUMBER).unwrap();
        return ASTNode {
            node_type: NodeTypes::NumericLiteral,
            value: token,
        };
    }
    /***
     * StringLiteral
     *  : STRING
     *  ;
     */
    fn string_literal(&mut self) -> ASTNode<Token> {
        let token = self.eat(TokenTypes::STRING).unwrap();
        return ASTNode {
            node_type: NodeTypes::StringLiteral,
            value: token,
        };
    }

    /***
     * PLUS
     *  : '
     *  ;
     */
    fn unary_expression(&mut self) -> ASTNode<Token> {
        let operator = self.eat(TokenTypes::PLUS).unwrap();
        let operand = self.eat(TokenTypes::NUMBER).unwrap();
        return ASTNode {
            node_type: NodeTypes::UnaryExpression,
            value: operator,
        };
    }
}

#[derive(Debug, Serialize, Deserialize)]

pub enum NodeTypes {
    Program,
    NumericLiteral,
    StringLiteral,
    BinaryExpression,
    UnaryExpression,
}

#[derive(Debug, Serialize, Deserialize)]

pub struct ASTNode<T> {
    node_type: NodeTypes,
    value: T,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_new_language() {
        let parser = Parser::new();
        // Assert conditions or attributes here if any; for now, it's just instantiation
        // assert_eq!(1, 1); // Dummy assertion, replace with actual checks
    }

    #[test]
    fn test_parse() {
        // Since parse currently only prints, we can't assert on output without capturing it
        // So we are just ensuring it doesn't crash
        let mut parser = Parser::new();
        let input = "3245".to_string();
        let ast = parser.parse(&input);
        println!("{:#?}", ast);
    }
}
