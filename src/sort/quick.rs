pub fn quick_sort(arr: &mut [i32]) {
    sort(arr, 0, arr.len() - 1);
}

fn sort(arr: &mut [i32], left: usize, right: usize) {
    if left < right {
        //println!("{:?}",arr);
        let pos = partion(arr, left, right);
        //println!("pos:{},l:{},r:{},{:?}",pos,left,right,arr);
        if pos > 0 {
            sort(arr, left, pos - 1);
        }
        sort(arr, pos + 1, right);
    }
}

pub fn test_partion() {
    let mut arr = [12,6,11,19,7,8,16,2,13,12];
    //let pos = partion(&mut arr, 0, 7);
    //println!("pos:{},{:?}", pos, arr);

    sort(&mut arr, 0, 9);
}

fn partion(arr: &mut [i32], left: usize, right: usize) -> usize {
    let mut l = left;
    let mut r = right;
    let pivot = arr[r];

    while l < r {
        // 单抽
        // note: arr[l]<=pivot
        while l < r && arr[l] <= pivot {
            l += 1;
        }
        if l < r {
            arr[r] = arr[l];
        }

        while l < r && arr[r] >= pivot {
            r -= 1;
        }

        if l < r {
            arr[l] = arr[r];
        }

        //println!("l:{},r:{},{:?}", l, r, arr);

    }
    
    arr[l] = pivot;
    l
}
