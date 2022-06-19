use yew::{function_component, html, Children, Classes, NodeRef, Properties};

use crate::tailwind::{BorderProps, DisplayProps};

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct BaseProps {
    #[prop_or("div".to_string())]
    pub is: String,
    #[prop_or_default]
    pub classes: Option<Classes>,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub inner_ref: NodeRef,

    #[prop_or_default]
    pub border: Option<BorderProps>,
    #[prop_or_default]
    pub display: Option<DisplayProps>,

    #[prop_or(false)]
    pub inline: bool,
    #[prop_or(false)]
    pub inline_block: bool,
}

// fn class_from_p(padding: String) -> String {
//     match padding.parse::<u16>().is_ok() {
//         true => format!("p-{}", padding),
//         false => format!("p{}", padding),
//     }
// }

#[function_component(Base)]
pub fn base(props: &BaseProps) -> Html {
    let mut classes = Classes::new();
    classes.push(&props.classes);

    if let Some(display) = &props.display {
        classes.push(Vec::from(display));
    }

    if let Some(border) = &props.border {
        classes.push(Vec::from(border));
    }

    // if let Some(p) = &props.p {
    //     classes.push(class_from_p(p.to_string()));
    // }

    // match &props.p {
    //     Some(p) => classes.push(class_from_p(p)),
    //     _ => (),
    // }

    html! {
        <@{props.is.clone()}
            ref={props.inner_ref.clone()}
            class={classes}
        >
            { for props.children.iter() }
        </@>
    }
}
