pub fn production_rate_per_hour(speed: u8) -> f64 {
    let speed: f64 = speed as f64;
    let per_speed: f64 = 221.0;
    let total_cars: f64 = per_speed * speed;
    if speed > 4.0 && speed < 9.0 {
        total_cars * 0.9
    } else if speed > 8.0 {
        total_cars * 0.77
    } else {
        total_cars
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    let per_hour = working_items_per_minute(speed);
    per_hour / 60
}
