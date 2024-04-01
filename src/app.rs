use std::sync::{Arc, Mutex, RwLock};

use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use shared::Issue;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use log::info;
use yew_hooks::use_effect_once;


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs {
    name: String
}

#[derive(Serialize, Deserialize)]
struct AddIssueArgs {
    name: String,
    summary: String,
}


#[derive(Serialize, Deserialize)]
struct EmptyArgs {
}

#[derive(Properties, PartialEq)]
pub struct CardProps {
    pub issue: Issue
}

#[function_component]
fn Card(props: &CardProps) -> Html {
    let key = props.issue.name.clone();
    html! {             
        <div id={ *key } class="card">
            <div class="">
                <h4><b>{"Key: "} { &*props.issue.name }  </b></h4>
                <p>{"Summary: "} { &*props.issue.summary } </p>
            </div>
        </div>  
    }
}

fn get_issues(issues: UseStateHandle<Box<Vec<Issue>>>, error: UseStateHandle<String>) {
    let issues = issues.clone();
    let error = error.clone();
    spawn_local(async move {
        let issues = issues.clone();
        let get_value = invoke("get_issues", to_value(&EmptyArgs{}).unwrap()).await;
        let val: Vec<Issue> = match serde_wasm_bindgen::from_value(get_value) {
            Ok(v) => v,
            Err(err) => { error.set(err.to_string());  Vec::new()},
        };
        issues.set(Box::new(val));
    });

}

#[function_component(App)]
pub fn app() -> Html {
    wasm_logger::init(wasm_logger::Config::default());

    let name_input_ref = use_node_ref();
    let summary_input_ref = use_node_ref();

    let name = use_state(|| String::new());
    let summary = use_state(|| String::new());
    let error = use_state(|| String::new());

    let issues = use_state(|| Box::new(Vec::new()));

    {
        let issues2 = issues.clone();
        let error2 = error.clone();
        use_effect_once(move || {
            get_issues(issues2, error2);

            || info!("asa")
        });
    }    

    // let issues = use_state(|| Box::new(Vec::new()));

    {
        let name = name.clone();
        let summary = summary.clone();
        let issues = issues.clone();
        let error = error.clone();
        let key2 = name.clone();
        use_effect_with(key2,
            move |_| {
                spawn_local(async move {
                    if name.is_empty() {
                        return;
                    }

                    let args = to_value(&AddIssueArgs { name: name.to_string(), summary: summary.to_string() }).unwrap();
                    info!("Hello2: {}", name.as_str());
                    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
                    invoke("add_issue", args).await;

                    get_issues(issues, error);
                });
                || {}
            },
        );
    }

    let get_issue = {
        let name = name.clone();
        let summary = summary.clone();
        let name_input_ref = name_input_ref.clone();
        let summary_input_ref = summary_input_ref.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            name.set(
                name_input_ref
                    .cast::<web_sys::HtmlInputElement>()
                    .unwrap()
                    .value(),
            );
            summary.set(
                summary_input_ref
                    .cast::<web_sys::HtmlInputElement>()
                    .unwrap()
                    .value(),
            );
        })
    };

    html! {
        <main class="container">
            <form class="row" onsubmit={get_issue}>
                <input id="greet-input" ref={name_input_ref} placeholder="Enter a key..." />
                <input id="greet-input" ref={summary_input_ref} placeholder="Enter a key..." />
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
