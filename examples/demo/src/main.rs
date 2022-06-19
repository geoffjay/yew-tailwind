extern crate yew;

use yew::{function_component, html, props};
use yew_tailwind::primitives::Box;
use yew_tailwind::tailwind::DisplayProps;

#[function_component(Boxes)]
fn boxes() -> Html {
    let box_demo_1 = r#"
        <Box>
        </Box>
    "#;

    let display = props! {
        DisplayProps {
            inline: true,
            block: true,
        }
    };

    let hidden = props!{
        DisplayProps { hidden: true }
    };

    html! {
        <>
            <Box display={display}>
                <code>{box_demo_1}</code>
            </Box>
            <Box>
                <Box is={"div"}>
                </Box>
                <Box is={"span"} display={hidden}>
                </Box>
            </Box>
        </>
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <Boxes />
    }
}

pub fn main() {
    yew::start_app::<App>();
}
