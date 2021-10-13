// This has been built with the original flipper smart contract code from Parity
//
// Copyright 2018-2021 Parity Technologies (UK) Ltd.
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

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
