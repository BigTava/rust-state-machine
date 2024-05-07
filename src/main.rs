mod balances;
mod system;

pub mod types {
    pub type AccountId = String;
    pub type Balance = u128;
    pub type BlockNumber = u32;
    pub type Nonce = u32;
}

#[derive(Debug)]
pub struct Runtime {
    system: system::Pallet,
    balances: balances::Pallet<types::AccountId, types::Balance>,
}

impl Runtime {
    fn new() -> Self {
        Self {
            system: system::Pallet::new(),
            balances: balances::Pallet::new(),
        }
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
