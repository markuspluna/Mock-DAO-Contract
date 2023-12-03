use crate::dependencies::PoolManagerClient;
/// This example demonstrates how a contract can authorize deep subcontract
/// calls on its behalf.
///
/// By default, only direct calls that contract makes are authorized. However,
/// in some scenarios one may want to authorize a deeper call (a common example
/// would be token transfer).
///
/// Here we provide the abstract example: contract A calls contract B, then
/// contract B calls contract C. Both contract B and contract C `require_auth`
/// for contract A address and contract A provides proper authorization to make
/// the calls succeed.
use soroban_sdk::{
    auth::{ContractContext, InvokerContractAuthEntry, SubContractInvocation},
    contract, contractimpl, vec, Address, Env, IntoVal, Symbol, Val, Vec,
};

#[contract]
pub struct DaoContract;

#[contractimpl]
impl DaoContract {
    pub fn call_fn(
        env: Env,
        manager: Address,
        contract: Address,
        fn_name: Symbol,
        args: Vec<Val>,
        token: Address,
        sub_args: Vec<Val>,
    ) {
        let sub_invocation: InvokerContractAuthEntry =
            InvokerContractAuthEntry::Contract(SubContractInvocation {
                context: ContractContext {
                    contract: token.clone(),
                    fn_name: Symbol::new(&env, "transfer"),
                    args: (
                        env.current_contract_address(),
                        contract.clone(),
                        (100 as i128),
                    )
                        .into_val(&env),
                },
                sub_invocations: vec![&env],
            });
        let sub_invocations: Vec<InvokerContractAuthEntry> = vec![&env, sub_invocation.clone()];
        env.authorize_as_current_contract(vec![
            &env,
            InvokerContractAuthEntry::Contract(SubContractInvocation {
                context: ContractContext {
                    contract: contract.clone(),
                    fn_name: fn_name.clone(),
                    args: args.clone(),
                },
                sub_invocations,
            }),
        ]);
        let client = PoolManagerClient::new(&env, &manager);
        client.authorized_fn_b(&env.current_contract_address(), &contract, &fn_name, &args);
        // env.invoke_contract::<Val>(&contract, &fn_name, args);
    }
}

// #[cfg(test)]
// mod tests {
//     use soroban_sdk::testutils::Address as _;
//     use soroban_sdk::{vec, Address, Env, IntoVal, Symbol, Val, Vec};

//     use crate::contract::DaoContract;
//     use crate::dependencies::{PoolClient, POOL_MANAGER_WASM, POOL_WASM};

//     #[test]
//     fn test() {
//         let env = Env::default();
//         let lending_pool_address = env.register_contract_wasm(None, POOL_WASM);
//         let dao_address = env.register_contract(None, DaoContract {});
//         let manager_address = env.register_contract_wasm(None, POOL_MANAGER_WASM);
//         let name = Symbol::new(&env, "POOL");
//         let oracle = Address::random(&env);
//         let mut bstop_rate = 100_0000;
//         let backstop_id = Address::random(&env);
//         let blnd_id = Address::random(&env);
//         let usdc_id = Address::random(&env);
//         env.budget().reset_unlimited();
//         env.as_contract(&dao_address, || {
//             let mut args: Vec<Val> = vec![
//                 &env.clone(),
//                 dao_address.into_val(&env),
//                 name.into_val(&env),
//                 oracle.into_val(&env),
//                 bstop_rate.into_val(&env),
//                 backstop_id.into_val(&env),
//                 blnd_id.into_val(&env),
//                 usdc_id.into_val(&env),
//             ];
//             let sub = vec![&env];
//             DaoContract::call_fn(
//                 env.clone(),
//                 manager_address.clone(),
//                 lending_pool_address.clone(),
//                 Symbol::new(&env, "initialize"),
//                 args,
//                 sub.clone(),
//             );
//             let pool_client = PoolClient::new(&env, &lending_pool_address);
//             let result = pool_client.try_initialize(
//                 &dao_address,
//                 &name,
//                 &oracle,
//                 &bstop_rate,
//                 &backstop_id,
//                 &blnd_id,
//                 &usdc_id,
//             );
//             match result {
//                 Ok(_) => panic!("Error"),
//                 Err(_) => (),
//             };
//             bstop_rate = 200_0000;
//             env.budget().reset_unlimited();
//             args = vec![&env.clone(), bstop_rate.into_val(&env)];
//             DaoContract::call_fn(
//                 env.clone(),
//                 manager_address.clone(),
//                 lending_pool_address.clone(),
//                 Symbol::new(&env, "update_pool"),
//                 args,
//                 sub,
//             );
//             // let pool_client = PoolClient::new(&env, &lending_pool_address);
//             // let pool_config: PoolConfig = pool_client
//             //     .env
//             //     .storage()
//             //     .instance()
//             //     .get(&Symbol::new(&env, "PoolConfig"))
//             //     .unwrap();
//             // assert_eq!(pool_config.bstop_rate, bstop_rate)
//         });
//     }
// }
