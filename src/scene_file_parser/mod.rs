pub mod lex;

use lex::{parse_lex, Token, TokenWithPos};
use std::collections::VecDeque;
use std::path::Path;

#[derive(Debug)]
pub struct FileBlock(Vec<BlockSegment>);
impl FileBlock {
    fn from_lex(tokens: Vec<TokenWithPos>) -> Self {
        let mut tokens = tokens.into_iter().collect::<VecDeque<_>>();
        let mut segments = Vec::new();
        while tokens.len() != 0 {
            segments.push(BlockSegment::from_lex(&mut tokens));
        }
        Self(segments)
    }
}

#[derive(Debug)]
pub enum BlockSegment {
    Block {
        block_type: String,
        name: Option<String>,
        block_segments: Vec<BlockSegment>,
    },
    Object {
        object_type: String,
        object_value: Vec<Properties>,
    },
}
impl BlockSegment {
    fn from_lex(tokens: &mut VecDeque<TokenWithPos>) -> Self {
        let token = tokens.pop_front().unwrap();
        match token.token {
            Token::Type(object_type) => {
                let mut object_value = Vec::new();
                while let Some(next) = tokens.front() {
                    match next.token {
                        Token::Type(_) => break,
                        Token::BlockBegin(_) => break,
                        Token::BlockEnd(_) => break,
                        _ => {
                            object_value.push(Properties::from_lex(tokens));
                        }
                    }
                }
                Self::Object {
                    object_type,
                    object_value,
                }
            }
            Token::BlockBegin(block_type) => {
                let mut name = None;
                let mut block_segments = Vec::new();
                if let Some(next) = tokens.front() {
                    if let Token::String(_) = next.token {
                        let next = tokens.pop_front().unwrap();
                        if let Token::String(s) = next.token {
                            name = Some(s);
                        } else {
                            unreachable!()
                        }
                    }
                }
                while let Some(token) = tokens.front() {
                    match &token.token {
                        Token::BlockEnd(end_type) => {
                            if end_type == &block_type {
                                tokens.pop_front();
                                break;
                            } else {
                                token.panic("Unpaired block end");
                            }
                        }
                        _ => {
                            block_segments.push(BlockSegment::from_lex(tokens));
                        }
                    }
                }
                Self::Block {
                    block_type,
                    name,
                    block_segments,
                }
            }
            _ => {
                token.panic("Unexpected block segments");
                unreachable!()
            }
        }
    }
}
#[derive(Debug)]
pub enum Properties {
    Value(BasicTypes),
    TypedValue {
        type_name: String,
        name: String,
        values: BasicTypes,
    },
}
impl Properties {
    fn from_lex(tokens: &mut VecDeque<TokenWithPos>) -> Self {
        let token = tokens.front().unwrap();
        match &token.token {
            Token::String(s) => {
                let words = s.split_whitespace().collect::<Vec<_>>();
                if words.len() == 2 {
                    match words[0] {
                        "string" | "float" | "integer" | "rgb" | "texture" => {
                            // TypedValue
                            let words = s.split_whitespace().collect::<Vec<_>>();
                            let type_name = String::from(words[0]);
                            let name = String::from(words[1]);
                            tokens.pop_front();
                            let values = BasicTypes::from_lex(tokens);
                            return Self::TypedValue {
                                type_name,
                                name,
                                values,
                            };
                        }
                        _ => {}
                    }
                }
                // SingleString
                if let Token::String(s) = tokens.pop_front().unwrap().token {
                    Self::Value(BasicTypes(vec![BasicType::BasicString(s)]))
                } else {
                    unreachable!()
                }
            }
            _ => Self::Value(BasicTypes::from_lex(tokens)),
        }
    }
}
#[derive(Debug)]
pub struct BasicTypes (Vec<BasicType>);
impl BasicTypes {
    fn from_lex(tokens: &mut VecDeque<TokenWithPos>) -> Self {
        let token = tokens.pop_front().unwrap();
        match token.token {
            Token::Array(internal_tokens) => {
                let values = internal_tokens
                    .into_iter()
                    .map(|token| BasicType::from_lex(token))
                    .collect::<Vec<_>>();
                Self(values)
            }
            _ => {
                let value = BasicType::from_lex(token);
                Self(vec![value])
            }
        }
    }
}
#[derive(Debug)]
pub enum BasicType {
    BasicString(String),
    BasicFloat(f32),
    BasicInteger(i32),
}
impl BasicType {
    fn from_lex(token: TokenWithPos) -> Self {
        match token.token {
            Token::String(s) => Self::BasicString(s),
            Token::Integer(i) => Self::BasicInteger(i),
            Token::Float(f) => Self::BasicFloat(f),
            _ => {
                token.panic("Unexpected basic type");
                unreachable!()
            }
        }
    }
}

pub fn read_scene(file: &Path) -> FileBlock {
    let tokens = parse_lex(file);
    FileBlock::from_lex(tokens)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn read_scene_work() {
        let file_block = read_scene(Path::new("scenes/landscape/view-0.pbrt"));
        //println!("{:#?}", file_block);
    }
}
