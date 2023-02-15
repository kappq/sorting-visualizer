use rand::seq::SliceRandom;

pub fn generate_random_array(size: usize) -> Vec<usize> {
    let mut rng = rand::thread_rng();
    let mut values = (1..=size).collect::<Vec<usize>>();
    values.shuffle(&mut rng);
    values.to_vec()
}
