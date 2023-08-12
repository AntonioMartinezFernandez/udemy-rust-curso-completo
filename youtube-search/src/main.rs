use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::HtmlInputElement;
// use web_sys::console;

use yew::prelude::*;

pub mod env;
pub mod youtube;
use crate::youtube::youtube_search;

fn main() {
    yew::Renderer::<App>::new().render();
}

/**********************
 * Main component
**********************/

/* STATE STRUCTURES */
#[derive(Clone)]
struct Video {
    id: String,
    name: String,
}

#[function_component]
fn App() -> Html {
    /* STATE */
    let video_state: UseStateHandle<Option<Video>> = use_state(|| None);

    /* CALLBACKS */

    // When the button of the SearchControls component is clicked, search text in YT,
    // and update the state with the video info
    let on_button_clicked_cb = {
        let video_state_clone = video_state.clone();

        Callback::from(move |search_text: String| {
            let video_state_clone = video_state_clone.clone();

            // To be able to use async functions (with 'await') inside a Callback, we need to wrap the code
            // in an async scope We do that with 'wasm_bindgen_futures' crate.
            wasm_bindgen_futures::spawn_local(async move {
                match youtube_search(search_text).await {
                    Ok(video_item) => video_state_clone.set(Some(Video {
                        id: video_item.id.video_id,
                        name: video_item.snippet.title,
                    })),
                    Err(err) => {
                        web_sys::console::log_1(&err.to_string().into());
                    }
                }
            });
        })
    };

    // Show Video Section will be showed only if we have state values
    let show_video_section = match (*video_state).clone() {
        Some(video) => html! {
            <ShowVideo id={video.id} name={video.name} />
        },
        None => html! {},
    };

    /* COMPONENT RENDERING */
    html! {
        <main>
            <SearchControls onbuttonclicked={on_button_clicked_cb} />
            {show_video_section}
        </main>
    }
}

/****************************
 * SearchControls component
****************************/

// Props
#[derive(Properties, PartialEq)]
struct SearchControlsProps {
    onbuttonclicked: Callback<String>, // this callback returns an String
}

// Component
#[function_component(SearchControls)]
fn search_controls(props: &SearchControlsProps) -> Html {
    /* STATE */
    let text_to_search: UseStateHandle<String> = use_state(|| String::new());

    /* CALLBACKS / CLOSURES */

    // Obtain the text from input event and update the state
    let on_search_input_cb = {
        let text_to_search_clone = text_to_search.clone();

        Callback::from(move |input_event: InputEvent| {
            // Obtain text input from InputEvent
            let text = get_value_from_input_event(input_event);

            // Using the web_sys crate, log the text in the browser console.
            // With the '.into()' method, we transform the 'text' variable in the necessary type (JsValue) variable
            // console::log_1(&text.into());

            text_to_search_clone.set(text);
        })
    };

    // Send the data saved in the state
    let onclick_cb = {
        let props_onbuttonclicked_clone = props.onbuttonclicked.clone();

        Callback::from(move |_| {
            props_onbuttonclicked_clone.emit(String::from(text_to_search.to_string()));
        })
    };

    /* COMPONENT RENDERING */
    html!(
    <>
        <div>
            {"What do you want to see?"}
        </div>
        <div>
            <input type="text" oninput={on_search_input_cb} />
        </div>
        <div>
            <button onclick={onclick_cb}>{"Search"}</button>
        </div>
    </>
    )
}

/************************
 * ShowVideo component
************************/

// Props
#[derive(Properties, PartialEq)]
struct ShowVideoProps {
    id: String,
    name: String,
}

// Component
#[function_component(ShowVideo)]
fn show_video(props: &ShowVideoProps) -> Html {
    let youtube_url = format!("https://www.youtube.com/embed/{}", props.id);

    /* COMPONENT RENDERING */
    html!(
        <div>
            <iframe width="560" height="315" src={youtube_url} title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share"></iframe>
        </div>
    )
}

/**********************
 * Auxiliar Methods
**********************/

fn get_value_from_input_event(e: InputEvent) -> String {
    let event: Event = e.dyn_into().unwrap_throw();
    let event_target = event.target().unwrap_throw();
    let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
    target.value()
}
