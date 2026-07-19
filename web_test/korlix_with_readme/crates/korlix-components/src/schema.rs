use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentSchema {
    pub name: String,
    pub category: ComponentCategory,
    pub props: Vec<PropSchema>,
    pub slots: Vec<SlotSchema>,
    pub default_classes: Vec<String>,
    pub runtime_features: Vec<RuntimeFeature>,
    pub html_tag: String,
    pub self_closing: bool,
    pub aria_role: Option<String>,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ComponentCategory {
    Primitive,
    Media,
    Icon,
    Avatar,
    Navigation,
    Feedback,
    Loader,
    Placeholder,
    Overlay,
    Form,
    Content,
    DataDisplay,
    Marketing,
    Dashboard,
    Ecommerce,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropSchema {
    pub name: String,
    pub type_ann: String,
    pub required: bool,
    pub default: Option<String>,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlotSchema {
    pub name: Option<String>,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RuntimeFeature {
    Core,
    Router,
    State,
    Toast,
    Overlay,
    Media,
    Theme,
    Forms,
    Motion,
}
