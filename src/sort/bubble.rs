#![allow(dead_code)]

// 坏O(n²)， 好O(n)
pub fn bubble_sort(arr: &mut [i32]) {
    //0..i =>[0,i)
    for i in (0..arr.len()).rev() {
        let mut flag = true;
        for j in 0..i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                flag = false;
            }
        }

        if flag == true {
            break;
        }

        //println!("{:?}", arr);
    }
}
