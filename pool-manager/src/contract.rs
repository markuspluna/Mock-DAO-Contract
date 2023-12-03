use soroban_sdk::{contract, contractimpl, Address, Env, Symbol, Val, Vec};

#[contract]
pub struct PoolManager;

#[contractimpl]
impl PoolManager {
    pub fn authorized_fn_b(
        env: Env,
        dao_contract: Address,
        contract: Address,
        fn_name: Symbol,
        args: Vec<Val>,
    ) {
        dao_contract.require_auth();
        env.invoke_contract::<Val>(&contract, &fn_name, args);
    }
}
