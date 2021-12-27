trait Interval {
    fn between(&self, left: &Self, right: &Self) -> bool;
}

impl<T> Interval for T 
where T: std::cmp::Ord 
{
    fn between(&self, left: &Self, right: &Self) -> bool {
        left <= self && self < right
    }
}

pub fn production_rate_per_hour(speed: u8) -> f64 {
    if speed.between(&1, &5) {
        speed as f64 * 221.0
    } else if speed.between(&5, &9) {
        speed as f64 * 221.0 * 0.9
    } else {
        speed as f64 * 221.0 * 0.77
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60.) as u32
}
