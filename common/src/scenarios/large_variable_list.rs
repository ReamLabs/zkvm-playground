use ssz_types::{
    typenum::{Unsigned, U2147483648, U4294967296, U1099511627776},
    VariableList
};

use crate::scenarios::Scenario;

pub struct LargeVariableList;

impl Scenario for LargeVariableList {
    fn name() -> String {
        "large_variable_list".to_string()
    }

    fn run() {
        // 2^31
        println!("U2147483648::to_usize(): {}", U2147483648::to_usize());
        println!("VariableList::<u8, U2147483648>::max_len(): {}", VariableList::<u8, U2147483648>::max_len());

        // 2^32
        println!("U4294967296::to_usize(): {}", U4294967296::to_usize());
        println!("VariableList::<u8, U4294967296>::max_len(): {}", VariableList::<u8, U4294967296>::max_len());

        // 2^40
        println!("U1099511627776::to_usize(): {}", U1099511627776::to_usize());
        println!("VariableList::<u8, U1099511627776>::max_len(): {}", VariableList::<u8, U1099511627776>::max_len());
    }
}
