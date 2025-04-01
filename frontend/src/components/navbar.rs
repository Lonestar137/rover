use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct NavbarProps {}

pub struct NavbarComponent {
    // props: NavbarProps,
}

impl Component for NavbarComponent {
    type Message = ();
    type Properties = NavbarProps;

    fn create(_ctx: &Context<Self>) -> Self {
        NavbarComponent {
            // props: ctx.props().clone(),
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <nav class="bg-white flex shadow-xl fixed top-0 left-0 right-0 z-50">
                <ul class="flex space-x-4">
                    <li><span class="font-medium text-2xl pr-4">{"Item"}</span></li>
                    <li><span class="font-medium text-2xl pr-4">{"Item2"}</span></li>
                </ul>
            </nav>
        }
    }
}
