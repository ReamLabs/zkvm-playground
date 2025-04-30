pub mod large_variable_list;
pub mod print_usize;
pub mod u64_max_value;

pub trait Scenario {
    fn name() -> String;
    fn run();
}
