#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod incrementer {

    #[ink(storage)]
    pub struct Incrementer {
        // ACTION: Add a `HashMap` from `AccountId` to `i32` named `my_value`!
        my_value_map: ink_storage::collections::HashMap<AccountId, i32>,
    }

    impl Incrementer {
        #[ink(constructor)]
        pub fn new(init_value: i32) -> Self {
            Self {
                my_value: init_value,
            }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self {
                my_value: 0,
            }
        }

        #[ink(message)]
        pub fn get(&self) -> i32 {
            self.my_value
        }

        #[ink(message)]
        pub fn inc(&mut self, by: i32) {
            self.my_value += by;
        }

        #[ink(message)]
        pub fn get_my_value(&self) -> i32 {
            // ACTION: Get `my_value` using `my_value_or_zero` on `&self.env().caller()`
            // ACTION: Return `my_value`
            let caller = self.env().caller();
            self.my_value_or_zero(&caller)
        }

        fn my_value_or_zero(&self, of: &AccountId) -> i32 {
            // ACTION: `get` and return the value of `of` and `unwrap_or` return 0
            let value = self.my_value.get(of).unwrap_or(&0);
            *value
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        // Alias `ink_lang` so we can use `ink::test`.
        use ink_lang as ink;

        #[ink::test]
        fn default_works() {
            let contract = Incrementer::default();
            assert_eq!(contract.get(), 0);
        }

        #[ink::test]
        fn it_works() {
            let mut contract = Incrementer::new(42);
            assert_eq!(contract.get(), 42);
            contract.inc(5);
            assert_eq!(contract.get(), 47);
            contract.inc(-50);
            assert_eq!(contract.get(), -3);
        }

        // Use `ink::test` to initialize accounts.
        #[ink::test]
        fn my_value_works() {
            let contract = Incrementer::new(11);
            assert_eq!(contract.get(), 11);
            assert_eq!(contract.get_mine(), 0);
        }
    }
}
