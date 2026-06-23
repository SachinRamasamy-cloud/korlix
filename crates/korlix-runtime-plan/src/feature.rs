#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
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
    A11y,
}
