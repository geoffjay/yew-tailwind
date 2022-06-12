use yew::{Children, function_component, html, Properties};
use crate::primitives::Base;

#[derive(Properties, PartialEq)]
pub struct BoxProps {
    #[prop_or("div".to_string())]
    pub is: String,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Box)]
pub fn _box(props: &BoxProps) -> Html {
    html! {
        <Base is={props.is.clone()}>
            { for props.children.iter() }
        </Base>
    }
}
