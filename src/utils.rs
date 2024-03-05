use rand::Rng;

pub fn permute_array(array: &mut [usize]) {
    let mut rng = rand::thread_rng();
    // Generate a random integer
    let len = (&array).len();
    let mut initial_range: usize = len;
    let mut tmp: usize;
    let mut exchange_index: usize;
    for _i in 0..array.len() {
        exchange_index = rng.gen_range(0..initial_range);
        initial_range -= 1;
        tmp = array[initial_range];
        array[initial_range] = array[exchange_index];
        array[exchange_index] = tmp;
    }
}

pub fn print_array(array: &[usize]) {
    for i in 0..array.len() {
        print!("{} ", array[i]);
    }
}
