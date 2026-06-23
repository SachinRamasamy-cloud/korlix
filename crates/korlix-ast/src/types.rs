use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum KType {
    String,
    Int,
    Float,
    Number,
    Bool,
    Null,
    List(Box<KType>),
    Record,
    Json,
    Date,
    Time,
    Url,
    Email,
    Color,
    Image,
    Icon,
    Component,
    Slot,
    Event,
    Any,
    Unknown,
}

impl std::fmt::Display for KType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KType::String => write!(f, "string"),
            KType::Int => write!(f, "int"),
            KType::Float => write!(f, "float"),
            KType::Number => write!(f, "number"),
            KType::Bool => write!(f, "bool"),
            KType::Null => write!(f, "null"),
            KType::List(t) => write!(f, "list<{}>", t),
            KType::Record => write!(f, "record"),
            KType::Json => write!(f, "json"),
            KType::Date => write!(f, "date"),
            KType::Time => write!(f, "time"),
            KType::Url => write!(f, "url"),
            KType::Email => write!(f, "email"),
            KType::Color => write!(f, "color"),
            KType::Image => write!(f, "image"),
            KType::Icon => write!(f, "icon"),
            KType::Component => write!(f, "component"),
            KType::Slot => write!(f, "slot"),
            KType::Event => write!(f, "event"),
            KType::Any => write!(f, "any"),
            KType::Unknown => write!(f, "unknown"),
        }
    }
}

impl KType {
    pub fn from_str(s: &str) -> Self {
        match s {
            "string" => Self::String,
            "int" => Self::Int,
            "float" => Self::Float,
            "number" => Self::Number,
            "bool" => Self::Bool,
            "null" => Self::Null,
            "date" => Self::Date,
            "time" => Self::Time,
            "url" => Self::Url,
            "email" => Self::Email,
            "color" => Self::Color,
            "image" => Self::Image,
            "icon" => Self::Icon,
            "component" => Self::Component,
            "slot" => Self::Slot,
            "event" => Self::Event,
            "any" => Self::Any,
            "json" => Self::Json,
            "record" => Self::Record,
            _ => Self::Unknown,
        }
    }
}
