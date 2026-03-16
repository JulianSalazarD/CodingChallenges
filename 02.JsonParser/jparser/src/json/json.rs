//! JSON Parser Implementation
//!
//! Provides a recursive descent JSON parser with tokenization and validation.

use std::collections::{HashMap, VecDeque};
use std::io::{Error, ErrorKind};

/// Maximum allowed nesting depth for JSON objects and arrays
const MAX_DEPTH: usize = 20;

#[derive(Debug, Clone, PartialEq)]
/// Represents the different types of values in JSON
pub enum Json {
    String(String),
    Value(Box<Json>),
    Array(Vec<Json>),
    Object(HashMap<String, Json>),
}

impl Json {
    /// Builds and validates a complete JSON value from the token stream.
    ///
    /// # Arguments
    /// * `json_parser` - Mutable reference to the JsonParser with tokens
    ///
    /// # Returns
    /// * `Ok(Json)` - Valid JSON object or array
    /// * `Err(Error)` - If JSON is empty, has unbalanced brackets, or invalid structure
    pub fn build(json_parser: &mut JsonParser) -> Result<Json, Error> {
        let mut depth: usize = 0;

        if json_parser.tokens.is_empty() {
            return Err(Error::new(ErrorKind::Other, "empty"));
        } else if json_parser.count_bracket != 0 || json_parser.count_brace != 0 {
            return Err(Error::new(ErrorKind::Other, "unbalanced brackets"));
        } else if !json_parser.temp_json.is_empty() {
            return Err(Error::new(ErrorKind::Other, "unexpected characters"));
        }
        if json_parser.tokens.front().unwrap() == "{" {
            let json = Json::build_object(json_parser, &mut depth)?;
            if !json_parser.tokens.is_empty() {
                return Err(Error::new(ErrorKind::Other, "expected end of array"));
            }
            return Ok(json);
        } else if json_parser.tokens.front().unwrap() == "[" {
            let json = Json::build_array(json_parser, &mut depth)?;
            if !json_parser.tokens.is_empty() {
                return Err(Error::new(ErrorKind::Other, "expected end of array"));
            }
            return Ok(json);
        } else {
            return Err(Error::new(ErrorKind::Other, "Not an object or array"));
        }
    }

    /// Recursively builds a JSON object from tokens.
    ///
    /// # Arguments
    /// * `json_parser` - Mutable reference to the JsonParser
    /// * `depth` - Current nesting depth (used for limit checking)
    ///
    /// # Returns
    /// * `Ok(Json::Object)` - Valid JSON object
    /// * `Err(Error)` - If object structure is invalid
    fn build_object(json_parser: &mut JsonParser, depth: &mut usize) -> Result<Json, Error> {
        *depth += 1;

        if *depth == MAX_DEPTH {
            return Err(Error::new(ErrorKind::Other, "exceeded max depth"));
        }

        let mut object = HashMap::new();

        let mut is_coma = false;

        if json_parser.tokens.front().unwrap() != "{" {
            return Err(Error::new(ErrorKind::Other, "expected '{'"));
        }
        json_parser.tokens.pop_front();

        while !json_parser.tokens.is_empty() {
            let key = json_parser.tokens.pop_front().unwrap();
            if key == "}" {
                if is_coma {
                    return Err(Error::new(ErrorKind::Other, "expected key"));
                }
                return Ok(Json::Object(object));
            }

            if !is_str(&key) {
                return Err(Error::new(ErrorKind::Other, "key must be a string"));
            }

            if json_parser.tokens.pop_front().unwrap() != ":" {
                return Err(Error::new(ErrorKind::Other, "expected ':' after key"));
            }

            if json_parser.tokens.front().unwrap() == "{" {
                let value = Json::build_object(json_parser, depth)?;
                object.insert(key, value);
            } else if json_parser.tokens.front().unwrap() == "[" {
                let value = Json::build_array(json_parser, depth)?;
                object.insert(key, value);
            } else {
                let value = json_parser.tokens.pop_front().unwrap();
                if !is_value(&value) {
                    return Err(Error::new(ErrorKind::Other, "value must be valid"));
                }
                object.insert(key, Json::String(value));
            }

            if json_parser.tokens.front().unwrap() == "," {
                is_coma = true;
                json_parser.tokens.pop_front();
            } else {
                is_coma = false;
            }
        }
        Ok(Json::Object(object))
    }

    /// Recursively builds a JSON array from tokens.
    ///
    /// # Arguments
    /// * `json_parser` - Mutable reference to the JsonParser
    /// * `depth` - Current nesting depth (used for limit checking)
    ///
    /// # Returns
    /// * `Ok(Json::Array)` - Valid JSON array
    /// * `Err(Error)` - If array structure is invalid
    fn build_array(json_parser: &mut JsonParser, depth: &mut usize) -> Result<Json, Error> {
        *depth += 1;

        if *depth == MAX_DEPTH {
            return Err(Error::new(ErrorKind::Other, "exceeded max depth"));
        }

        let mut array = Vec::new();

        let mut is_coma = false;

        if json_parser.tokens.front().unwrap() != "[" {
            return Err(Error::new(ErrorKind::Other, "expected '['"));
        }
        json_parser.tokens.pop_front();

        while !json_parser.tokens.is_empty() {
            match json_parser.tokens.front().unwrap().as_str() {
                "[" => {
                    let json_array = Json::build_array(json_parser, depth)?;
                    array.push(json_array);
                }
                "]" => {
                    if is_coma {
                        return Err(Error::new(ErrorKind::Other, "expected key"));
                    }
                    json_parser.tokens.pop_front();
                    return Ok(Json::Array(array));
                }
                "{" => {
                    let json_object = Json::build_object(json_parser, depth)?;
                    array.push(json_object);
                }
                _ => {
                    let value = json_parser.tokens.pop_front().unwrap();
                    if is_value(&value) {
                        array.push(Json::String(value));
                    } else {
                        return Err(Error::new(ErrorKind::Other, "value must be valid"));
                    }
                }
            }

            if json_parser.tokens.front().unwrap() == "," {
                is_coma = true;
                json_parser.tokens.pop_front();
            } else {
                is_coma = false;
            }
        }

        Ok(Json::Array(array))
    }
}

