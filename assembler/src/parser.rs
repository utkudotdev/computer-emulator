use crate::lexer::{
    InstructionKind, LabelPropertyKind, NumberLiteralKind, Register, Token, TokenKind,
};
use std::iter::Peekable;
use std::num::ParseIntError;

#[derive(Debug)]
pub enum Node<'a> {
    Label {
        name: &'a str,
    },
    LabelReference {
        label_name: &'a str,
        reference_kind: LabelPropertyKind,
    },
    Instruction {
        kind: InstructionKind,
        arguments: Vec<Node<'a>>,
    },
    RegisterLiteral {
        register: Register,
    },
    NumberLiteral {
        value: u8,
    },
}

#[derive(Eq, PartialEq, Debug)]
pub enum ErrorTokenKind {
    Newline,
    Colon,
    Dot,
    LabelIdentifier,
    LabelProperty,
    Instruction,
    NumberLiteral,
    RegisterLiteral,
}

#[derive(Debug)]
pub enum ParseErrorKind {
    InvalidNumberLiteral { cause: ParseIntError },
    UnexpectedToken { expected_types: Vec<ErrorTokenKind> },
}

#[derive(Debug)]
pub struct ParseError<'a> {
    pub token: Option<Token<'a>>,
    pub kind: ParseErrorKind,
    pub help: Option<String>,
}

// TODO: better errors
pub struct Parser<'a, TIter>
where
    TIter: Iterator<Item = Token<'a>>,
{
    input_tokens: Peekable<TIter>,
}

impl<'a, TIter> Parser<'a, TIter>
where
    TIter: Iterator<Item = Token<'a>>,
{
    pub fn new(tokens: TIter) -> Self {
        Parser {
            input_tokens: tokens.peekable(),
        }
    }

    pub fn parse(mut self) -> Result<Vec<Node<'a>>, Vec<ParseError<'a>>> {
        let mut program: Vec<Node> = vec![];
        let mut errors: Vec<ParseError> = vec![];

        while let Some(token) = self.input_tokens.next() {
            match token {
                Token {
                    kind: TokenKind::Newline,
                    ..
                } => {}
                Token {
                    kind: TokenKind::LabelIdentifier,
                    text,
                    ..
                } => match self.parse_label(text) {
                    Ok(n) => program.push(n),
                    Err(err) => errors.push(err),
                },
                Token {
                    kind: TokenKind::Instruction { kind },
                    ..
                } => match self.parse_instruction(kind) {
                    Ok(n) => program.push(n),
                    Err(err) => errors.push(err),
                },
                other => errors.push(ParseError {
                    token: Some(other),
                    kind: ParseErrorKind::UnexpectedToken {
                        expected_types: vec![
                            ErrorTokenKind::Newline,
                            ErrorTokenKind::LabelIdentifier,
                            ErrorTokenKind::Instruction,
                        ],
                    },
                    help: None,
                }),
            }
        }

        if errors.is_empty() {
            Ok(program)
        } else {
            Err(errors)
        }
    }

    fn parse_label(&mut self, text: &'a str) -> Result<Node<'a>, ParseError<'a>> {
        match self.input_tokens.next() {
            Some(Token {
                kind: TokenKind::Colon,
                ..
            }) => Ok(Node::Label { name: text }),
            other => Err(ParseError {
                token: other,
                kind: ParseErrorKind::UnexpectedToken {
                    expected_types: vec![ErrorTokenKind::Colon],
                },
                help: None,
            }),
        }
    }

    fn parse_instruction(&mut self, kind: InstructionKind) -> Result<Node<'a>, ParseError<'a>> {
        let mut args = vec![];

        while let Some(token) = self.input_tokens.next() {
            match token {
                Token {
                    kind: TokenKind::Newline,
                    ..
                } => break,
                Token {
                    kind: TokenKind::LabelIdentifier,
                    text,
                    ..
                } => args.push(self.parse_label_reference(text)?),
                Token {
                    kind: TokenKind::RegisterLiteral { register },
                    ..
                } => args.push(Node::RegisterLiteral { register }),
                Token {
                    kind: TokenKind::NumberLiteral { kind: num_kind },
                    text,
                    ..
                } => match parse_number_literal(num_kind, text) {
                    Ok(value) => args.push(Node::NumberLiteral { value }),
                    Err(err) => {
                        return Err(ParseError {
                            token: Some(token),
                            kind: ParseErrorKind::InvalidNumberLiteral { cause: err },
                            help: None,
                        })
                    }
                },
                other => {
                    return Err(ParseError {
                        token: Some(other),
                        kind: ParseErrorKind::UnexpectedToken {
                            expected_types: vec![
                                ErrorTokenKind::Newline,
                                ErrorTokenKind::LabelIdentifier,
                                ErrorTokenKind::RegisterLiteral,
                                ErrorTokenKind::NumberLiteral,
                            ],
                        },
                        help: None,
                    })
                }
            }
        }

        Ok(Node::Instruction {
            kind,
            arguments: args,
        })
    }

    fn parse_label_reference(&mut self, text: &'a str) -> Result<Node<'a>, ParseError<'a>> {
        match self.input_tokens.next() {
            Some(Token {
                kind: TokenKind::Dot,
                ..
            }) => match self.input_tokens.next() {
                Some(Token {
                    kind: TokenKind::LabelProperty { kind: prop },
                    ..
                }) => Ok(Node::LabelReference {
                    label_name: text,
                    reference_kind: prop,
                }),
                other => Err(ParseError {
                    token: other,
                    kind: ParseErrorKind::UnexpectedToken {
                        expected_types: vec![ErrorTokenKind::LabelProperty],
                    },
                    help: None,
                }),
            },
            other => Err(ParseError {
                token: other,
                kind: ParseErrorKind::UnexpectedToken {
                    expected_types: vec![ErrorTokenKind::Dot],
                },
                help: None,
            }),
        }
    }
}

fn parse_number_literal(kind: NumberLiteralKind, text: &str) -> Result<u8, ParseIntError> {
    match kind {
        NumberLiteralKind::Decimal => u8::from_str_radix(text, 10),
        NumberLiteralKind::Hex => u8::from_str_radix(&text[2..], 16),
        NumberLiteralKind::Binary => u8::from_str_radix(&text[2..], 2),
    }
}
