use crate::scenarios::Scenario;

pub struct U64MaxValue;

impl Scenario for U64MaxValue {
    fn name() -> String {
        "u64_max_value".to_string()
    }

    fn run() {
        let u64_max_value: u64 = u64::MAX;

        println!("u64::MAX = {}", u64::MAX);
        println!("u64_max_value: {}", u64_max_value);
    }
}
