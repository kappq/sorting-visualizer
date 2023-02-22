use leptos::*;

use crate::components::*;
use crate::State;

#[component]
pub fn Visualizer(cx: Scope, state: RwSignal<State>) -> impl IntoView {
    let array = state.with(|state| state.array);
    view! {
        cx,
        <div id="visualizer">
            <For
                each=array
                key=|height| *height
                view=move |height| view! { cx, <Bar height array /> }
            />
        </div>
    }
}
