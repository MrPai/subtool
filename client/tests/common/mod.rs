#[subxt::subxt(
    runtime_metadata_path = "../artifacts/heiko.scale",
    derive_for_all_types = "Clone"
)]
pub mod chain {}

pub use chain::runtime_types::pallet_liquid_staking::types::UnlockChunk;

pub use chain::runtime_types::heiko_runtime::RuntimeCall as HeikoRuntimeCall;
pub use chain::runtime_types::pallet_assets::pallet::Call as AssetsCall;
pub use chain::runtime_types::orml_xcm::module::Call as OrmlXcmCall;
