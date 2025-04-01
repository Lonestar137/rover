use yew::prelude::*;

#[derive(Clone, PartialEq, Properties, Default)]
pub struct DirectoryCardProps {
    pub path: String,
    #[prop_or_default]
    pub thumbnails: Vec<String>,
    #[prop_or_default]
    pub directory_size: String,
}

pub struct DirectoryCard {
    props: DirectoryCardProps,
}

impl Component for DirectoryCard {
    type Message = ();
    type Properties = DirectoryCardProps;

    fn create(ctx: &Context<Self>) -> Self {
        let dir_path = ctx.props().path.to_string();
        DirectoryCard {
            props: DirectoryCardProps {
                path: dir_path,
                ..Default::default()
            },
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <p>{&self.props.path}</p>
            </div>
        }
        // let change_color = if self.props.change_percent > 0.0 {
        //     "green"
        // } else {
        //     "red"
        // };

        // html! {
        //     <div >
        //         <div>
        //             <div class="bg-white-100 shadow size-24">
        //                 <div>
        //                     <h2>{ &self.props.stock_name }</h2>
        //                     <p>{ format!("${:.2}", self.props.stock_price) }</p>
        //                     <p style={format!("color: {}", change_color)}>
        //                         { format!("{:.2}%", self.props.change_percent) }
        //                     </p>
        //                 </div>
        //             </div>
        //         </div>
        //     </div>
        // }
    }
}
