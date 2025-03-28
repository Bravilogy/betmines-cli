pub trait FilterValidation {
    fn is_low_performing(&self) -> bool;
    fn is_valid(&self) -> bool;
}

pub trait FilterScoring {
    fn get_score(&self) -> f64;
}
