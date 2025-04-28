use crate::scenarios::Scenario;

pub struct PrintUsize;

impl Scenario for PrintUsize {
    fn name() -> String {
        "print_usize".to_string()
    }

    fn run() {
        // u64
        println!("u64::MAX = {}", u64::MAX);

        // usize
        println!("usize::BITS = {}", usize::BITS);
        println!("usize::MAX = {}", usize::MAX);
    }
}
