pub mod components;

mod algorithms;
mod helpers;

#[derive(Debug, Clone, Copy)]
pub enum Algorithm {
    Quicksort,
    Bubblesort,
}

#[derive(Debug, Clone, Copy)]
pub struct State {
    algorithm: Algorithm,
    array_size: usize,
    delay: u32,
}

impl Default for State {
    fn default() -> Self {
        Self {
            algorithm: Algorithm::Quicksort,
            array_size: 50,
            delay: 50,
        }
    }
}
