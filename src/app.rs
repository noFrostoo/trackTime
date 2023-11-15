use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use shared::JiraIssue;
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

#[derive(Properties, PartialEq)]
pub struct CardProps {
    pub issue: JiraIssue
}

#[function_component]
fn Card(props: &CardProps) -> Html {
    let key = props.issue.key.clone();
    html! {             
        <div id={ *key } class="card">
            <div class="">
                <h4><b>{"Key: "} { &*props.issue.key }  </b></h4>
                <p>{"Summary: "} { &*props.issue.summary } </p>
            </div>
        </div>  
    }
}

#[function_component(App)]
pub fn app() -> Html {
    wasm_logger::init(wasm_logger::Config::default());

    let get_issue_input_ref = use_node_ref();

    let key = use_state(|| String::new());
    let error = use_state(|| String::new());
    let issues = use_state(|| Box::new(Vec::new()));

    {
        let issues = issues.clone();
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

                    let val: JiraIssue = match serde_wasm_bindgen::from_value(new_msg) {
                        Ok(v) => v,
                        Err(err) => { error.set(err.to_string());  JiraIssue::empty()},
                    };


                    let mut vec = issues.as_ref().clone();
                    vec.push(val);
                    issues.set(Box::new(vec));
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
            <form class="row" onsubmit={get_issue}>
                <input id="greet-input" ref={get_issue_input_ref} placeholder="Enter a key..." />
                <button type="submit">{"Get"}</button>
            </form>
            <div class = "row">
                {
                    issues.iter().map(|issue| {
                        html!{<Card issue={issue}/>}
                    }).collect::<Html>()
                }
            </div>
            


        </main>
    }
}
