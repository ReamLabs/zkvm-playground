use crate::scenarios::Scenario;

pub struct PrintUsize;

impl Scenario for PrintUsize {
    fn name() -> String {
        "print_usize".to_string()
    }

    fn run() {
        // usize
        println!("usize::BITS = {}", usize::BITS);
        println!("usize::MAX = {}", usize::MAX);
    }
}
