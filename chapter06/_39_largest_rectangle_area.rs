

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


fn main() {
    let v = vec![3, 2, 5, 4, 6, 1, 4, 2];
    let r = largest_rectangle_area_violent_version(v.clone());
    println!("r={:?}", r);
    assert_eq!(r, 12);
}