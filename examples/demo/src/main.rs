extern crate yew;

use yew::{function_component, html, props};
use yew_tailwind::primitives::Box;
use yew_tailwind::components::Container;
use yew_tailwind::tailwind::{BorderProps, DisplayProps, RadiusOption};

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

    let hidden = props! {
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
    let border = props! {
        BorderProps {
            radius: RadiusOption::Medium,
        }
    };

    html! {
        <Container border={border}>
            <Boxes />
        </Container>
    }
}

pub fn main() {
    yew::start_app::<App>();
}
