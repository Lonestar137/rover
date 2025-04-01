mod components;
use std::fs;
use std::path::{Path, PathBuf};

use crate::components::directory_card::DirectoryCard;
use crate::components::navbar::NavbarComponent;

// use serde::{Deserialize, Serialize};
// use gloo_net::http::Request;
use yew::prelude::*;
use yew_router::prelude::*;

// #[derive(Debug, Serialize, Deserialize)]
// struct Application {
//     application: Data,
// }

// #[derive(Debug, Serialize, Deserialize)]
// struct Data {
//     workdir: String,
// }

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    // #[at("/stock/:id")]
    // Stock { id: String },
    // #[at("/about")]
    // About,
    // #[not_found]
    // #[at("/404")]
    // NotFound,
    // #[at("/dashboard")]
    // Dashboard,
}

#[function_component(Home)]
fn home() -> Html {
    // let subdirectories = use_state(|| vec![]);

    // {
    //     let subdirectories = subdirectories.clone();
    //     use_effect(move || {
    //         wasm_bindgen_futures::spawn_local(async move {
    //             let response = Request::get("http://127.0.0.1:8080/list")
    //                 .send()
    //                 .await
    //                 .unwrap();
    //             let dirs: Vec<String> = response.json().await.unwrap();
    //             subdirectories.set(dirs);
    //         });
    //         || ()
    //     });
    // }

    html! {
        <div>
            // {for subdirectories.iter().map(|path| html! {
            //     <DirectoryCard path={path.clone()} />
            // })}
        </div>
    }
}

// Main app component
#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

// Switch function to map routes to components
fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
    }
}

fn main() {
    // let contents = fs::read_to_string("config.yaml").unwrap();
    // let config: Application = serde_yaml::from_str(&contents).unwrap();
    // println!("{:#?}", config);

    yew::Renderer::<App>::new().render();
}
