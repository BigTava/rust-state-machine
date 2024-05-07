use std::{ collections::BTreeMap, ops::AddAssign };
use num::traits::{ Zero, One };

#[derive(Debug)]
pub struct Pallet<BlockNumber, AccountId, Nonce> {
    block_number: BlockNumber,
    nonce: BTreeMap<AccountId, Nonce>,
}

impl<BlockNumber, AccountId, Nonce> Pallet<BlockNumber, AccountId, Nonce>
    where
        BlockNumber: Zero + One + AddAssign + Copy,
        AccountId: Ord + Clone,
        Nonce: Zero + One + Copy
{
    pub fn new() -> Self {
        Self {
            block_number: BlockNumber::zero(),
            nonce: BTreeMap::new(),
        }
    }

    pub fn block_number(&self) -> BlockNumber {
        self.block_number
    }

    pub fn inc_block_number(&mut self) {
        self.block_number += BlockNumber::one();
    }

    // Increment the nonce of an account. This helps us keep track of how many transactions each
    // account has made.
    pub fn inc_nonce(&mut self, who: &AccountId) {
        let nonce = *self.nonce.get(who).unwrap_or(&Nonce::zero());
        let new_nonce = nonce + Nonce::one();
        self.nonce.insert(who.clone(), new_nonce);
    }
}

#[cfg(test)]
mod test {
    use crate::system::Pallet;
    use crate::types::{ AccountId, BlockNumber, Nonce };

    #[test]
    fn init_system() {
        let mut system = Pallet::<BlockNumber, AccountId, Nonce>::new();
        system.inc_block_number();
        system.inc_nonce(&"alice".to_string());

        assert_eq!(system.block_number(), 1);
        assert_eq!(system.nonce.get(&"alice".to_string()), Some(&1));
    }
}
