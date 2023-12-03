mod token;
pub use token::Client as TokenClient;
#[cfg(any(test, feature = "testutils"))]
pub use token::WASM as TOKEN_WASM;

mod lending_pool;
// pub use backstop::{BackstopDataKey, WASM as BACKSTOP_WASM};
// pub use backstop::{Client as BackstopClient, PoolBackstopData};
pub use lending_pool::Client as PoolClient;
pub use lending_pool::PoolConfig;
#[cfg(any(test, feature = "testutils"))]
pub use lending_pool::WASM as POOL_WASM;

mod pool_manager;
pub use pool_manager::Client as PoolManagerClient;
#[cfg(any(test, feature = "testutils"))]
pub use pool_manager::WASM as POOL_MANAGER_WASM;
