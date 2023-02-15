use leptos::*;

use crate::components::*;
use crate::helpers::generate_random_array;
use crate::State;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let state = create_rw_signal(cx, State::default());

    let array_size = state.with(|state| state.array_size);
    let array = create_rw_signal(cx, generate_random_array(array_size));

    view! {
        cx,
        <div id="app">
            <Sidebar state array />
            <Visualizer state array />
        </div>
    }
}
