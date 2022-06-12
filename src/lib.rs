use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, metadata, near_bindgen, Balance};

metadata! {
    #[near_bindgen]
    #[derive(Default, BorshDeserialize, BorshSerialize)]
    pub struct Deposit {
        balances: Balance,
    }

    #[near_bindgen]
    impl Deposit {
        #[payable]
        pub fn deposit(&mut self) {
            if near_sdk::env::attached_deposit() == 0 {
                near_sdk::env::panic_str("Invalid deposit");
            }

            self.balances += env::attached_deposit();
        }
    }

    
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::AccountId;
    use near_sdk::test_utils::test_env::bob;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::testing_env;

    fn set_predecessor_and_deposit(predecessor: AccountId, deposit: Balance) {
        testing_env!(VMContextBuilder::new()
            .predecessor_account_id(predecessor)
            .attached_deposit(deposit)
            .build())
    }

    #[test]
    fn deposit() {
        set_predecessor_and_deposit(bob(), 1);
        Deposit::default().deposit();
    }

    #[test]
    #[should_panic]
    fn invalid_deposit_panic() {
        set_predecessor_and_deposit(bob(), 0);
        Deposit::default().deposit();
    }
}
