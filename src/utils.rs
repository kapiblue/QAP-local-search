use rand::Rng;

pub fn permute_array<R: Rng + ?Sized>(rng: &mut R, array: &mut [u32]) {
    // Generate a random integer
    let len = (&array).len();
    let mut initial_range: usize = len;
    let mut tmp: u32;
    let mut exchange_index: usize;
    for _i in 0..array.len() {
        exchange_index = rng.gen_range(0..initial_range);
        initial_range -= 1;
        tmp = array[initial_range];
        array[initial_range] = array[exchange_index];
        array[exchange_index] = tmp;
    }
}

pub fn generate_random_int_pair<R: Rng + ?Sized>(rng: &mut R, range: u32) -> (u32, u32) {
    let x1: u32 = rng.gen_range(0..range);
    let x2: u32 = (rng.gen_range(0..range - 1) + 1 + x1) % range;

    return (x1, x2);
}

pub fn arange(array: &mut [u32], low: u32, step: u32) {
    let mut current:u32 = low;
    for i in 0..array.len() {
        array[i] = current;
        current += step;
    }
}

pub fn print_array(array: &[u32]) {
    for i in 0..array.len() {
        print!("{} ", array[i]);
    }
}
