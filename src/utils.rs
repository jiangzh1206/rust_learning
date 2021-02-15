
extern crate rand;
use rand::Rng;


pub fn rand_array(arr: &mut [i32]) {
    let mut rng = rand::thread_rng();
    for i in 0..arr.len() {
        arr[i] = rng.gen_range(1..20);
    }

    
}

pub fn print_array(arr: &[i32]) {
    for v in arr {
        print!("{}  ", v);
    }
    println!("")
}