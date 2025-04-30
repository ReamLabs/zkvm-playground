#[allow(unused_imports)] // Otherwise build fails with "error: linking with `rust-lld`"
use risc0_zkvm::guest::env;

use playground_common::{
    helpers::print_separator,
    scenarios::{Scenario, large_variable_list::LargeVariableList, print_usize::PrintUsize, u64_max_value::U64MaxValue}
};

pub fn main() {
    print_separator(std::any::type_name::<PrintUsize>());
    PrintUsize::run();

    print_separator(std::any::type_name::<LargeVariableList>());
    LargeVariableList::run();

    print_separator(std::any::type_name::<U64MaxValue>());
    U64MaxValue::run();

    print_separator("END");
}
