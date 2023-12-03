#[cfg(test)]
mod tests {
    use std::println;

    use soroban_sdk::auth::{ContractContext, InvokerContractAuthEntry, SubContractInvocation};
    use soroban_sdk::testutils::{Address as _, MockAuth, MockAuthInvoke};
    use soroban_sdk::xdr::SorobanAuthorizationEntry;
    use soroban_sdk::{vec, Address, Env, IntoVal, Symbol, Val, Vec};

    use crate::lending_pool::{
        PoolClient, Request, ReserveConfig, ReserveEmissionMetadata, POOL_WASM,
    };
    use crate::mock_dao::{MockDaoClient, MOCK_DAO_WASM};
    use crate::pool_manager::POOL_MANAGER_WASM;
    use crate::TokenClient;
    use crate::TOKEN_WASM;

    // #[test]
    // fn wasm_test() {
    //     let env = Env::default();

    //     let lending_pool_address = env.register_contract_wasm(None, POOL_WASM);
    //     let dao_address = env.register_contract_wasm(None, MOCK_DAO_WASM);
    //     let manager_address = env.register_contract_wasm(None, POOL_MANAGER_WASM);
    //     let name = Symbol::new(&env, "POOL");
    //     let oracle = Address::random(&env);
    //     let mut bstop_rate = 100_0000;
    //     let backstop_id = Address::random(&env);
    //     let blnd_id = Address::random(&env);
    //     let usdc_id = Address::random(&env);
    //     env.budget().reset_unlimited();
    //     let mut args: Vec<Val> = vec![
    //         &env.clone(),
    //         dao_address.into_val(&env),
    //         name.into_val(&env),
    //         oracle.into_val(&env),
    //         bstop_rate.into_val(&env),``
    //         backstop_id.into_val(&env),
    //         blnd_id.into_val(&env),
    //         usdc_id.into_val(&env),
    //     ];
    //     let dao_client = MockDaoClient::new(&env, &dao_address);
    //     dao_client.call_fn(
    //         &manager_address,
    //         &lending_pool_address,
    //         &Symbol::new(&env, "initialize"),
    //         &args,
    //     );
    //     let pool_client = PoolClient::new(&env, &lending_pool_address);
    //     let result = pool_client.try_initialize(
    //         &dao_address,
    //         &name,
    //         &oracle,
    //         &bstop_rate,
    //         &backstop_id,
    //         &blnd_id,
    //         &usdc_id,
    //     );

    //     match result {
    //         Ok(_) => panic!("Error"),
    //         Err(_) => (),
    //     };
    //     bstop_rate = 200_0000;
    //     env.budget().reset_unlimited();
    //     args = vec![&env.clone(), bstop_rate.into_val(&env)];
    //     dao_client.call_fn(
    //         &manager_address,
    //         &lending_pool_address,
    //         &Symbol::new(&env, "update_pool"),
    //         &args,
    //     );
    //     // let pool_client = PoolClient::new(&env, &lending_pool_address);
    //     // let pool_config: PoolConfig = pool_client
    //     //     .env
    //     //     .storage()
    //     //     .instance()
    //     //     .get(&Symbol::new(&env, "PoolConfig"))
    //     //     .unwrap();
    //     // assert_eq!(pool_config.bstop_rate, bstop_rate)
    // }
    #[test]
    fn wasm_deposit_test() {
        let env = Env::default();

        let bombadil = Address::random(&env);
        let lending_pool_address = env.register_contract_wasm(None, POOL_WASM);
        let dao_address = env.register_contract_wasm(None, MOCK_DAO_WASM);
        let manager_address = env.register_contract_wasm(None, POOL_MANAGER_WASM);
        let test_token_address = Address::random(&env);
        let token_address = env.register_contract_wasm(&test_token_address, TOKEN_WASM);
        let name = Symbol::new(&env, "POOL");
        let oracle = Address::random(&env);
        let mut bstop_rate = 100_0000;
        env.budget().reset_unlimited();
        let backstop_id = Address::random(&env);
        let blnd_id = Address::random(&env);
        let usdc_id = Address::random(&env);
        env.budget().reset_unlimited();
        let dao_client = MockDaoClient::new(&env, &dao_address);
        let pool_client = PoolClient::new(&env, &lending_pool_address);
        let token = TokenClient::new(&env, &token_address);
        token.initialize(
            &bombadil,
            &7,
            &"testtoken".into_val(&env),
            &"TKN".into_val(&env),
        );
        env.mock_auths(&[
            // MockAuth {
            //     address: &bombadil,
            //     invoke: &MockAuthInvoke {
            //         contract: &token_address,
            //         fn_name: "set_admin",
            //         args: vec![&env, bombadil.into_val(&env)],
            //         sub_invokes: &[],
            //     },
            // },
            MockAuth {
                address: &bombadil,
                invoke: &MockAuthInvoke {
                    contract: &token_address,
                    fn_name: "mint",
                    args: vec![
                        &env,
                        manager_address.into_val(&env),
                        100_0000000.into_val(&env),
                    ],
                    sub_invokes: &[],
                },
            },
        ]);
        // env.mock_all_auths();
        // println!("initializedd");
        // token.set_admin(&bombadil);
        println!("set admin");
        token.mint(&manager_address, &100_0000000);
        println!("minted");
        // println!("initialized");
        // token.mint(&bombadil, &100_0000000);
        // println!("minted");
        // token.mint(&dao_address, &100_0000000);

        pool_client.initialize(
            &bombadil,
            &name,
            &oracle,
            &bstop_rate,
            &backstop_id,
            &blnd_id,
            &usdc_id,
        );
        let asset_config = ReserveConfig {
            decimals: 7,
            c_factor: 0_7500000,
            l_factor: 0_7500000,
            util: 0_7500000,
            max_util: 0_9500000,
            r_one: 0_0500000,
            r_two: 0_5000000,
            r_three: 1_5000000,
            reactivity: 0_000_002_000, // 10e-5
            index: 0,
        };
        pool_client.init_reserve(&test_token_address, &asset_config);

        // enable emissions for pool
        let reserve_emissions: soroban_sdk::Vec<ReserveEmissionMetadata> = soroban_sdk::vec![
            &env,
            ReserveEmissionMetadata {
                res_index: 0, // TKN
                res_type: 0,  // d_token
                share: 0_600_0000
            },
            ReserveEmissionMetadata {
                res_index: 0, // TKN
                res_type: 1,  // b_token
                share: 0_400_0000
            },
        ];
        pool_client.set_emissions_config(&reserve_emissions);
        let requests: Vec<Request> = vec![
            &env,
            Request {
                request_type: 2,
                amount: 100,
                address: token_address.clone(),
            },
        ];
        let sub_args = vec![
            &env.clone(),
            dao_address.into_val(&env),
            lending_pool_address.into_val(&env),
            100.into_val(&env),
        ];

        let args = vec![
            &env.clone(),
            dao_address.into_val(&env),
            dao_address.into_val(&env),
            dao_address.into_val(&env),
            requests.into_val(&env),
        ];
        println!("calling fn :)");
        println!("token has {:?}", token_address);
        println!("pool_address {:?}", lending_pool_address);
        println!("dao address {:?}", dao_address);
        dao_client.call_fn(
            &manager_address,
            &lending_pool_address,
            &Symbol::new(&env, "submit"),
            &args,
            &token_address,
            &sub_args,
        );
    }
}
