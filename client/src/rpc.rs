
#[subxt::subxt(runtime_metadata_path = "../artifacts/heiko.scale")]
pub mod chain {}




#[cfg(test)]
mod test {
    use super::*;
    // use sp_core::crypto::{AccountId32, Ss58AddressFormat, Ss58Codec};
    #[test]
    fn tt(){
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {

        });
    }
}