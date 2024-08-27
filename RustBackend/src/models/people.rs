use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Person {
    pub uid: u64,
    pub name: String,
    pub color: Colors
}

impl Person {
    pub fn new(n: String, c: Colors) -> Self {
        Person {
            uid: 0,
            name: n,
            color: c
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Colors {
    red,
    blue,
    yellow,
    none
}

impl fmt::Display for Colors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Colors::yellow => write!(f, "yellow"),
            Colors::red => write!(f, "red"),
            Colors::blue => write!(f, "blue"),
            Colors::none => write!(f, "none"),
        }
    }
}

impl From<Option<String>> for Colors {
    fn from(item: Option<String>) -> Self {
        if item.is_some() {
            let color: Colors = match item.unwrap().as_str() {
                "red" => Colors::red,
                "blue" => Colors::blue,
                "yellow" => Colors::yellow,
                _ => Colors::none,
            };

            return color
        }

        Colors::none
    }
}