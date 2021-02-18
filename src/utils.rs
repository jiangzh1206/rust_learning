
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


// 数组中两种数（a, b）出现奇数次
pub fn print_oddnums(arr: &[i32])->(i32,i32){
    let mut eor = 0;
    for v in arr{
        eor = eor ^ v;
    }

    // eor 等于 a^b
    // 取eor2进制最右侧的1
    let right_one = eor & (!eor + 1);

    let mut a = 0;
    for v in arr {
        if v & right_one != 0{
            a = a ^ v;
        }
    }
    (a, eor ^ a)

}