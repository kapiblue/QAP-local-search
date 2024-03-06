use num_traits::Num;
use rand::Rng;

pub fn permute_array<R: Rng + ?Sized, T>(rng: &mut R, array: &mut [T])
where
    T: Copy,
{
    // Generate a random integer
    let len = (&array).len();
    let mut initial_range: usize = len;
    let mut tmp: T;
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

pub fn arange<T: Num>(array: &mut [T], low: T, step: T)
where
    T: Copy,
{
    let mut current: T = low;
    for i in 0..array.len() {
        array[i] = current;
        current = current + step;
    }
}

// print a generic type array
pub fn print_array<T>(array: &[T])
where
    T: std::fmt::Display,
{
    for i in 0..array.len() {
        print!("{} ", array[i]);
    }
}
