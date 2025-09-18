pub fn calc_dist(prev_x: i32, prev_y: i32, x: i32, y: i32) -> i32 {
    (((prev_x - x) * (prev_x - x) + (prev_y - y) * (prev_y - y)) as f32).sqrt() as i32
}