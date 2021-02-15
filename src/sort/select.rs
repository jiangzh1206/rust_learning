#![allow(dead_code)]

//use crate::utils::print_array;

// 好坏都是O(n²)
pub fn select_sort(arr: &mut[i32]) {
    // crate::utils::print_array(arr);
    // print_array(arr);
    for i in 0..arr.len()-1 {
        let mut min_pos = i;
        // 最小值
        for j in i+1..arr.len() {
            if arr[j] < arr[min_pos] {
                min_pos = j;  
            }
        }
        arr.swap(min_pos, i);

        //print_array(arr);
    }
}



