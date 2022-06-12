extern crate yew;

use yew::{function_component, html};
use yew_tailwind::primitives::Box;

#[function_component(App)]
fn app() -> Html {
    html! {
        <Box>
            <Box is={"div"}>
                <Box is={"span"}>
                </Box>
            </Box>
        </Box>
    }
}

pub fn main() {
    yew::start_app::<App>();
}
