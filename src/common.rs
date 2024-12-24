#![allow(unused)]

const FLEX_HORIZONTAL: &str = "display: flex; flex-direction: row; gap: 16px;";
const FLEX_VERTICAL: &str = "display: flex; flex-direction: column; gap: 16px;";
pub(crate) const HIDDEN_INPUT_STYLE: &str = "position: absolute; opacity: 0; pointer-events: none;";

/// Orientation
#[derive(Clone, PartialEq, Default)]
pub enum Orientation {
    Horizontal,
    #[default]
    Vertical,
}

impl Orientation {
    pub fn to_style(&self) -> &'static str {
        match self {
            Orientation::Horizontal => FLEX_HORIZONTAL,
            Orientation::Vertical => FLEX_VERTICAL,
        }
    }
}

/// Radio Button Size
#[derive(Clone, PartialEq, Default)]
pub enum Size {
    #[default]
    XSmall,
    Small,
    Medium,
    Large,
    XLarge,
    XXLarge,
    Custom(&'static str),
}

impl Size {
    pub fn to_style(&self) -> String {
        match self {
            Size::XSmall => "padding: 5px;".to_string(),
            Size::Small => "padding: 7px;".to_string(),
            Size::Medium => "padding: 10px;".to_string(),
            Size::Large => "padding: 20px;".to_string(),
            Size::XLarge => "padding: 25px;".to_string(),
            Size::XXLarge => "padding: 30px;".to_string(),
            Size::Custom(custom_size) => {
                format!("padding: {};  width: {};", custom_size, custom_size)
            }
        }
    }
}

/// Styling types
#[derive(Clone, PartialEq, Default)]
pub enum Type {
    Primary,
    Secondary,
    Success,
    Info,
    Warning,
    Danger,
    #[default]
    None,
    Custom(&'static str),
}

impl Type {
    pub fn to_style(&self) -> String {
        match self {
            Type::Primary => "background-color: #007bff; color: white;".to_string(),
            Type::Secondary => "background-color: #6c757d; color: white;".to_string(),
            Type::Success => "background-color: #28a745; color: white;".to_string(),
            Type::Info => "background-color: #17a2b8; color: white;".to_string(),
            Type::Warning => "background-color: #ffc107; color: white;".to_string(),
            Type::Danger => "background-color: #dc3545; color: white;".to_string(),
            Type::None => "".to_string(),
            Type::Custom(custom_color) => {
                format!("background-color: {}; color: white;", custom_color).to_string()
            }
        }
    }
}
