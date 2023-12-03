use soroban_sdk;

mod pool_manager_wasm {
    soroban_sdk::contractimport!(
        file = "../target/wasm32-unknown-unknown/release/pool_manager.wasm"
    );
}
pub use pool_manager_wasm::Client as PoolManagerClient;
pub use pool_manager_wasm::WASM as POOL_MANAGER_WASM;
