use codec::{Decode, Encode};
use sp_runtime::AccountId32 as AccountId;
use std::error::Error;
use std::str::FromStr;
use subxt::{storage, tx::TxPayload, OnlineClient, PolkadotConfig};

use chain::runtime_types::xcm::{VersionedMultiLocation as LocalVersionedMultiLocation, VersionedXcm as LocalVersionedXcm};
// use chain::runtime_types::xcm::{
//     VersionedMultiLocation, VersionedXcm,
//     v0::junction::NetworkId,
//     v1::{
//         multilocation::Junctions::*,
//         multilocation::MultiLocation,
//         junction::Junction::*,
//         multiasset::*,
//     },
//     v2::{Xcm,Instruction::*, WeightLimit::Unlimited},
// };

use xcm::prelude::*;
use sp_runtime::traits::AccountIdConversion;
use cumulus_primitives_core::ParaId;

mod common;
use common::*;

#[test]
fn test() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let url = "wss://heiko-rpc.parallel.fi:443";
        inner_fix_usdt_failed(url).await.unwrap();
    });
}

#[rustfmt::skip]
async fn inner_fix_usdt_failed(url: &str) -> Result<(), Box<dyn Error>> {
    let api = OnlineClient::<PolkadotConfig>::from_url(url).await?;
    let metadata = api.metadata();

    let accounts = [
        ("hJK6oRtiPda7k1X5yGhVxYT7TZnu94REcbYt4uG5nMZECeAW3",224477299,),// https://parallel-heiko.subscan.io/xcm_message/kusama-80444856306587ba3497d5d73bb558b32fbf517b
        ("hJGNLzjF4hnG6fEERozYYXjCPnoGavqtALTXoZHm9GdnkR1sK",135900000,),// https://parallel-heiko.subscan.io/xcm_message/kusama-075a500aca5cf63026e1d069b56818c53e3d41d9
        ("hJK6oRtiPda7k1X5yGhVxYT7TZnu94REcbYt4uG5nMZECeAW3",272894,),// https://parallel-heiko.subscan.io/xcm_message/kusama-82c7a4832f7b38104f2fc5facf38573b559d334d
        ("hJJNn7Lz4ThikkYpdqq7N8SPmCpbuFPo9tua7drjatPW7dcQ9",100000,),// https://parallel-heiko.subscan.io/xcm_message/kusama-0bd1aad742b37fdac81d5acdcd51f33c59e2181d
        ("hJK6oRtiPda7k1X5yGhVxYT7TZnu94REcbYt4uG5nMZECeAW3",100000,),// https://parallel-heiko.subscan.io/xcm_message/kusama-d45fc07ab6e8bfb93f21921bda2cc507b1f60c50
        ("hJKjLjV7pM9PohiYYLaMZukf6Ziw95AAGjDQDUwguBK8UeAti",7036340,),// https://parallel-heiko.subscan.io/xcm_message/kusama-7a81487b3cb250b4ef641be3dd821eabda86f7d9
        ("hJLEWZgwfZZBZoqGTG3sYbSR1DxGJA5SMB57x28QqfeAzQJ5H",80059795,),// https://parallel-heiko.subscan.io/xcm_message/kusama-57521d99d9d63f291bbfe8f9a52f3d63cd9c1c6b
        ("hJJBRj4C1DgCH1XfPeToFpppDcixBt6SoYjYFhgppsDq2ouPa",53835791,),// https://parallel-heiko.subscan.io/xcm_message/kusama-3c56b76c8dea76337d374d5c77e651681646daf3
        ("hJJBRj4C1DgCH1XfPeToFpppDcixBt6SoYjYFhgppsDq2ouPa",1000075,),// https://parallel-heiko.subscan.io/xcm_message/kusama-57f2c942219cee7eddbc73ec2531c3c5e80731de
        ("hJLEWZgwfZZBZoqGTG3sYbSR1DxGJA5SMB57x28QqfeAzQJ5H",1475989,),// https://parallel-heiko.subscan.io/xcm_message/kusama-437d85f0390925f483cafa5bbb89d9b5c45bae00
        ("hJGqexuuRvrrAKKXtnpRoNTLzx8Xbhu7YU72zyMtEFKK9LtMc",25000000,),// https://parallel-heiko.subscan.io/xcm_message/kusama-60225b76c41b6e8181e9bf081e3a0719c1b456c9
        ("hJJbhsrGL1nvZ1k4mvW82tY3zfYZubaSsDLptB6ncyfCGnaD3",37148308,),// https://parallel-heiko.subscan.io/xcm_message/kusama-e1652e5d23f5e3e91dabf12d92df94fdeb403b8c
    ];
    let destination = MultiLocation::new(1, X1(Parachain(1000)));
    let fees: MultiAsset = (MultiLocation::parent(), 30_000_000_000).into();
    let mut message: xcm::v2::Xcm<()> = Xcm(vec![
        WithdrawAsset(MultiAssets::from(fees.clone())),
        BuyExecution {
            fees: fees.clone(),
            weight_limit: Unlimited,
        },
    ]);

    // Claim Asset
    for &(_, amount) in accounts.iter() {
        let asset: MultiAsset = (MultiLocation::new(0, X2(PalletInstance(50),GeneralIndex(1984))), amount).into();
        let claim_msg = ClaimAsset { assets: asset.into(), ticket: MultiLocation::default() };
        message.0.push(claim_msg);
    }
    // Deposit Asset
    for &(addr, amount) in accounts.iter() {
        let asset: MultiAsset = (MultiLocation::new(0, X2(PalletInstance(50),GeneralIndex(1984))), amount).into();
        message.0.push(
            DepositAsset {
                assets: asset.into(),
                max_assets: 1,
                beneficiary: convert(AccountId::from_str(addr)?),
            }
        )
    }
    let account = ParaId::from(2085).into_account_truncating();
    message.0.push(RefundSurplus);
    message.0.push(
        DepositAsset {
            assets: Wild(All),
            max_assets: 1,
            beneficiary: convert(account),
        }
    );
    // construct transaction
    let d = VersionedMultiLocation::V1(destination).encode();
    let v = VersionedXcm::V2(message).encode();
    // let inner_tx = chain::tx().system().set_storage(items);
    let inner_tx = chain::tx().orml_xcm().send_as_sovereign(
        LocalVersionedMultiLocation::decode(&mut &d[..]).unwrap(),
        LocalVersionedXcm::decode(&mut &v[..]).unwrap()
    );
    // println!(
    //     "orml_xcm_tx: {:?}",
    //     format!(
    //         "0x{}",
    //         hex::encode(api.tx().call_data(&inner_tx)?)
    //     )
    // );

    let pre_image_tx = chain::tx()
        .preimage()
        .note_preimage(api.tx().call_data(&inner_tx)?);
    println!(
        "pre_image_tx: {:?}",
        format!(
            "0x{}",
            hex::encode(pre_image_tx.encode_call_data(&metadata)?)
        )
    );
    Ok(())
}

fn convert(account: AccountId) -> MultiLocation {
    MultiLocation {
        parents: 0,
        interior: X1(AccountId32 {
            network: NetworkId::Any,
            id: account.into(),
        }),
    }
}