mod add_issue_form;
mod app;
mod card;
mod tracking_card;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
