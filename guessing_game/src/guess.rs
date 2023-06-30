pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(val: i32) -> Guess {
        if val < 0 || val > 100 {
            panic!("Guess value must be between 1 to 100.")
        }
        Guess { value: val }
    }
    pub fn value(&self) -> i32 {
        self.value
    }
}
