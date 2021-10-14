// This has been built with the original flipper smart contract code from Parity
// Original Code available at https://github.com/paritytech/ink

#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
pub mod authorize_access {
    #[ink(storage)]
    pub struct AuthorizeAccess {
        value: bool,
    }

    impl AuthorizeAccess {
        /// Creates a new Authorize Acccess smart contract initialized with the given value.
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self { value: init_value }
        }

        /// Creates a new Authorize Access smart contract initialized to `false`.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }

        /// changes the current value of the Authorize Access's bool to true.
        #[ink(message)]
        pub fn authorize_access(&mut self) {
            self.value = true;
        }

        /// changes the current value of the Authorize Access's bool to false.
        #[ink(message)]
        pub fn revoke_access(&mut self) {
            self.value = false;
        }

        /// Returns the current value of the Authorize Access's bool.
        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn default_works() {
            let authorize_access = AuthorizeAccess::default();
            assert_eq!(authorize_access.get(), false);
        }

        #[test]
        fn it_works() {
            let mut authorize_access = AuthorizeAccess::new(false);
            assert_eq!(authorize_access.get(), false);
            authorize_access.authorize_access();
            assert_eq!(authorize_access.get(), true);
            authorize_access.revoke_access();
            assert_eq!(authorize_access.get(), false);
        }
    }
}
