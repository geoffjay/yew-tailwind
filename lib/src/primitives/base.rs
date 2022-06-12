use yew::{Children, function_component, html, Properties};

#[derive(Properties, PartialEq)]
pub struct BaseProps {
    #[prop_or("div".to_string())]
    pub is: String,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Base)]
pub fn base(props: &BaseProps) -> Html {
    html! {
        <@{props.is.clone()}>
            { for props.children.iter() }
        </@>
    }
}
