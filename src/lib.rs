#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod erc20 {
    use ink::storage::Mapping;
    use ink::prelude::string::String; // ✅ Import String for Result type

    #[ink(storage)]
    pub struct Erc20 {
        total_supply: Balance,
        balances: Mapping<AccountId, Balance>,
        owner: AccountId,
    }

    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        to: AccountId,
        value: Balance,
    }

    impl Erc20 {
        #[ink(constructor)]
        pub fn new(initial_supply: Balance) -> Self {
            let caller = Self::env().caller();
            let mut balances = Mapping::new();
            balances.insert(caller, &initial_supply);

            Self {
                total_supply: initial_supply,
                balances,
                owner: caller,
            }
        }

        #[ink(message)]
        pub fn total_supply(&self) -> Balance {
            self.total_supply
        }

        #[ink(message)]
        pub fn balance_of(&self, account: AccountId) -> Balance {
            self.balances.get(account).unwrap_or(0)
        }

        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, amount: Balance) -> Result<(), String> {
            let sender = self.env().caller();
            let sender_balance = self.balance_of(sender);

            if sender_balance < amount {
                return Err("Insufficient balance".into()); // ✅ Use String instead of &str
            }

            if let Some(new_sender_balance) = sender_balance.checked_sub(amount) {
                self.balances.insert(sender, &new_sender_balance);
            } else {
                return Err("Underflow error".into());
            }

            let recipient_balance = self.balance_of(to);
            if let Some(new_recipient_balance) = recipient_balance.checked_add(amount) {
                self.balances.insert(to, &new_recipient_balance);
            } else {
                return Err("Overflow error".into());
            }

            self.env().emit_event(Transfer {
                from: sender,
                to,
                value: amount,
            });

            Ok(())
        }
    }
}
