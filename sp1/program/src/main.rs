// These two lines are necessary for the program to properly compile.
//
// Under the hood, we wrap your main function with some extra code so that it behaves properly
// inside the zkVM.
#![no_main]
sp1_zkvm::entrypoint!(main);

use playground_common::{
    helpers::print_separator,
    scenarios::{Scenario, large_variable_list::LargeVariableList, print_usize::PrintUsize}
};

pub fn main() {
    print_separator(std::any::type_name::<PrintUsize>());
    PrintUsize::run();

    print_separator(std::any::type_name::<LargeVariableList>());
    LargeVariableList::run();

    print_separator("END");
}
