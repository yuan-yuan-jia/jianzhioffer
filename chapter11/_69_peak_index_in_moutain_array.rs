

fn peak_index_in_moutain_array(array: &[i32]) -> isize {
    let mut left = 0;
    let mut right = array.len() - 1;
    
    while left <= right {
        let mid = (left + right) >> 1;
        if mid != 0 && mid != array.len() - 1 {
            if array[mid] >= array[mid + 1] && array[mid] >= array[mid - 1] {
                return mid as isize;
            }else if array[mid] < array[mid + 1]{
                // 
                left = mid + 1;
            }else {
                right = mid - 1;
            }
        }else {
            return mid as isize;
        }
    }
    -1
}

/*Driver Code */
fn main() {
    let i = [1, 3, 5, 4, 2];
    let r = peak_index_in_moutain_array(&i[..]);
    println!("r:{}", r);
    assert_eq!(r, 2);
}