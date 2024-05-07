use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct AddIssueProps {
    pub add_issue: Callback<(String, String)>,
}

#[function_component(AddIssueForm)]
pub fn add_issue_form(props: &AddIssueProps) -> Html {
    let add_issue = props.add_issue.clone();
    let name_input_ref = use_node_ref();
    let summary_input_ref = use_node_ref();

    let on_summit = {
        let name_input_ref = name_input_ref.clone();
        let summary_input_ref = summary_input_ref.clone();
        move |e: SubmitEvent| {
            e.prevent_default();
            let name = name_input_ref
                .cast::<web_sys::HtmlInputElement>()
                .unwrap()
                .value();
            let summary = summary_input_ref
                .cast::<web_sys::HtmlInputElement>()
                .unwrap()
                .value();
            add_issue.emit((name, summary));
        }
    };

    html! {
        <div>
            <form class="column wrap-flex" onsubmit={on_summit}>
                <input id="greet-input" ref={name_input_ref} placeholder="Enter a key..." />
                <input id="greet-input" ref={summary_input_ref} placeholder="Enter summary..." />
            <button type="submit">{"Add"}</button>
            </form>
        </div>
    }
}
