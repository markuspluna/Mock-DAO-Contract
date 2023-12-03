use soroban_sdk;

mod mock_dao_wasm {
    soroban_sdk::contractimport!(file = "../target/wasm32-unknown-unknown/release/mock_dao.wasm");
}
pub use mock_dao_wasm::Client as MockDaoClient;
pub use mock_dao_wasm::WASM as MOCK_DAO_WASM;
