use std::collections::BTreeMap;
use num::traits::{ CheckedAdd, CheckedSub, Zero };

pub trait Config: crate::system::Config {
    type Balance: Zero + CheckedSub + CheckedAdd + Copy;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
    balances: BTreeMap<T::AccountId, T::Balance>,
}

impl<T: Config> Pallet<T> {
    /// Create a new instance of the balances module.
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new(),
        }
    }

    pub fn set_balance(&mut self, who: &T::AccountId, amount: T::Balance) {
        self.balances.insert(who.clone(), amount);
    }

    pub fn balance(&self, who: &T::AccountId) -> T::Balance {
        *self.balances.get(who).unwrap_or(&T::Balance::zero())
    }
}

#[macros::call]
impl<T: Config> Pallet<T> {
    pub fn transfer(
        &mut self,
        caller: T::AccountId,
        to: T::AccountId,
        amount: T::Balance
    ) -> crate::support::DispatchResult {
        let caller_balance = self.balance(&caller);
        let to_balance = self.balance(&to);

        let new_caller_balance = caller_balance.checked_sub(&amount).ok_or("Not enough funds.")?;
        let new_to_balance = to_balance.checked_add(&amount).ok_or("Overflow.")?;

        self.set_balance(&caller, new_caller_balance);
        self.set_balance(&to, new_to_balance);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Pallet;
    use crate::types::{ AccountId, Balance, BlockNumber, Nonce };

    struct TestConfig;
    impl crate::system::Config for TestConfig {
        type AccountId = AccountId;
        type BlockNumber = BlockNumber;
        type Nonce = Nonce;
    }

    impl super::Config for TestConfig {
        type Balance = Balance;
    }
    #[test]
    fn init_balances() {
        let mut balances = Pallet::<TestConfig>::new();

        assert_eq!(balances.balance(&"alice".to_string()), 0);

        balances.set_balance(&"alice".to_string(), 100);
        assert_eq!(balances.balance(&"alice".to_string()), 100);
        assert_eq!(balances.balance(&"bob".to_string()), 0);
    }

    #[test]
    fn transfer_balance() {
        let mut balances = Pallet::<TestConfig>::new();

        assert_eq!(
            balances.transfer("alice".to_string(), "bob".to_string(), 100),
            Err("Not enough funds.")
        );

        balances.set_balance(&"alice".to_string(), 100);
        assert_eq!(balances.transfer("alice".to_string(), "bob".to_string(), 50), Ok(()));
        assert_eq!(balances.balance(&"alice".to_string()), 50);
        assert_eq!(balances.balance(&"bob".to_string()), 50);
    }
}
