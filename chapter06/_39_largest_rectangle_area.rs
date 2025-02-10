

fn largest_rectangle_area_violent_version(heights: Vec<i32>) -> i32 {
    let mut max_area = 0;
    
    for i in 0..heights.len() {
        let mut min = *heights.get(i).unwrap();
        let mut j = i;
        while j < heights.len() {
            min = min.min(*heights.get(j).unwrap());
            let mut area = min * (j - i + 1) as i32;
            max_area = max_area.max(area);
            j += 1;
        }
    }
    max_area
}

fn helper(heights: &[i32], start: usize, end: usize) -> i32 {
    if start == end {
        return 0;
    }

    if start + 1 == end {
        return *heights.get(start as usize).unwrap();
    }

    let mut min_index = start;
    let mut i = start + 1;
    while i < end {
        if heights[i as usize] < heights[min_index as usize] {
            min_index = i;
        }
        i += 1;
    }

    let area = (end - start) as i32 * heights[min_index as usize];
    let left = helper(heights, start, min_index);
    let right = helper(heights, min_index + 1, end);
    area.max(left).max(right)
}

fn largest_rectangle_area_divide_and_conquer_version(heights: Vec<i32>) -> i32 {
    let mut r = 0;
    helper(heights.as_slice(), 0, heights.len())
}


fn main() {
    let v = vec![3, 2, 5, 4, 6, 1, 4, 2];
    let r = largest_rectangle_area_violent_version(v.clone());
    println!("r={:?}", r);
    assert_eq!(r, 12);
    let r = largest_rectangle_area_divide_and_conquer_version(v.clone());
    println!("r={:?}", r);
    assert_eq!(r, 12);
}