use std::fmt;
use yew::Properties;

#[derive(Clone, Debug, PartialEq)]
pub enum RadiusOption {
    None,
    Small,
    Default,
    Medium,
    Large,
    XLarge,
    XXLarge,
    XXXLarge,
    Full,
}

impl fmt::Display for RadiusOption {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RadiusOption::None => write!(f, "rounded-none"),
            RadiusOption::Small => write!(f, "rounded-sm"),
            RadiusOption::Default => write!(f, "rounded"),
            RadiusOption::Medium => write!(f, "rounded-md"),
            RadiusOption::Large => write!(f, "rounded-lg"),
            RadiusOption::XLarge => write!(f, "rounded-xl"),
            RadiusOption::XXLarge => write!(f, "rounded-2xl"),
            RadiusOption::XXXLarge => write!(f, "rounded-3xl"),
            RadiusOption::Full => write!(f, "rounded-full"),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum StyleOption {
    None,
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct BorderProps {
    pub radius: Option<RadiusOption>,
    pub style: Option<StyleOption>,
}

impl From<&BorderProps> for Vec<String> {
    fn from(bp: &BorderProps) -> Vec<String> {
        let mut vec = vec!["border".to_string()];

        if let Some(radius) = &bp.radius {
            vec.push(format!("border-{}", radius));
        }

        vec
    }
}
