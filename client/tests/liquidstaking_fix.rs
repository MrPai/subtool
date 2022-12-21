
    
    use subxt::{OnlineClient, SubstrateConfig};
    use sp_runtime::AccountId32;
    use std::str::FromStr;
    
    mod common;
    use common::*;
    // use sp_core::crypto::{AccountId32, Ss58AddressFormat, Ss58Codec};
    #[test]
    fn tt(){
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            inner().await.unwrap();
        });
    }

    async fn inner() ->Result<(),Box<dyn std::error::Error>>{
        let url = "wss://heiko-rpc.parallel.fi:443";
        let api = OnlineClient::<SubstrateConfig>::from_url(url).await?;
        let account = AccountId32::from_str("hJLe4pxfrfRewDe1wvSXoPGbmecVe9ckW28K5rnhiyQsDVYy1")?;
        let unlockings = chain::storage().liquid_staking().unlockings(&account);
        let x = api.storage().fetch(&unlockings, None).await?;
        println!("{:?}", x);

        Ok(())
    }   
