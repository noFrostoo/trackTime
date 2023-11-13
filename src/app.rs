use std::fmt::Display;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use log::info;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs {
    key: String
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct JiraIssue {
    id: String,
    key: String,
    url: String,
    summary: String,
    assignee_email: String,
    time_tracked_all: i64,
}

impl JiraIssue {
    fn new(id: String, key: String, url: String, summary: String, assignee_email: String, time_tracked_all: i64) -> Self { Self { id, key, url, summary, assignee_email, time_tracked_all } }
    fn empty() -> Self { 
        Self { 
            id: String::new(), 
            key: String::new(), 
            url: String::new(), 
            summary: String::new(), 
            assignee_email: String::new(), 
            time_tracked_all: 0 } 
    }
}

impl Display for JiraIssue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}: {})", self.key, self.summary)
    }
}


#[function_component(App)]
pub fn app() -> Html {
    wasm_logger::init(wasm_logger::Config::default());

    let get_issue_input_ref = use_node_ref();

    let key = use_state(|| String::new());
    let error = use_state(|| String::new());

    let issue_value = use_state(|| JiraIssue::empty());
    {
        let issue_value = issue_value.clone();
        let key = key.clone();
        let key2 = key.clone();
        use_effect_with(key2,
            move |_| {
                spawn_local(async move {
                    if key.is_empty() {
                        return;
                    }

                    let args = to_value(&GreetArgs { key: key.to_string() }).unwrap();
                    info!("Hello2: {}", key.as_str());
                    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
                    let new_msg = invoke("get_issue", args).await;
                    info!("Hello3: {:?}", new_msg);
                    let val: JiraIssue = match serde_wasm_bindgen::from_value(new_msg) {
                        Ok(v) => v,
                        Err(err) => { error.set(err.to_string());  JiraIssue::empty()},
                    };
                    info!("Hello3: {:?}", val);
                    issue_value.set(val);
                });

                || {}
            },
        );
    }

    let get_issue = {
        let key = key.clone();
        let get_issue_input_ref = get_issue_input_ref.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            key.set(
                get_issue_input_ref
                    .cast::<web_sys::HtmlInputElement>()
                    .unwrap()
                    .value(),
            );
        })
    };

    html! {
        <main class="container">
            <p>{"Click on the Tauri and Yew logos to learn more."}</p>

            <form class="row" onsubmit={get_issue}>
                <input id="greet-input" ref={get_issue_input_ref} placeholder="Enter a name..." />
                <button type="submit">{"Greet"}</button>
            </form>
            <div class="card">
                <div class="">
                    <h4><b>{"Key"} { &*issue_value. } </b></h4>
                    <p>{"Summary"} </p>
                </div>
            </div> 


        </main>
    }
}
