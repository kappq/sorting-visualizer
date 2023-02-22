use leptos::*;

use crate::components::*;
use crate::State;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let state = create_rw_signal(cx, State::new(cx));

    view! {
        cx,
        <div id="app">
            <Sidebar state />
            <Visualizer state />
        </div>
    }
}
