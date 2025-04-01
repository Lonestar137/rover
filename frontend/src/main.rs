mod components;
use std::fs;
use std::path::{Path, PathBuf};

use crate::components::directory_card::DirectoryCard;
use crate::components::navbar::NavbarComponent;

// use serde::{Deserialize, Serialize};
use gloo_net::http::Request;
use web_sys;
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

// TODO: this function is sending a bajillion requests a second, fixme
#[function_component(Home)]
fn home() -> Html {
    // State to store the list of subdirectories
    let subdirectories = use_state(|| vec![]);
    // State to store any error messages
    let error_message = use_state(|| None);

    {
        let subdirectories = subdirectories.clone();
        let error_message = error_message.clone();

        // Fetch the list of subdirectories when the component is mounted
        use_effect(move || {
            wasm_bindgen_futures::spawn_local(async move {
                // Make the API request
                match Request::get("http://127.0.0.1:8088/list").send().await {
                    Ok(response) => {
                        // Parse the JSON response into a vector of strings
                        match response.json::<Vec<String>>().await {
                            Ok(dirs) => {
                                web_sys::console::log_1(
                                    &format!("Fetched directories: {:?}", dirs).into(),
                                );
                                subdirectories.set(dirs); // Update state with fetched data
                            }
                            Err(err) => {
                                web_sys::console::log_1(
                                    &format!("Failed to parse JSON: {:?}", err).into(),
                                );
                                error_message
                                    .set(Some(format!("Failed to parse response: {:?}", err)));
                            }
                        }
                    }
                    Err(err) => {
                        web_sys::console::log_1(&format!("Failed to fetch data: {:?}", err).into());
                        error_message.set(Some(format!("Failed to fetch data: {:?}", err)));
                    }
                }
            });
            || ()
        });
    }

    html! {
        <div>
            <p>{"TEST"}</p>
            // Display an error message if one exists
            {if let Some(error) = (*error_message).clone() {
                html! { <p style="color: red;">{error}</p> }
            } else {
                html! {}
            }}
            // Render DirectoryCard components for each subdirectory
            {for subdirectories.iter().map(|path| html! {
                <DirectoryCard path={path.clone()} />
            })}
            <p>{"TEST"}</p>
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
