
fn binary_search(arr: &[i32], key: i32) -> Option<usize>{
    let mut left = 0;
    let mut right = arr.len() - 1;

    while left <= right {
        let mid = left + (right - left) / 2;
        if arr[mid] == key {
            return Some(mid);
        }
        else if arr[mid] < key {
            left = mid + 1;
        }
        else{
            right = mid - 1;
        }
    }
    return None;
}


fn main(){
    let arr = [1,3,5,7,9,11,13];
    let key = 7;
    match binary_search(&arr, key){
        Some(index) => println!("Key found at index {}", index),
        None => println!("Key is not found"),
    }

}