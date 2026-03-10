#[derive(PartialEq)]
pub enum JsonParserState {
    InitObject,
    Start,
    InKey,
    InValue,
    EndObject,
}

pub enum JsonObject {
    JsonString(String),
    JsonObject(Vec<char>),
}

impl JsonObject {
    pub fn push(&mut self, ch: char) {
        match self {
            JsonObject::JsonString(s) => {
                s.push(ch);
            }
            JsonObject::JsonObject(v) => {
                v.push(ch);
            }
        }
    }

    pub fn is_valid(&self) -> bool {
        match self {
            JsonObject::JsonString(s) => is_str(&s),
            JsonObject::JsonObject(s) => {
                if is_str(&s.iter().collect::<String>().trim()) {
                    true
                } else if is_bool(&s.iter().collect::<String>().trim()) {
                    true
                } else if is_null(&s.iter().collect::<String>().trim()) {
                    true
                } else if is_number(&s.iter().collect::<String>().trim()) {
                    true
                } else {
                    false
                }
            }
        }
    }
}

fn is_str(content: &str) -> bool {
    if !content.starts_with('"') || !content.ends_with('"') {
        return false;
    }
    let mut is_backslash = false;

    let mut count_four_chars = 0;

    for ch in content.chars() {
        if count_four_chars > 0 {
            if !ch.is_digit(16) {
                return false;
            }
            count_four_chars -= 1;
            continue;
        }

        if ch == '\\' {
            is_backslash = true;
            continue;
        }
        if is_backslash {
            is_backslash = false;
            if ch == 'u' {
                count_four_chars = 4;
            } else if !matches!(ch, '"' | '\\' | '/' | 'b' | 'f' | 'n' | 'r' | 't') {
                return false;
            }
        }
    }

    return true;
}

fn is_bool(content: &str) -> bool {
    matches!(content, "true" | "false")
}

fn is_null(content: &str) -> bool {
    matches!(content, "null")
}

fn is_number(content: &str) -> bool {
    match content.parse::<f64>() {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn parse(content: String) -> bool {
    if content.is_empty() {
        return false;
    }

    if !content.starts_with("{") {
        return false;
    }

    let mut state = JsonParserState::InitObject;

    let mut is_valid = true;

    let mut json_object = JsonObject::JsonString("".to_string());

    for ch in content.chars().skip(1) {
        //dbg!(ch);
        match state {
            JsonParserState::InitObject => {
                if ch == '}' {
                    state = JsonParserState::EndObject;
                    break;
                } else if ch.is_whitespace() {
                    continue;
                } else {
                    json_object = JsonObject::JsonString(ch.to_string());
                    state = JsonParserState::InKey;
                }
            }
            JsonParserState::Start => {
                if ch.is_whitespace() {
                    continue;
                } else {
                    json_object = JsonObject::JsonString(ch.to_string());
                    state = JsonParserState::InKey;
                }
            }
            JsonParserState::InKey => {
                if ch == ':' {
                    if !json_object.is_valid() {
                        is_valid = false;
                        break;
                    }
                    state = JsonParserState::InValue;
                    json_object = JsonObject::JsonString("".to_string());
                } else {
                    json_object.push(ch);
                }
            }

            JsonParserState::InValue => {
                if ch.is_whitespace() {
                    continue;
                } else if ch == ',' {
                    if !json_object.is_valid() {
                        is_valid = false;
                        break;
                    }
                    state = JsonParserState::Start;
                } else if ch == '}' {
                    if !json_object.is_valid() {
                        is_valid = false;
                    }
                    state = JsonParserState::EndObject;
                } else {
                    json_object.push(ch);
                }
            }
            JsonParserState::EndObject => {
                break;
            }
        }
    }

    if state != JsonParserState::EndObject {
        is_valid = false;
    }

    is_valid
}
