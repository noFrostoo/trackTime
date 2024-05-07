use std::time::Duration;

use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TrackingCardProps {
    pub name: String,
    pub duration: Duration,
    pub stop_tracking: Callback<()>
}

#[function_component(TracingCard)]
pub fn tracking_card(props: &TrackingCardProps) -> Html {
    let name = props.name.clone();
    let stop_tracking = props.stop_tracking.clone();

    let on_clicked = {
        move |_: MouseEvent| {
            stop_tracking.emit(())
        }
    };

    let duration = props.duration.clone();
    let seconds = duration.as_secs() % 60;
    let minutes = (duration.as_secs() / 60) % 60;
    let hours = (duration.as_secs() / 60) / 60;

    html! {
        <div class="card" onclick={on_clicked} >
            <div class="">
                <h4><b>{"Name: "} { name }  </b></h4>
                <p>{"Time: "} {hours} {":"} {minutes} {":"} {seconds} </p>
            </div>
        </div>
    }
}
