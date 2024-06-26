use std::time::Duration;

use crate::add_issue_form::AddIssueForm;
use crate::card::Card;
use crate::tracking_card::TracingCard;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use shared::Issue;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_hooks::{use_effect_once, use_interval};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct AddIssueArgs {
    name: String,
    summary: String,
}

#[derive(Serialize, Deserialize)]
struct EmptyArgs {}

#[derive(Serialize, Deserialize)]
pub struct StartTrackingProps {
    name: String,
}

fn get_issues(issues: UseStateHandle<Box<Vec<Issue>>>, error: UseStateHandle<String>) {
    let issues = issues.clone();
    let error = error.clone();
    spawn_local(async move {
        let issues = issues.clone();
        let get_value = invoke("get_issues", to_value(&EmptyArgs {}).unwrap()).await;
        let val: Vec<Issue> = match serde_wasm_bindgen::from_value(get_value) {
            Ok(v) => v,
            Err(err) => {
                error.set(err.to_string());
                Vec::new()
            }
        };
        issues.set(Box::new(val));
    });
}

fn get_tracing_info(tracking_issue: UseStateHandle<Option<String>>, error: UseStateHandle<String>, elapsed_time: UseStateHandle<Duration>) {
    spawn_local(async move {
        let args = to_value(&EmptyArgs {}).unwrap();
        let get_value = invoke("get_elapsed_time", args).await;

        let val: Duration = match serde_wasm_bindgen::from_value(get_value) {
            Ok(v) => v,
            Err(err) => {
                error.set(err.to_string());
                Duration::from_micros(0)
            }
        };
        elapsed_time.set(val);

        let args = to_value(&EmptyArgs {}).unwrap();
        let get_value = invoke("get_tracing_issue_name", args).await;
        let val: Option<String> = match serde_wasm_bindgen::from_value(get_value) {
            Ok(v) => v,
            Err(err) => {
                error.set(err.to_string());
                Some(err.to_string())
            }
        };
        tracking_issue.set(val)
    });
}

#[function_component(App)]
pub fn app() -> Html {
    wasm_logger::init(wasm_logger::Config::default());

    let name = use_state(|| String::new());
    let summary = use_state(|| String::new());
    let error = use_state(|| String::new());

    let issues = use_state(|| Box::new(Vec::new()));
    let tracking_issue: UseStateHandle<Option<String>> = use_state(|| None);
    let elapsed_time = use_state(|| Duration::from_micros(0));

    {
        let issues2 = issues.clone();
        let error2 = error.clone();
        use_effect_once(move || {
            get_issues(issues2, error2);
            || () // cleaning function
        });
    }

    {
        let tracking_issue = tracking_issue.clone();
        let error = error.clone();
        let elapsed_time = elapsed_time.clone();
        use_interval(
            move || {
                get_tracing_info(tracking_issue.clone(), error.clone(), elapsed_time.clone());
            },
            999,
        );
    }

    let start_tracking = {
        let tracking_issue = tracking_issue.clone();
        Callback::from(move |name: Box<String>| {
            let tracking_issue = tracking_issue.clone();
            spawn_local(async move {
                let args = to_value(&StartTrackingProps {
                    name: *name.clone(),
                })
                .unwrap();

                invoke("start_tracking_cmd", args).await;
                tracking_issue.set(Some(*name.clone()));
            });
        })
    };

    let stop_tracking = {
        let issues = issues.clone();
        let error = error.clone();
        let tracking_issue = tracking_issue.clone();
        Callback::from(move |()| {
            let tracking_issue = tracking_issue.clone();
            let issues = issues.clone();
            let error = error.clone();
            spawn_local(async move {
                let args = to_value(&EmptyArgs{}).unwrap();

                invoke("stop_tracking_cmd", args).await;
                tracking_issue.set(None);
                get_issues(issues, error);
            });
        })
    };

    {
        let name = name.clone();
        let summary = summary.clone();
        let issues = issues.clone();
        let error = error.clone();
        let name_copy = name.clone(); // clone to solved moved error
        use_effect_with(name_copy, move |_| {
            spawn_local(async move {
                if name.is_empty() {
                    return;
                }

                let args = to_value(&AddIssueArgs {
                    name: name.to_string(),
                    summary: summary.to_string(),
                })
                .unwrap();

                invoke("add_issue", args).await;
                get_issues(issues, error);
            });
            || {}
        });
    }

    let add_issue = {
        let name = name.clone();
        let summary = summary.clone();
        Callback::from(move |data: (String, String)| {
            name.set(data.0);
            summary.set(data.1);
        })
    };

    html! {
        <main class="container">
            <div class = "column column-25 wrap-flex">
                <div class = "row wrap-flex">
                    <AddIssueForm add_issue={add_issue}/>
                </div>
                if tracking_issue.is_some() {
                    <TracingCard name={tracking_issue.as_ref().unwrap().clone()} duration={*elapsed_time} stop_tracking={stop_tracking.clone()} />
                }


            </div>
            <div class="divider-vertical"></div>
            <div class = "column column-75">
                <div class = "row wrap-flex">
                    {
                        issues.iter().map(|issue| {
                            html!{<Card issue={issue} start_tracking={start_tracking.clone()} />}
                        }).collect::<Html>()
                    }
                </div>
            </div>



        </main>
    }
}
