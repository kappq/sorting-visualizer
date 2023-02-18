use leptos::*;

use crate::State;

#[component]
pub fn Bar(cx: Scope, height: usize, state: RwSignal<State>) -> impl IntoView {
    let bar_ref = NodeRef::<HtmlElement<Div>>::new(cx);
    create_effect(cx, move |_| {
        if let Some(element) = bar_ref.get() {
            let max_height = state.with(|state| state.array_size);
            let new_height = height as f32 * (100_f32 / max_height as f32);
            let style = format!("height: {new_height}%;");
            element.prop("style", style);
        }
    });

    let max_height = state.with(|state| state.array_size);
    let new_height = height as f32 * (100_f32 / max_height as f32);
    let style = format!("height: {new_height}%;");
    view! {
        cx,
        <div _ref=bar_ref class="bar" prop:style=style></div>
    }
}
