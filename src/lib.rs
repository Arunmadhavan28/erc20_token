#![cfg_attr(not(feature = "std"), no_std)]



#[ink::contract]
mod erc20 {
    use ink::storage::Mapping;

    /// An ERC-20 Token Contract
    #[ink(storage)]
    pub struct Erc20 {
        /// Total supply of the token
        total_supply: Balance,
        /// Mapping of account balances
        balances: Mapping<AccountId, Balance>,
        /// Owner of the contract
        owner: AccountId,
    }

    /// Events for logging transfers
    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        to: AccountId,
        value: Balance,
    }

    impl Erc20 {
        /// Constructor to initialize the token with `initial_supply`
        #[ink(constructor)]
        pub fn new(initial_supply: Balance) -> Self {
            let caller = Self::env().caller();
            let mut balances = Mapping::new(); // ✅ Fixed initialization
            balances.insert(caller, &initial_supply);

            Self {
                total_supply: initial_supply,
                balances,
                owner: caller,
            }
        }

        /// Returns the total supply of the token
        #[ink(message)]
        pub fn total_supply(&self) -> Balance {
            self.total_supply
        }

        /// Returns the balance of a given account
        #[ink(message)]
        pub fn balance_of(&self, account: AccountId) -> Balance {
            self.balances.get(account).unwrap_or(0)
        }

        /// Transfers tokens from the caller to another account
        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, amount: Balance) -> bool {
            let sender = self.env().caller();
            let sender_balance = self.balance_of(sender);

            // Ensure sender has enough balance
            if sender_balance < amount {
                return false;
            }

            // Update sender's balance (checked subtraction to avoid underflow)
            if let Some(new_sender_balance) = sender_balance.checked_sub(amount) {
                self.balances.insert(sender, &new_sender_balance);
            } else {
                return false;
            }

            // Update recipient's balance (checked addition to avoid overflow)
            let recipient_balance = self.balance_of(to);
            if let Some(new_recipient_balance) = recipient_balance.checked_add(amount) {
                self.balances.insert(to, &new_recipient_balance);
            } else {
                return false;
            }

            // Emit event for the transfer
            self.env().emit_event(Transfer {
                from: sender,
                to,
                value: amount,
            });

            true
        }
    }
}
