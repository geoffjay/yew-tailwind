use yew::{function_component, html, props, Children, Classes, Html, Properties};

use crate::primitives::{Base, BaseProps};
use crate::tailwind::{BorderProps, DisplayProps};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct BoxProps {
    #[prop_or_default]
    pub classes: Option<Classes>,
    #[prop_or("div".to_string())]
    pub is: String,
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub display: Option<DisplayProps>,
    #[prop_or_default]
    pub border: Option<BorderProps>,

    #[prop_or(false)]
    pub inline: bool,
    #[prop_or(false)]
    pub inline_block: bool,
}

#[function_component(Box)]
pub fn _box(props: &BoxProps) -> Html {
    let el = match &props.is == "div" && (props.inline || props.inline_block) {
        true => "span".to_string(),
        false => props.is.clone(),
    };

    let mut classes = Classes::new();
    classes.push(&props.classes);

    let display = props.display.as_ref().cloned();
    let border = props.border.as_ref().cloned();

    let base_props = props! {
        BaseProps {
            is: el,
            classes: classes,
            inline: props.inline,
            inline_block: props.inline_block,
            border: border,
            display: display,
        }
    };

    html! {
        <Base ..base_props>
            { for props.children.iter() }
        </Base>
    }
}
