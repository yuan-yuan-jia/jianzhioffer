

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

fn largest_rectangle_area_stack_version(heights: Vec<i32>) -> i32 {
    let mut area = 0;
    let mut stack = std::collections::VecDeque::new();
    for i in heights.iter().enumerate() {
            // 栈顶的元素大于当前元素
        while  !stack.is_empty() && *heights.get( *stack.back().unwrap()).unwrap() > *i.1 { 
            let stack_peek = stack.pop_back().unwrap() as i32;
            let rectangle_height = heights.get(stack_peek as usize).unwrap();
            // 计算以rectangle_height为高的面积       
            let left_index = stack.back();
            let right_index = i.0 as i32;
            let width;
            if let Some(left) = left_index {
                width = right_index - (*left) as i32 - 1; 
            } else {
                width = stack_peek - 1;    
            }
            area = area.max(width * (*rectangle_height));
        }
        stack.push_back(i.0);
            
    }
    while !stack.is_empty() {
        let stack_peek = stack.pop_back().unwrap() as i32;
        let rectangle_height = heights.get(stack_peek as usize).unwrap();
        let left_index = match stack.back() {
            Some(r) => { (*r) as i32},
            None => stack_peek - 1 
        };
        let right_index = stack_peek + 1;
        let width = right_index - left_index - 1;
        
        area = area.max(width * (*rectangle_height));
    }
    area
}

fn main() {
    let v = vec![3, 2, 5, 4, 6, 1, 4, 2];
    let r = largest_rectangle_area_violent_version(v.clone());
    println!("r={:?}", r);
    assert_eq!(r, 12);
    let r = largest_rectangle_area_divide_and_conquer_version(v.clone());
    println!("r={:?}", r);
    assert_eq!(r, 12);
    let r = largest_rectangle_area_stack_version(v.clone());
    println!("r={:?}", r);
    assert_eq!(r, 12);
}