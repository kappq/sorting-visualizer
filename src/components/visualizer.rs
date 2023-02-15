use leptos::*;

use crate::components::*;
use crate::State;

#[component]
pub fn Visualizer(cx: Scope, state: RwSignal<State>, array: RwSignal<Vec<usize>>) -> impl IntoView {
    view! {
        cx,
        <div id="visualizer">
            <For
                each=array
                key=|height| *height
                view=move |height| view! { cx, <Bar height state /> }
            />
        </div>
    }
}
