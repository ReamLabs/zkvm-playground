use ssz_types::{
    typenum::{U1073741824, Unsigned},
    VariableList
};

use crate::scenarios::Scenario;

pub struct LargeVariableList;

impl Scenario for LargeVariableList {
    fn name() -> String {
        "large_merkle".to_string()
    }

    fn run() {
        println!("U1073741824::to_usize(): {}", U1073741824::to_usize());
        println!("VariableList::<u8, U1073741824>::max_len(): {}", VariableList::<u8, U1073741824>::max_len());
    }
}
