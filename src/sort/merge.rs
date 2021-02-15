#![allow(dead_code)]

pub fn merge_sort(arr: &mut [i32], left: usize, right: usize) {
    if left == right {
        return;
    }
    
    let mid = left + (right-left)/2;
    // 左边排序
    merge_sort(arr, left, mid);

    // 右边排序
    merge_sort(arr, mid+1, right);

    // 合并
    merge(arr, left, mid, right);
}

// left----mid------right
fn merge(arr: &mut [i32], left: usize, mid: usize, right: usize) {
    let mut temp = vec![];
    let mut l = left;
    let mut r = mid + 1;

    while l <= mid && r <= right {
        if arr[l] <= arr[r] {
            temp.push(arr[l]);
            l += 1;
        } else {
            temp.push(arr[r]);
            r += 1;
        }
    }

    for i in l..=mid {
        temp.push(arr[i]);
    }
    for i in r..=right {
        temp.push(arr[i]);
    }

    let mut index = left;
    for v in temp {
        arr[index] = v;
        index += 1;
    }

    //println!("{:?}", arr);
}
