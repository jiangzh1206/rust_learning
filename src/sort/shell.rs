//extern crate itertools;
//use itertools::Itertools;

// 改进的插入排序,按间隔h进行插入排序
// Knuth序列：h=h*3+1;
// 坏O(n²)， 好O(n的1.3次方)， 空间O(1)
pub fn shell_sort(arr: &mut [i32]) {
    let mut h = 1;
    while h <= arr.len() / 3 {
        h = h * 3 + 1;
    }

    while h > 0 {
        for i in h..arr.len() {
            for j in (h..=i).rev().step_by(h) {
                if arr[j - h] < arr[j] {
                    break;
                }
                if arr[j - h] > arr[j] {
                    arr.swap(j - h, j);
                }
            }
            //println!("{:?}", arr);
        }
        h = (h - 1) / 3;
    }
}
