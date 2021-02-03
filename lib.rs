#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod flipper {

    #[ink(storage)]
    pub struct Flipper {
        is_private: bool,
        tally: u32,
    }

    impl Flipper {
        #[ink(constructor)]
        pub fn new(is_private: bool, tally: u32 ) -> Self {
            Self { is_private, tally}
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(false, 0)
        }

      
        #[ink(message)]
        pub fn change_private(&mut self) {
            self.is_private = !self.is_private;
        }

         #[ink(message)]
        pub fn adder(&mut self) {
            self.tally = self.tally + 1
        }
        #[ink(message)]
        pub fn get(&self) -> u32 {
            self.tally
        }
    }
}
