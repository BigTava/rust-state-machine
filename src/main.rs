mod balances;
mod system;
mod support;

use crate::support::Dispatch;

pub mod types {
    pub type AccountId = String;
    pub type Balance = u128;
    pub type BlockNumber = u32;
    pub type Nonce = u32;
    pub type Extrinsic = crate::support::Extrinsic<AccountId, crate::RuntimeCall>;
    pub type Header = crate::support::Header<BlockNumber>;
    pub type Block = crate::support::Block<Header, Extrinsic>;
}

pub enum RuntimeCall {
    BalancesTransfer {
        to: types::AccountId,
        amount: types::Balance,
    },
}

#[derive(Debug)]
pub struct Runtime {
    system: system::Pallet<Self>,
    balances: balances::Pallet<Self>,
}

impl system::Config for Runtime {
    type AccountId = types::AccountId;
    type BlockNumber = types::BlockNumber;
    type Nonce = types::Nonce;
}

impl balances::Config for Runtime {
    type Balance = types::Balance;
}

impl Dispatch for Runtime {
    type Caller = <Runtime as system::Config>::AccountId;
    type Call = RuntimeCall;

    fn dispatch(
        &mut self,
        caller: Self::Caller,
        runtime_call: Self::Call
    ) -> support::DispatchResult {
        match runtime_call {
            RuntimeCall::BalancesTransfer { to, amount } => {
                self.balances.transfer(&caller, &to, amount)?;
            }
        }
        Ok(())
    }
}

impl Runtime {
    fn new() -> Self {
        Self {
            system: system::Pallet::new(),
            balances: balances::Pallet::new(),
        }
    }

    fn execute_bock(&mut self, block: types::Block) -> support::DispatchResult {
        self.system.inc_block_number();
        if block.header.block_number != self.system.block_number() {
            return Err(&"block number foes not match what is expected");
        }
        for (i, support::Extrinsic { caller, call }) in block.extrinsics.into_iter().enumerate() {
            self.system.inc_nonce(&caller);
            let _res = self
                .dispatch(caller, call)
                .map_err(|e|
                    eprintln!(
                        "Extrinsic Error\n\tBlock Number: {}\n\tExtrinsic Number: {}\n\tError: {}",
                        block.header.block_number,
                        i,
                        e
                    )
                );
        }
        Ok(())
    }
}

fn main() {
    let alice = "alice".to_string();
    let bob = "bob".to_string();
    let charlie = "charlie".to_string();
    let mut runtime = Runtime::new();
    runtime.balances.set_balance(&alice, 100);

    runtime.system.inc_block_number();
    assert_eq!(runtime.system.block_number(), 1);

    runtime.system.inc_nonce(&alice);
    let _res = runtime.balances.transfer(&alice, &bob, 30).map_err(|e| eprintln!("{}", e));

    runtime.system.inc_nonce(&alice);
    let _res = runtime.balances.transfer(&alice, &charlie, 20).map_err(|e| eprintln!("{}", e));
}
