
    
    use subxt::{OnlineClient, SubstrateConfig, storage, tx::TxPayload};
    use sp_runtime::AccountId32;
    use std::str::FromStr;
    use codec::Encode;
    use std::error::Error;

    mod common;
    use common::*;
    // use sp_core::crypto::{AccountId32, Ss58AddressFormat, Ss58Codec};
    #[test]
    fn test(){
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let url = "wss://heiko-rpc.parallel.fi:443";
            inner(url).await.unwrap();
        });
    }

    async fn inner(url: &str) ->Result<(),Box<dyn Error>>{
        let api = OnlineClient::<SubstrateConfig>::from_url(url).await?;
        let metadata = api.metadata();

        let accounts = [
            ("hJLe4pxfrfRewDe1wvSXoPGbmecVe9ckW28K5rnhiyQsDVYy1",22762609489953),
            ("hJHTaMU6c9VHM6vq7LCk4mZrAG6V5RVShjoAzQoQBvWtyKsDa",53279195117398),
            ("hJFU3r4zioT39AaBiTriJCVvoepeEGViF38DAkKECUjVwsvZK",1913584636256187),
            ("hJK6oRtiPda7k1X5yGhVxYT7TZnu94REcbYt4uG5nMZECeAW3",43573520985352),
            ("hJFFeJeajRKf71M9gqnkFwHgSpADFj27x9fCLrmreUtUoKYmd",67857611214758),
        ];
        let mut items = vec![];
        for (addr,amount) in accounts.into_iter() {
            let account = AccountId32::from_str(addr)?;
            let unlockings = chain::storage().liquid_staking().unlockings(&account);
            let mut original_value = api.storage().fetch(&unlockings, None).await?.unwrap();
            let old_value = original_value.clone();
            let lookup_bytes = storage::utils::storage_address_bytes(&unlockings, &metadata)?;
            // let raw_key = format!("0x{}", hex::encode(&lookup_bytes));
            let encoded = api.storage().fetch_raw(&lookup_bytes, None).await?.unwrap();
            assert!(encoded == original_value.encode());

            if addr.eq("hJFU3r4zioT39AaBiTriJCVvoepeEGViF38DAkKECUjVwsvZK") {
                let item = original_value.get_mut(1).unwrap();
                assert!(item.value > amount);
                *item = UnlockChunk{
                    value: amount,
                    era: item.era,
                }

            }else {
                let item = original_value.get_mut(0).unwrap();
                assert!(item.value > amount);
                *item = UnlockChunk{
                    value: amount,
                    era: item.era,
                }
            }
            println!("account: {:?}",addr);
            println!("old: {:?}",old_value);
            println!("fix: {:?}",original_value);
            println!("-----------------------------");
            // let fix_raw_value = format!("0x{}", hex::encode(original_value.encode()));
            
            items.push((lookup_bytes,original_value.encode()));
        }

        // construct transaction
        let inner_tx = chain::tx().system().set_storage(items);
        let pre_image_tx = chain::tx().preimage().note_preimage(inner_tx.encode_call_data(&metadata)?);
        println!("pre_image_tx: {:?}",format!("0x{}", hex::encode(pre_image_tx.encode_call_data(&metadata)?)));

        // let sudo_tx = chain::tx().sudo().sudo(inner_tx.encode_call_data(&metadata)?);
        // println!("sudo_tx: {:?}",format!("0x{}", hex::encode(sudo_tx.encode_call_data(&metadata)?)));
        Ok(())
    }
    #[allow(dead_code)]
    async fn inner_test(url: &str) ->Result<(),Box<dyn Error>>{
        let api = OnlineClient::<SubstrateConfig>::from_url(url).await?;
        let account = AccountId32::from_str("hJLe4pxfrfRewDe1wvSXoPGbmecVe9ckW28K5rnhiyQsDVYy1")?;
        let unlockings = chain::storage().liquid_staking().unlockings(&account);
        // println!("key: {:?}", unlockings.encode());
        let x = api.storage().fetch(&unlockings, None).await?.unwrap();
        println!("value: {:?}", x);
        println!("value encode: {:?}", hex::encode(x.encode()));

        let metadata = api.metadata();
        let lookup_bytes = storage::utils::storage_address_bytes(&unlockings, &metadata)?;
        let x = format!("0x{}", hex::encode(&lookup_bytes));
        println!("raw key: {:?}", x);

        let encoded = api.storage().fetch_raw(&lookup_bytes, None).await?.unwrap();
        let x = format!("0x{}", hex::encode(encoded));
        println!("raw value: {:?}", x);

        Ok(())
    }   
