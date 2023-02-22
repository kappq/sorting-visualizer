use leptos::*;

fn calculate_height(height: usize, max_height: usize) -> f32 {
    height as f32 * (100.0 / max_height as f32)
}

#[component]
pub fn Bar(cx: Scope, height: usize, array: RwSignal<Vec<usize>>) -> impl IntoView {
    let bar_ref = NodeRef::<HtmlElement<Div>>::new(cx);
    create_effect(cx, move |_| {
        if let Some(element) = bar_ref.get() {
            let max_height = array.with(|array| array.len());
            let style = format!("height: {}%;", calculate_height(height, max_height));
            element.prop("style", style);
        }
    });

    let max_height = array.with(|array| array.len());
    let style = format!("height: {}%;", calculate_height(height, max_height));
    view! {
        cx,
        <div _ref=bar_ref class="bar" prop:style=style></div>
    }
}
