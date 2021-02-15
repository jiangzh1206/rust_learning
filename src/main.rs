mod sort;
mod utils;

fn main() {
    let mut arr: [i32; 10] = [0; 10];
    utils::rand_array(&mut arr);

    utils::print_array(&arr);

    //sort::select::select_sort(&mut arr);
    //sort::bubble::bubble_sort(&mut arr);
    //sort::insert::insert_sort1(&mut arr);
    //sort::shell::shell_sort(&mut arr);

    let len = arr.len();
    sort::merge::merge_sort(&mut arr, 0, len - 1);

    utils::print_array(&arr);
}
