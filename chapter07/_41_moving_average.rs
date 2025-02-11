

struct moving_average {
    capacity: i32,
    queue: std::collections::VecDeque<i32>,
    sum: i32
}

impl moving_average {
    fn next(&mut self,val: i32) -> f64 {
        self.queue.push_back(val);
        self.sum += val;
        if self.queue.len()  > self.capacity as usize {
            self.sum -= self.queue.pop_back().unwrap();
        }
        self.sum as f64 / self.queue.len() as f64
    }
}

fn main() {

}