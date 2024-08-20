mod balances;
mod system;

type AccountId = String;
type Balance = u128;

#[derive(Debug)]
pub struct Runtime {
    balances: balances::Pallet<AccountId, Balance>,
    system: system::Pallet,
}

impl Runtime {
    fn new() -> Self {
        Runtime {
            balances: balances::Pallet::new(),
            system: system::Pallet::new(),
        }
    }
}

fn main() {
    let mut runtime = Runtime::new();
    runtime.balances.set_balance(&"alice".to_string(), 100);

    runtime.system.inc_block_number();
    assert_eq!(runtime.system.block_number(), 1);
    runtime.system.inc_nonce(&"alice".to_string());

    let _res = runtime
        .balances
        .transfer(&"alice".to_string(), &"bob".to_string(), &30)
        .map_err(|e| eprintln!("{}", e));

    runtime.system.inc_block_number();

    let _res = runtime
        .balances
        .transfer(&"alice".to_string(), &"bob".to_string(), &20)
        .map_err(|e| eprintln!("{}", e));

    println!("{:#?}", runtime);
}
