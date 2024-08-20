use num::traits::{CheckedAdd, CheckedSub, Zero};
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Pallet<AccountId, Balance> {
    balances: BTreeMap<AccountId, Balance>,
}

impl<AccountId, Balance> Pallet<AccountId, Balance>
where
    AccountId: Ord + Clone,
    Balance: Zero + CheckedSub + CheckedAdd + Copy,
{
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new(),
        }
    }

    pub fn set_balance(&mut self, account: &AccountId, amount: Balance) {
        self.balances.insert(account.clone(), amount);
    }

    pub fn balance(&self, account: &AccountId) -> Balance {
        *self.balances.get(account).unwrap_or(&Balance::zero())
    }

    pub fn transfer(
        &mut self,
        caller: &AccountId,
        to: &AccountId,
        amount: &Balance,
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
    #[test]
    fn init_balances() {
        let mut balances = crate::balances::Pallet::new();
        assert_eq!(balances.balance(&"Michel".to_string()), 0);

        balances.set_balance(&"Michel".to_string(), 10);

        assert_eq!(balances.balance(&"Michel".to_string()), 10);
        assert_eq!(balances.balance(&"Vini".to_string()), 0);
    }

    #[test]
    fn transfer_balance() {
        let mut balances = crate::balances::Pallet::new();
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
