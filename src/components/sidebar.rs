use leptos::web_sys::Event;
use leptos::*;

use crate::algorithms::*;
use crate::helpers::generate_random_array;
use crate::{Algorithm, State};

#[component]
pub fn Sidebar(cx: Scope, state: RwSignal<State>, array: RwSignal<Vec<usize>>) -> impl IntoView {
    let set_algorithm = move |algorithm| state.update(|state| state.algorithm = algorithm);

    let on_array_size_change = move |ev: Event| {
        let new_size = event_target_value(&ev).parse().unwrap();
        state.update(|state| state.array_size = new_size);
        array.set(generate_random_array(new_size));
    };

    let on_delay_change = move |ev: Event| {
        let new_delay = event_target_value(&ev).parse().unwrap();
        state.update(|state| state.delay = new_delay);
    };

    let randomize_array = move |_| {
        let array_size = state.with(|state| state.array_size);
        array.set(generate_random_array(array_size));
    };

    let visualize = move |_| {
        let array_size = state.with(|state| state.array_size);
        let delay = state.with(|state| state.delay);
        match state.with(|state| state.algorithm) {
            Algorithm::Quicksort => spawn_local(quicksort(array, delay, 0, array_size - 1)),
            Algorithm::Bubblesort => spawn_local(bubblesort(array, delay)),
        };
    };

    view! {
        cx,
        <div id="sidebar">
            <label for="algorithm">"Algorithm"</label><br />
            <select id="algorithm">
                <option on:click=move |_| set_algorithm(Algorithm::Quicksort)>"QuickSort"</option>
                <option on:click=move |_| set_algorithm(Algorithm::Bubblesort)>"BubbleSort"</option>
            </select>
            <br />
            <label for="size">"Array Size"</label><br />
            <input id="size" type="range" min="10" max="150" value="50" on:change=on_array_size_change />
            <br />
            <label for="delay">"Delay"</label><br />
            <input id="delay" type="range" min="5" max="100" value="50" on:change=on_delay_change />
            <br />
            <button on:click=randomize_array>"Randomize Array"</button>
            <br />
            <button on:click=visualize>"Visualize!"</button>
        </div>
    }
}
