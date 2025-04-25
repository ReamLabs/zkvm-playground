pub mod large_variable_list;
pub mod print_usize;

pub trait Scenario {
    fn name() -> String;
    fn run();
}
