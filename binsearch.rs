fn bin_search(arr: Vec<i32>, target: i32) -> i32 {
    let mut left: usize = 0 as usize;
    let mut right: usize = (arr.len() - 1) as usize;

    while left <= right {
        let mid: usize = left + (right - left) / 2;

        if arr[mid] == target {
            return mid as i32;
        }

        if target < arr[mid] {    
            right = mid - 1;
        }

        if target > arr[mid] {
            left = mid + 1;
        }
    }

    return -1;
}

fn main() {
    println!("result {}", bin_search(vec![1, 2, 3, 6, 7], 3));
}