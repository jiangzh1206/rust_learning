
#![allow(dead_code)]

// 坏O(n²)， 好O(n)， 空间O(1)
pub fn insert_sort(arr: &mut [i32]) {
    for i in 1..arr.len() {
        for j in (1..=i).rev() {
            if arr[j-1] < arr[j] {
                break;
            }
            if arr[j - 1] > arr[j] {
                arr.swap(j - 1, j);
            }
        }

        //println!("{:?}", arr);
    }
}


pub fn insert_sort1(arr: &mut [i32]) {
    for i in 1..arr.len() {
        let val = arr[i];
        let mut pos:i32 = -1;
        for j in (1..=i).rev() {
            if arr[j-1] < val {
                break;
            }
            if arr[j - 1] > val {
                arr[j]=arr[j-1];
                pos=j as i32 - 1;
            }
        }
        if pos >=0 {
            arr[pos as usize]=val;
        }
        println!("{:?}", arr);
    }
}