use soroban_sdk;
mod lending_pool_wasm {
    soroban_sdk::contractimport!(
        file = "../../blend-contracts/target/wasm32-unknown-unknown/optimized/pool.wasm"
    );
}
pub use lending_pool_wasm::Client as PoolClient;
#[cfg(any(test, feature = "testutils"))]
pub use lending_pool_wasm::WASM as POOL_WASM;
pub use lending_pool_wasm::{Request, ReserveConfig, ReserveEmissionMetadata};
