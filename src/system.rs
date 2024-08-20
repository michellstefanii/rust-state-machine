use std::collections::BTreeMap;

type AccountId = String;
type Nonce = u32;
type BlockNumber = u32;

#[derive(Debug)]
pub struct Pallet {
    block_number: BlockNumber,
    nonce: BTreeMap<AccountId, Nonce>,
}

impl Pallet {
    pub fn new() -> Self {
        Pallet {
            block_number: 0,
            nonce: BTreeMap::new(),
        }
    }

    pub fn block_number(&self) -> BlockNumber {
        self.block_number
    }

    pub fn inc_block_number(&mut self) {
        self.block_number += 1;
    }

    pub fn inc_nonce(&mut self, account: &AccountId) {
        let nonce = self.nonce.get(account).unwrap_or(&0) + 1;
        self.nonce.insert(account.clone(), nonce);
    }
}

#[cfg(test)]
mod test {
  #[test]
  fn init_system() {
    let mut system = super::Pallet::new();

    assert_eq!(system.block_number(), 0);
    assert_eq!(system.nonce.get(&"daniel".to_string()), None);

    system.inc_block_number();

    assert_eq!(system.block_number(), 1);

    system.inc_nonce(&"daniel".to_string());
    assert_eq!(system.nonce.get(&"daniel".to_string()).unwrap(), &1);
  }
}