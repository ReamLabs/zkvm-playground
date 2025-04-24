// These two lines are necessary for the program to properly compile.
//
// Under the hood, we wrap your main function with some extra code so that it behaves properly
// inside the zkVM.
#![no_main]
sp1_zkvm::entrypoint!(main);

use playground_common::scenarios::print_usize;

pub fn main() {
    println!("### SP1 guest running...");

    print_usize::run();
}
