use leptos::*;

use crate::State;

fn calculate_new_height(height: f32, max_height: f32) -> f32 {
    height * (100.0 / max_height)
}

#[component]
pub fn Bar(cx: Scope, height: usize, state: RwSignal<State>) -> impl IntoView {
    let bar_ref = NodeRef::<HtmlElement<Div>>::new(cx);
    create_effect(cx, move |_| {
        if let Some(element) = bar_ref.get() {
            let new_height =
                calculate_new_height(height as f32, state.with(|state| state.array_size) as f32);
            let style = format!("height: {new_height}%;");
            element.prop("style", style);
        }
    });

    let new_height =
        calculate_new_height(height as f32, state.with(|state| state.array_size) as f32);
    let style = format!("height: {new_height}%;");
    view! {
        cx,
        <div _ref=bar_ref class="bar" prop:style=style></div>
    }
}
