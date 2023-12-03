pub mod day_1 {
    use std::env;

    pub fn run() {
        let args: Vec<String> = env::args().collect();
        dbg!(args);
        // let contents = fs::read_to_string(file);
    }

    pub fn parse_calibration() {}
}

#[cfg(test)]
mod day_1_test {
    use super::*;

    #[test]
    fn parse_calibration() {
        day_1::run();
    }
}
