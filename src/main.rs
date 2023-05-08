fn main() {
    let mut arr = vec![4, 1, 8, 2, 9];
    let max_arr = selection_sort(&mut arr);
    println!("{:?}", max_arr);
}

fn selection_sort(arr: &mut Vec<i32>) -> Vec<i32> {
    let mut new_vec = Vec::new();
    for _ in 0..arr.len() {
        if let Some(val) = max(arr) {
            new_vec.push(val);
        }
    }
    new_vec
}

fn max(arr: &mut Vec<i32>) -> Option<i32> {
    for i in 0..arr.len() - 1 {
        if arr[i + 1] < arr[i] {
            arr.swap(i, i + 1);
        }
    }
    arr.pop()
}
