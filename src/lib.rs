pub mod components;

mod algorithms;
mod helpers;

use helpers::generate_random_array;
use leptos::{create_rw_signal, RwSignal, Scope};

#[derive(Debug, Clone, Copy)]
pub enum Algorithm {
    Quicksort,
    Bubblesort,
}

#[derive(Debug, Clone)]
pub struct State {
    array: RwSignal<Vec<usize>>,
    algorithm: Algorithm,
    delay: u32,
}

impl State {
    pub fn new(cx: Scope) -> Self {
        let array = create_rw_signal(cx, generate_random_array(50));

        State {
            array,
            algorithm: Algorithm::Quicksort,
            delay: 50,
        }
    }
}
