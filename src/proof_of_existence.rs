use core::fmt::Debug;
use std::collections::BTreeMap;

pub trait Config: crate::system::Config {
    type Content: Debug + Ord;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
    claims: BTreeMap<T::Content, T::AccountId>,
}

impl<T: Config> Pallet<T> {
    pub fn new() -> Self {
        Self {
            claims: BTreeMap::new(),
        }
    }

    pub fn get_claim(&self, claim: &T::Content) -> Option<&T::AccountId> {
        self.claims.get(claim)
    }
}

#[macros::call]
impl<T: Config> Pallet<T> {
    pub fn create_claim(
        &mut self,
        caller: T::AccountId,
        claim: T::Content
    ) -> crate::support::DispatchResult {
        if self.claims.contains_key(&claim) {
            return Err(&"This content is already claimed");
        }
        self.claims.insert(claim, caller);
        Ok(())
    }

    pub fn revoke_claim(
        &mut self,
        caller: T::AccountId,
        claim: T::Content
    ) -> crate::support::DispatchResult {
        let owner = self.get_claim(&claim).ok_or("Claim does not exist")?;
        if caller != *owner {
            return Err(&"You are not the owner of this claim");
        }
        self.claims.remove(&claim);
        Ok(())
    }
}

#[cfg(test)]
mod test {
    struct TestConfig;
    use crate::types::{ AccountId, BlockNumber, Nonce, Content };

    impl super::Config for TestConfig {
        type Content = Content;
    }

    impl crate::system::Config for TestConfig {
        type AccountId = AccountId;
        type BlockNumber = BlockNumber;
        type Nonce = Nonce;
    }

    #[test]
    fn basic_proof_of_existence() {
        let alice = "alice".to_string();
        let bob = "bob".to_string();
        let alice_claim = "alice_claim";
        let bob_claim = "bob_claim";

        let mut poe = super::Pallet::<TestConfig>::new();

        assert_eq!(poe.get_claim(&alice_claim), None);
        assert_eq!(poe.create_claim(alice.clone(), &alice_claim), Ok(()));
        assert_eq!(poe.get_claim(&alice_claim), Some(&alice));

        assert_eq!(poe.create_claim(bob.clone(), &bob_claim), Ok(()));
        assert_eq!(poe.revoke_claim(bob.clone(), bob_claim), Ok(()));
        assert_eq!(poe.create_claim(bob, &bob_claim), Ok(()));
    }
}
