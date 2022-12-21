#[subxt::subxt(
    runtime_metadata_path = "../artifacts/heiko.scale",
    derive_for_all_types = "Clone"
)]
pub mod chain {}

pub use chain::runtime_types::pallet_liquid_staking::types::UnlockChunk;
