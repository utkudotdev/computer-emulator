use crate::location::Location;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum InstructionKind {
    NOP,
    STR,
    LOD,
    LDI,
    INC,
    DEC,
    MOV,
    INP,
    OUT,
    SEP,
    RSP,
    ADD,
    SUB,
    BOR,
    AND,
    CMP,
    GRT,
    LES,
    BRN,
    SSJ,
    RSJ,
    RET,
    SSF,
    RSF,
}

impl InstructionKind {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "nop" => Some(InstructionKind::NOP),
            "str" => Some(InstructionKind::STR),
            "lod" => Some(InstructionKind::LOD),
            "ldi" => Some(InstructionKind::LDI),
            "inc" => Some(InstructionKind::INC),
            "dec" => Some(InstructionKind::DEC),
            "mov" => Some(InstructionKind::MOV),
            "inp" => Some(InstructionKind::INP),
            "out" => Some(InstructionKind::OUT),
            "sep" => Some(InstructionKind::SEP),
            "rsp" => Some(InstructionKind::RSP),
            "add" => Some(InstructionKind::ADD),
            "sub" => Some(InstructionKind::SUB),
            "bor" => Some(InstructionKind::BOR),
            "and" => Some(InstructionKind::AND),
            "cmp" => Some(InstructionKind::CMP),
            "grt" => Some(InstructionKind::GRT),
            "les" => Some(InstructionKind::LES),
            "brn" => Some(InstructionKind::BRN),
            "ssj" => Some(InstructionKind::SSJ),
            "rsj" => Some(InstructionKind::RSJ),
            "ret" => Some(InstructionKind::RET),
            "ssf" => Some(InstructionKind::SSF),
            "rsf" => Some(InstructionKind::RSF),
            _ => None,
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Register {
    A,
    X,
    Y,
    Z,
}

impl Register {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "a" => Some(Register::A),
            "x" => Some(Register::X),
            "y" => Some(Register::Y),
            "z" => Some(Register::Z),
            _ => None,
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum LabelPropertyKind {
    Address,
    Page,
}

impl LabelPropertyKind {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "addr" => Some(LabelPropertyKind::Address),
            "page" => Some(LabelPropertyKind::Page),
            _ => None,
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum NumberLiteralKind {
    Decimal,
    Hex,
    Binary,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum TokenKind {
    Newline,
    Colon,
    Dot,
    LabelIdentifier,
    LabelProperty { kind: LabelPropertyKind },
    Instruction { kind: InstructionKind },
    NumberLiteral { kind: NumberLiteralKind },
    RegisterLiteral { register: Register },
}

#[derive(Clone, Debug)]
pub struct Token<'a> {
    pub kind: TokenKind,
    pub location: Location,
    pub text: &'a str,
}

pub struct Lexer<'a> {
    /// The program. Contains only ASCII characters.
    program: &'a str,
    /// The current location of the lexer in the program.
    location: Location,
}

impl<'a> Lexer<'a> {
    /// Creates a new lexer given a program. The program must only contain ASCII characters.
    ///
    /// # Arguments
    ///
    /// * `program`: The program, containing only ASCII characters.
    pub fn new(program: &'a str) -> Self {
        assert!(program.is_ascii());

        Lexer {
            program,
            location: Location::start(),
        }
    }

    pub fn iter(&'a mut self) -> Iter<'a> {
        Iter { lexer: self }
    }

    pub fn next_token(&mut self) -> Option<Token<'a>> {
        while let Some(char) = self.current_char() {
            match char {
                c if let Some(kind) = single_char_token(c) => return self.handle_single(kind),
                c if c.is_ascii_digit() => return self.handle_number(),
                c if c.is_ascii_whitespace() => self.advance(),
                ';' => self.handle_comment(),
                _ => return self.handle_ident(),
            };
        }

        None
    }

    fn handle_single(&mut self, kind: TokenKind) -> Option<Token<'a>> {
        let current = self.current_char_as_str()?;
        self.advance();
        Some(Token {
            kind,
            location: self.location,
            text: current,
        })
    }

    fn handle_number(&mut self) -> Option<Token<'a>> {
        let start = self.location;
        let num = self.get_sequence()?;

        let kind = match num.get(0..2) {
            Some("0x") => NumberLiteralKind::Hex,
            Some("0b") => NumberLiteralKind::Binary,
            _ => NumberLiteralKind::Decimal,
        };

        Some(Token {
            kind: TokenKind::NumberLiteral { kind },
            location: start,
            text: num,
        })
    }

    fn handle_ident(&mut self) -> Option<Token<'a>> {
        let start = self.location;
        let str = self.get_sequence()?;

        let kind = match str {
            s if let Some(inst) = InstructionKind::from_str(s) => {
                TokenKind::Instruction { kind: inst }
            }
            s if let Some(reg) = Register::from_str(s) => {
                TokenKind::RegisterLiteral { register: reg }
            }
            s if let Some(prop) = LabelPropertyKind::from_str(s) => {
                TokenKind::LabelProperty { kind: prop }
            }
            _ => TokenKind::LabelIdentifier,
        };

        Some(Token {
            kind,
            location: start,
            text: str,
        })
    }

    fn handle_comment(&mut self) {
        assert!(
            self.current_char().is_some(),
            "Current char must be available to handle a comment"
        );

        while let Some(current) = self.current_char() {
            if current == '\n' {
                break;
            }
            self.advance()
        }
    }

    /// Gets a sequence of characters stretching from the current lexer location to just before a sequence terminator
    /// or the end of the file is reached. None is returned if the current location is out of bounds. The returned
    /// sequence will never be empty.
    fn get_sequence(&mut self) -> Option<&'a str> {
        let start = self.location;
        while let Some(current) = self.current_char() {
            if is_sequence_terminator(current) {
                break;
            }
            self.advance()
        }

        let result = self.program.get(start.index..self.location.index);
        assert!(!result.is_some_and(|val| val.is_empty()));

        result
    }

    /// Advances the current location of the lexer, if possible, adjusting `self.location` accordingly.
    fn advance(&mut self) {
        let current = self.current_char();

        if let Some('\n') = current {
            self.location = self.location.advance_line()
        } else if let Some(_) = current {
            self.location = self.location.advance_col()
        }
    }

    /// Returns the current character if the lexer has not finished. Returns None if it has.
    fn current_char(&self) -> Option<char> {
        self.program
            .as_bytes()
            .get(self.location.index)
            .map(|&c| c as char)
    }

    // Hack of the century
    /// Does roughly the same thing as current_char, but returns a string slice instead to make
    /// constructing tokens easier.
    fn current_char_as_str(&self) -> Option<&'a str> {
        let index = self.location.index;
        self.program.get(index..=index)
    }
}

fn is_sequence_terminator(c: char) -> bool {
    single_char_token(c).is_some() || c.is_ascii_whitespace() || c == ';'
}

fn single_char_token(c: char) -> Option<TokenKind> {
    match c {
        '\n' => Some(TokenKind::Newline),
        ':' => Some(TokenKind::Colon),
        '.' => Some(TokenKind::Dot),
        _ => None,
    }
}

pub struct Iter<'a> {
    lexer: &'a mut Lexer<'a>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.lexer.next_token()
    }
}
