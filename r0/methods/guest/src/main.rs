#[allow(unused_imports)] // Otherwise build fails with "error: linking with `rust-lld`"
use risc0_zkvm::guest::env;

use playground_common::scenarios::print_usize;

fn main() {
    println!("### RISC Zero guest running...");

    print_usize::run();
}
