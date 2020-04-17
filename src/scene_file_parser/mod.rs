pub mod lex;

use crate::def::{Float, Integer};
use lex::{parse_lex, Token, TokenWithPos};
use std::collections::VecDeque;
use std::{ops::Index, path::Path};

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

impl Into<Vec<BlockSegment>> for FileBlock {
    fn into(self) -> Vec<BlockSegment> {
        self.0
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
        object_value: PropertySet,
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
                            object_value.push(Property::from_lex(tokens));
                        }
                    }
                }
                Self::Object {
                    object_type,
                    object_value: PropertySet::from(object_value),
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
    pub fn get_object(&self) -> Option<(&str, &PropertySet)> {
        match self {
            BlockSegment::Object {
                object_type,
                object_value,
            } => Some((object_type, object_value)),
            _ => None,
        }
    }
    pub fn get_object_by_type(&self, to_find_object_type: &str) -> Option<&PropertySet> {
        match self {
            BlockSegment::Object {
                object_type,
                object_value,
            } => {
                if object_type == to_find_object_type {
                    Some(object_value)
                } else {
                    None
                }
            }
            _ => None,
        }
    }
    pub fn get_block(&self, block_type: &str) -> Option<(&Option<String>, &Vec<BlockSegment>)> {
        let block_type_ = block_type;
        if let BlockSegment::Block {
            block_type,
            name,
            block_segments,
        } = self
        {
            if block_type == block_type_ {
                return Some((name, block_segments));
            }
        }
        None
    }
}

pub trait ParseFromBlockSegment {
    type T;
    fn parse_from_segment(segment: &BlockSegment) -> Option<Self::T>;
}
#[derive(Debug)]
pub struct PropertySet(Vec<Property>);
impl From<Vec<Property>> for PropertySet {
    fn from(xs: Vec<Property>) -> Self {
        Self(xs)
    }
}

impl PropertySet {
    pub fn get_name(&self) -> Option<&str> {
        self[0].get_string()
    }
    pub fn get_string(&self, name: &str) -> Option<String> {
        self.get_value::<String>(name)
    }
    pub fn get_value<T: ParseFromProperty>(&self, name: &str) -> Option<T> {
        let name_ = name;
        for p in &self.0 {
            if let Property::TypedValue {
                type_name,
                name,
                values,
            } = p
            {
                if name == name_ {
                    return Some(T::parse_from_property(type_name, values));
                }
            }
        }
        None
    }
    pub fn get_default<T: ParseFromProperty>(&self, name: &str) -> T {
        self.get_value(name).unwrap_or(T::parse_default())
    }
}
impl Index<usize> for PropertySet {
    type Output = BasicTypes;
    fn index(&self, index: usize) -> &Self::Output {
        match &self.0[index] {
            Property::Value(r) => r,
            Property::TypedValue {
                type_name: _,
                name: _,
                values,
            } => values,
        }
    }
}

#[derive(Debug)]
pub enum Property {
    Value(BasicTypes),
    TypedValue {
        type_name: String,
        name: String,
        values: BasicTypes,
    },
}
impl Property {
    fn from_lex(tokens: &mut VecDeque<TokenWithPos>) -> Self {
        let token = tokens.front().unwrap();
        match &token.token {
            Token::String(s) => {
                let words = s.split_whitespace().collect::<Vec<_>>();
                if words.len() == 2 {
                    match words[0] {
                        "point" | "string" | "float" | "integer" | "rgb" | "texture" => {
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
#[derive(Debug, Clone)]
pub struct BasicTypes(Vec<BasicType>);
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
    pub fn get_string(&self) -> Option<&str> {
        if let BasicType::BasicString(s) = self.0.first()? {
            Some(s)
        } else {
            None
        }
    }
    pub fn get_floats(&self) -> Option<Vec<Float>> {
        let mut r = Vec::new();
        for basic_type in &self.0 {
            match basic_type {
                BasicType::BasicFloat(f) => {
                    r.push(*f)
                }
                BasicType::BasicInteger(i) => {
                    r.push(*i as Float)
                }
                _ => panic!()
            }
        }
        Some(r)
    }
    pub fn get_float(&self) -> Option<Float> {
        if let BasicType::BasicFloat(f) = self.0.first()? {
            Some(*f)
        } else {
            Some(self.get_integer()? as Float)
        }
    }
    pub fn get_integer(&self) -> Option<Integer> {
        if let BasicType::BasicInteger(i) = self.0.first()? {
            Some(*i)
        } else {
            None
        }
    }
}
pub trait ParseFromProperty {
    fn parse_from_property(property_type: &str, basic_type: &BasicTypes) -> Self;
    fn parse_default() -> Self;
}

impl ParseFromProperty for String {
    fn parse_from_property(_: & str, basic_type: & BasicTypes) -> Self {
        String::from(basic_type.get_string().unwrap())
    }
    fn parse_default() -> Self {
        String::new()
    }
}
impl ParseFromProperty for Float {
    fn parse_from_property(_: & str, basic_type: & BasicTypes) -> Self {
        basic_type.get_float().unwrap()
    }
    fn parse_default() -> Self {
        0.
    }
}
impl ParseFromProperty for Integer {
    fn parse_from_property(_: & str, basic_type: & BasicTypes) -> Self {
        basic_type.get_integer().unwrap()
    }
    fn parse_default() -> Self {
        0
    }
}
impl ParseFromProperty for usize {
    fn parse_from_property(_: & str, basic_type: & BasicTypes) -> Self {
        basic_type.get_integer().unwrap() as usize
    }
    fn parse_default() -> Self {
        0
    }
}
#[derive(Debug, Clone)]
pub enum BasicType {
    BasicString(String),
    BasicFloat(Float),
    BasicInteger(Integer),
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

pub fn read_scene(file: &Path) -> Vec<BlockSegment> {
    let tokens = parse_lex(file);
    FileBlock::from_lex(tokens).0
}
