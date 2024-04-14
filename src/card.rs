use shared::Issue;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct CardProps {
    pub issue: Issue,
    pub start_tracking: Callback<Box<String>>,
}

#[function_component(Card)]
pub fn card(props: &CardProps) -> Html {
    let key = props.issue.name.clone();
    let start_tracking = props.start_tracking.clone();

    let on_clicked = {
        let key2 = key.clone();
        move |_: MouseEvent| {
            let key2 = key2.clone();
            start_tracking.emit(Box::new(key2))
        }
    };

    html! {
        <div id={ key } onclick={on_clicked} class="card">
            <div class="">
                <h4><b>{"Name: "} { &*props.issue.name }  </b></h4>
                <p>{"Summary: "} { &*props.issue.summary } </p>
            </div>
        </div>
    }
}
