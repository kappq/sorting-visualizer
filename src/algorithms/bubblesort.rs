use gloo_timers::future::TimeoutFuture;
use leptos::RwSignal;

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
