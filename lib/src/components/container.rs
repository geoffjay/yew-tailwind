use yew::{function_component, html, props, Children, Classes, Properties};

use crate::primitives::{Box, BoxProps};
use crate::tailwind::{BorderProps, DisplayProps};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ContainerProps {
    #[prop_or("div".to_string())]
    pub is: String,
    #[prop_or_default]
    pub classes: Option<Classes>,
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub border: Option<BorderProps>,
    #[prop_or_default]
    pub display: Option<DisplayProps>,
}

#[function_component(Container)]
pub fn container(props: &ContainerProps) -> Html {
    let mut classes = Classes::from("container");
    classes.push(&props.classes);

    let border = props.border.as_ref().cloned();
    let display = props.display.as_ref().cloned();

    let box_props = props! {
        BoxProps {
            is: props.is.clone(),
            classes: classes,
            border: border,
            display: display,
        }
    };

    html! {
        <Box ..box_props>
            { for props.children.iter() }
        </Box>
    }
}
