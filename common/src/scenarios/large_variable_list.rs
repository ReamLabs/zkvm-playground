use ssz_types::{
    typenum::{Unsigned, U16, U256, U65536, U2147483648, U4294967296, U1099511627776},
    VariableList
};

use crate::scenarios::Scenario;

pub struct LargeVariableList;

impl Scenario for LargeVariableList {
    fn name() -> String {
        "large_variable_list".to_string()
    }

    fn run() {
        // 2^4
        println!("U16::to_usize(): {}", U16::to_usize());
        println!("VariableList::<u8, U16>::max_len(): {}", VariableList::<u8, U16>::max_len());

        // 2^8
        println!("U256::to_usize(): {}", U256::to_usize());
        println!("VariableList::<u8, U256>::max_len(): {}", VariableList::<u8, U256>::max_len());

        // 2^16
        println!("U65536::to_usize(): {}", U65536::to_usize());
        println!("VariableList::<u8, U65536>::max_len(): {}", VariableList::<u8, U65536>::max_len());

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
