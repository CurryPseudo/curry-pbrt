use std::fs;
use std::path::Path;
use std::path::PathBuf;
use crate::def::{Float, Integer};
#[derive(Debug)]
pub struct TokenWithPos {
    pub token: Token,
    pub line: usize,
    pub column: usize,
    pub file: PathBuf,
    pub origin: String,
}
impl TokenWithPos {
    pub fn panic(&self, s: &str) {
        panic!(
            "file [{:?}] line {} column {} [{}]: {}",
            self.file, self.line, self.column, self.origin, s
        );
    }
}
#[derive(Debug)]
pub enum Token {
    Include,
    Type(String),
    BlockBegin(String),
    BlockEnd(String),
    String(String),
    Integer(Integer),
    Float(Float),
    Array(Vec<TokenWithPos>),
}
pub struct LexParser<'a> {
    token_with_pos: Vec<TokenWithPos>,
    file: &'a Path,
    line: usize,
    column: usize,
    index: usize,
    s: &'a str,
}
impl<'a> LexParser<'a> {
    fn new(s: &'a str, file: &'a Path) -> Self {
        Self {
            token_with_pos: Vec::new(),
            file,
            line: 1,
            column: 1,
            index: 0,
            s,
        }
    }
    fn next_equal(&self, c: char) -> Option<(usize, &'a str)> {
        let delta = self.s[self.index + 1..].chars().position(|_c| _c == c)? + 1;
        Some((self.index + delta, &self.s[self.index..self.index + delta]))
    }
    fn word(&self) -> Option<(usize, &'a str)> {
        let delta = self.s[self.index + 1..]
            .chars()
            .position(|c| c == ' ' || c == '\n' || c == '\t' || c == ']')?
            + 1;
        Some((self.index + delta, &self.s[self.index..self.index + delta]))
    }
    fn line(&self) -> Option<(usize, &'a str)> {
        self.next_equal('\n')
    }
    fn push_token(&mut self, token: Token, origin: &str) {
        let token_with_pos = TokenWithPos {
            token,
            file: PathBuf::from(self.file),
            line: self.line,
            column: self.column,
            origin: String::from(origin),
        };
        self.token_with_pos.push(token_with_pos);
    }
    fn panic(&self, s: &str) {
        panic!(
            "file [{:?}] line {} column {}: {}",
            self.file, self.line, self.column, s
        );
    }
    fn parse(mut self) -> Vec<TokenWithPos> {
        let mut array_left_info = None;
        while self.index != self.s.len() {
            match self.s[self.index..].chars().next().unwrap() {
                '\n' => {
                    self.index += 1;
                    self.line += 1;
                    self.column = 1;
                }
                ' ' | '\t' => {
                    self.index += 1;
                    self.column += 1;
                }
                '"' => {
                    if let Some((end, until_next_equal)) = self.next_equal('"') {
                        let content = &until_next_equal[1..];
                        let mut is_include = false;
                        if let Some(token) = self.token_with_pos.last() {
                            if let Token::Include = token.token {
                                self.token_with_pos.pop();
                                let include_file = self.file.parent().unwrap().join(Path::new(content));
                                let tokens = parse_lex(&include_file);
                                self.token_with_pos.extend(tokens.into_iter());
                                is_include = true;
                            }
                        }
                        if !is_include {
                            self.push_token(
                                Token::String(String::from(content)),
                                &self.s[self.index..end + 1],
                            );
                        }
                        self.column += end - self.index + 1;
                        self.index = end + 1;
                    } else {
                        self.panic("Unpaired '\"'");
                    }
                }
                '#' => {
                    if let Some((end, _)) = self.line() {
                        self.column += end - self.index;
                        self.index = end;
                    } else {
                        break;
                    }
                }
                '[' => {
                    array_left_info = Some((
                        self.token_with_pos.len(),
                        self.line,
                        self.column,
                        self.index,
                    ));
                    self.column += 1;
                    self.index += 1;
                }
                ']' => {
                    if let Some((array_token_begin, line, column, index)) = array_left_info {
                        let array_token = self
                            .token_with_pos
                            .drain(array_token_begin..)
                            .collect::<Vec<_>>();
                        self.token_with_pos.push(TokenWithPos {
                            token: Token::Array(array_token),
                            file: PathBuf::from(self.file),
                            line,
                            column,
                            origin: String::from(&self.s[index..self.index + 1]),
                        });
                        self.column += 1;
                        self.index += 1;
                    } else {
                        self.panic("Unpaired ']'");
                    }
                }
                'A'..='Z' => {
                    if let Some((end, word)) = self.word() {
                        if word == "Include" {
                            self.push_token(Token::Include, word);
                        } else if word.ends_with("Begin") {
                            self.push_token(
                                Token::BlockBegin(String::from(&word[..word.len() - 5])),
                                word,
                            );
                        } else if word.ends_with("End") {
                            self.push_token(
                                Token::BlockEnd(String::from(&word[..word.len() - 3])),
                                word,
                            );
                        } else {
                            self.push_token(Token::Type(String::from(word)), word);
                        }
                        self.column += end - self.index;
                        self.index = end;
                    } else {
                        break;
                    }
                }
                '-' | '0'..='9' => {
                    if let Some((end, word)) = self.word() {
                        if let Ok(i) = word.parse::<Integer>() {
                            self.push_token(Token::Integer(i), word);
                        } else if let Ok(f) = word.parse::<Float>() {
                            self.push_token(Token::Float(f), word);
                        } else {
                            self.panic(&format!("Cant parse number \"{}\"", word));
                        }
                        self.column += end - self.index;
                        self.index = end;
                    } else {
                        break;
                    }
                }
                c => {
                    self.panic(&format!("Cant parse char '{}", c));
                }
            }
        }
        self.token_with_pos
    }
}
pub fn parse_lex(file: &Path) -> Vec<TokenWithPos> {
    let s = fs::read_to_string(file).unwrap();
    LexParser::new(&s, file).parse()
}
