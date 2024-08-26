use num::traits::{CheckedAdd, CheckedSub, Zero};
use std::collections::BTreeMap;

pub trait Config: crate::system::Config {
    type Balance: Zero + CheckedSub + CheckedAdd + Copy;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
    balances: BTreeMap<T::AccountId, T::Balance>,
}

impl<T: Config> Pallet<T> {
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new(),
        }
    }

    pub fn set_balance(&mut self, account: &T::AccountId, amount: T::Balance) {
        self.balances.insert(account.clone(), amount);
    }

    pub fn balance(&self, account: &T::AccountId) -> T::Balance {
        *self.balances.get(account).unwrap_or(&T::Balance::zero())
    }

    pub fn transfer(
        &mut self,
        caller: &T::AccountId,
        to: &T::AccountId,
        amount: &T::Balance,
    ) -> Result<(), &'static str> {
        let caller_balance = self.balance(&caller);
        let to_balance = self.balance(&to);

        let new_caller_balance = caller_balance
            .checked_sub(&amount)
            .ok_or("Not enough funds.")?;
        let new_to_balance = to_balance.checked_add(&amount).ok_or("Funds not added.")?;

        self.set_balance(&caller, new_caller_balance);
        self.set_balance(&to, new_to_balance);

        Ok(())
    }
}

mod tests {
    struct TestConfig;

    impl super::Config for TestConfig {
        type Balance = u128;
    }

    impl crate::system::Config for TestConfig {
        type AccountId = String;
        type BlockNumber = u32;
        type Nonce = u32;
    }

    #[test]
    fn init_balances() {
        let mut balances = super::Pallet::<TestConfig>::new();
        assert_eq!(balances.balance(&"Michel".to_string()), 0);

        balances.set_balance(&"Michel".to_string(), 10);

        assert_eq!(balances.balance(&"Michel".to_string()), 10);
        assert_eq!(balances.balance(&"Vini".to_string()), 0);
    }

    #[test]
    fn transfer_balance() {
        let mut balances = super::Pallet::<TestConfig>::new();
        let caller = "alice".to_string();
        let to = "bob".to_string();

        assert_eq!(balances.balance(&caller), 0);
        assert_eq!(balances.balance(&to), 0);

        balances.set_balance(&caller, 30);
        balances.set_balance(&to, 10);

        let result = balances.transfer(&caller, &to, &40);
        assert_eq!(result, Err("Not enough funds."));

        let _ = balances.transfer(&caller, &to, &20);

        assert_eq!(balances.balance(&caller.to_string()), 10);
        assert_eq!(balances.balance(&to.to_string()), 30);
    }
}