#[derive(Debug)]
/// Tokenizer for JSON content.
/// Converts a JSON string into a queue of tokens for parsing.
pub struct JsonParser {
    tokens: VecDeque<String>,
    temp_json: Vec<char>,
    count_bracket: i32,
    count_brace: i32,
}

impl JsonParser {
    /// Creates a new JsonParser instance with empty state.
    pub fn new() -> Self {
        Self {
            tokens: VecDeque::new(),
            temp_json: Vec::new(),
            count_bracket: 0,
            count_brace: 0,
        }
    }

    /// Pushes a token to the queue and tracks bracket/brace counts.
    ///
    /// # Arguments
    /// * `s` - The string token to add
    fn push(&mut self, s: String) {
        if s.is_empty() {
            return;
        } else if s == "{" {
            self.count_bracket += 1;
        } else if s == "}" {
            self.count_bracket -= 1;
        } else if s == "[" {
            self.count_brace += 1;
        } else if s == "]" {
            self.count_brace -= 1;
        }
        self.tokens.push_back(s);
    }

    /// Flushes any pending characters and pushes a delimiter token.
    ///
    /// # Arguments
    /// * `s` - The delimiter string to add
    fn push_and_clear(&mut self, s: String) {
        if !self.temp_json.is_empty() {
            self.push(self.temp_json.iter().collect::<String>());
            self.temp_json.clear();
        }
        self.push(s);
    }

    /// Tokenizes JSON content into a stream of tokens.
    ///
    /// # Arguments
    /// * `content` - The raw JSON string to tokenize
    pub fn build_json_object(&mut self, content: String) {
        self.tokens.clear();
        self.temp_json.clear();
        self.count_bracket = 0;
        self.count_brace = 0;

        let mut is_backslash = false;
        let mut in_string = false;

        for ch in content.chars() {
            if is_backslash != false && ch == '"' {
                in_string = !in_string;
            }

            if is_backslash != false && ch == '\\' {
                is_backslash = false;
            } else {
                is_backslash = true;
            }

            if in_string {
                self.temp_json.push(ch);
                continue;
            } else if ch.is_whitespace() {
                continue;
            } else if matches!(ch, '{' | '}' | '[' | ']' | ':' | ',') {
                self.push_and_clear(ch.to_string());
            } else {
                self.temp_json.push(ch);
            }
        }
    }
}

/// Validates if a string is a valid JSON string literal.
///
/// # Arguments
/// * `content` - The string to validate
///
/// # Returns
/// * `true` - If the string is valid JSON string format
/// * `false` - Otherwise
fn is_str(content: &str) -> bool {
    if content.len() < 2 || !content.starts_with('"') || !content.ends_with('"') {
        return false;
    }
    let inner = &content[1..content.len() - 1];

    let mut chars = inner.chars();
    while let Some(ch) = chars.next() {
        if (ch as u32) < 0x20 {
            return false;
        }

        if ch == '\\' {
            match chars.next() {
                Some('"') | Some('\\') | Some('/') | Some('b') | Some('f') | Some('n')
                | Some('r') | Some('t') => {
                    continue;
                }
                Some('u') => {
                    for _ in 0..4 {
                        match chars.next() {
                            Some(hex) if hex.is_ascii_hexdigit() => continue,
                            _ => return false,
                        }
                    }
                }
                _ => return false,
            }
        } else if ch == '"' {
            return false;
        }
    }

    true
}

/// Validates if a string is a valid JSON boolean literal.
///
/// # Arguments
/// * `content` - The string to validate
///
/// # Returns
/// * `true` - If the string is "true" or "false"
/// * `false` - Otherwise
fn is_bool(content: &str) -> bool {
    matches!(content, "true" | "false")
}

/// Validates if a string is a valid JSON null literal.
///
/// # Arguments
/// * `content` - The string to validate
///
/// # Returns
/// * `true` - If the string is "null"
/// * `false` - Otherwise
fn is_null(content: &str) -> bool {
    matches!(content, "null")
}

/// Validates if a string is a valid JSON number.
///
/// # Arguments
/// * `content` - The string to validate
///
/// # Returns
/// * `true` - If the string represents a valid JSON number
/// * `false` - Otherwise
fn is_number(content: &str) -> bool {
    if content.is_empty() {
        return false;
    }

    let bytes = content.as_bytes();
    let mut i = 0;

    if bytes[i] == b'-' {
        i += 1;
        if i == bytes.len() {
            return false;
        }
    }

    if bytes[i] == b'0' {
        i += 1;
        if i < bytes.len() && bytes[i].is_ascii_digit() {
            return false;
        }
    } else if bytes[i].is_ascii_digit() {
        while i < bytes.len() && bytes[i].is_ascii_digit() {
            i += 1;
        }
    } else {
        return false;
    }

    content.parse::<f64>().is_ok()
}

/// Validates if a string is any valid JSON value (string, boolean, null, or number).
///
/// # Arguments
/// * `content` - The string to validate
///
/// # Returns
/// * `true` - If the string is any valid JSON value type
/// * `false` - Otherwise
fn is_value(content: &str) -> bool {
    if is_str(content) {
        true
    } else if is_bool(content) {
        true
    } else if is_null(content) {
        true
    } else if is_number(content) {
        true
    } else {
        false
    }
}
