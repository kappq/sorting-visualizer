use async_recursion::async_recursion;
use gloo_timers::future::TimeoutFuture;
use leptos::RwSignal;

async fn partition(array: RwSignal<Vec<usize>>, delay: u32, start: usize, end: usize) -> usize {
    let pivot = array.with(move |array| array[end]);
    let mut i = start;

    for j in start..end {
        if array.with(|array| array[j] < pivot) {
            if i != j {
                TimeoutFuture::new(delay).await;
                array.update(|array| array.swap(i, j));
            }
            i += 1;
        }
    }

    TimeoutFuture::new(delay).await;
    array.update(|array| array.swap(i, end));
    i
}

#[async_recursion(?Send)]
pub async fn quicksort(array: RwSignal<Vec<usize>>, delay: u32, start: usize, end: usize) {
    if start >= end {
        return;
    }

    let pivot_index = partition(array, delay, start, end).await;
    quicksort(array, delay, start, pivot_index - 1).await;
    quicksort(array, delay, pivot_index + 1, end).await;
}

pub async fn bubblesort(array: RwSignal<Vec<usize>>, delay: u32) {
    for i in 0..array.with(|array| array.len()) {
        for j in 0..(array.with(|array| array.len()) - i - 1) {
            if array.with(|array| array[j] > array[j + 1]) {
                TimeoutFuture::new(delay).await;
                array.update(|array| array.swap(j, j + 1));
            }
        }
    }
}
