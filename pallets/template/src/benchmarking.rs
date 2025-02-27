// This file is part of Substrate.

// Copyright (C) 2020 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Balances pallet benchmarking.

#![cfg(feature = "runtime-benchmarks")]
// #![recursion_limit = "256"]

use super::*;

use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_support::{traits::Currency, weights::Weight};
use frame_system::RawOrigin;
use pallet_contracts::{BalanceOf, Module as Contracts};
use sp_runtime::traits::{Bounded, Hash};
use sp_std::prelude::*;

macro_rules! addr {
    () => {{
        let caller: T::AccountId = whitelisted_caller();
        let origin = RawOrigin::Signed(caller.clone());
        T::Currency::make_free_balance_be(&caller, BalanceOf::<T>::max_value());

        // Put WASM module
        let module = include_bytes!("../res/groth16.wasm").to_vec();
        let hash = T::Hashing::hash(&module);
        if let Err(e) = <Contracts<T>>::instantiate_with_code(
            origin.clone().into(),
            BalanceOf::<T>::max_value() / 2_u32.into(), // endowment
            Weight::max_value(),                        // gas_limit
            module,                                     // code
            [106, 55, 18, 226].to_vec(),                // default
            b"".to_vec(),                               // salt
        ) {
            panic!("{:?}", e);
        }

        // Calculate addr
        (
            Contracts::<T>::contract_address(&caller, &hash, &b"".to_vec()),
            caller,
        )
    }};
}

benchmarks! {
    where_clause { where
        T::AccountId: UncheckedFrom<T::Hash>,
        T::AccountId: AsRef<[u8]>,
    }

    bls12_377_verify {
        let (addr, caller) = addr!();
        let data = hex::decode("4e1f2d8108850171c5a779bb19e0aeb6030768f5001017bbd4edb645306ff1591424c501e7aa8b480fc18d8ed404c893d473afa9b82c01fb3e19bdf85f4fb8acf97d1507b75eb48972c8a10e13f4637f7bee71f89eafc141453e4646c82e188d9ef8cf71fa2f000085014d0869345487bb4070110fa318b0f408dcffce12304589878797615d8a72cc00b6a1e3f50cc0e59770638640090726013db560d3a19288ecedc60ed9ee0afd50ad3ac96db89d99fc249b85a453ef837e9213f8977ac572d818dc43387678a50100910a54f796d78550d14c4ee03cd5915fd8037569696f878312b0093f0149919fc717c0b3f8a2d419fad0edde7e1d55cd2f002c312b660250225a00baf5df511dcce61eb23b28f3980be7fcde0091284670e9064015f452f244e65cad047a20fdd000ca76892d04e1283e575e098c8e4281ba9113443515f22247921b9c3189dc03dd83cf22b9da3eda70cdd2644a795376002e0b1d0d1d5945dab12a28a31497b48d07739028ac3153be3cf367e3d7e788ac43f6d3ccd3c5c20a4f2d6b0db05ce40000f70cb4191a256ed9a4d90305f5864993032a0e40433040c259b14c8069d117f3491d75d993271a0a3f13efdd5b18ce006d90eff3b45f8fa10b43a2d4fb80a554304ba5d94ddc19c92b2e1e2f756bd3042c4c0d6e10b6c22c2ea5eb136fb42900aca080ed5cfa9ccf92da7e4f631f1f9fd52a4dd227898e21585a7a55656f6315fa3f1e541011b941084fb663515ede0024ff6c7745872af24c5de2acfbf4b7a238f3f80face2310d83d9e953de2051a70abc3be988220291e03c92840e1c1c01005720823639fe8f3818ade6c24c6d6ed8f78b87369a9fac8a548e01ca001e424ba931033958641fe06677c3f4a1b39f01f6d3e9a2052ad1185af9a80086665df50b41674f7f6f181773cee6e78d09ea34ce970ccb5907d5cb2f69c9e0ed8d1f01009d629094c47bdf026bbd3424f6e7748b0c3529b495cf24677f309d9f1ed7d3435513224cba204902fd733522b3b855015f395e5b2a7f0fe9e50665d705c48f9b8b20a3f9619f50c1dcddbea7c4a561cdffed750045f875b2d2c569e8d5d0320015e739c8881a211865f82e39bb6ad21a66ceb1808a5ecc3f11833d61dc938a3b160b1b1829527b52e9d3d98800f29701078dfc5fda2f3dcebaca0f43ac23f6da5044f51cf12ac8b3439df503ab1732f5047601b2cd9d93e6d161dc4719beda00000d06e2342d048b06618cfc80a5fb5301373032d5780ca45e5287e824fd1d3dccfc36d1befe73edaea7378968a003f94e1801de729f41f406c7d94db4995cc2956cdd6b250937dd52b3c8ac812dff25985eb802c727dbfb7c6b055db545d687b1bd000090f4ad7dc16060ebfbe2bff08077e404e878850682e15f64a5b0f243f53a6d61dc358d41fa6aed615c6296904d71c200c04fe75b3adbecdc3d00df22556a1c949e4b87c81483225f6eaa7560e3035a1c919c4a0d446125dca2e2ef541d58a200ec9b0644bea1bd34e84ff04b65c0cc8733d425b43225f81a7580e1027a41b4ce85eb513213f54a76e2b69320dc6d51003b06cb148aa149b0b0e1ffb8f5901adbda3e431c19404cae29f26aac97085c6d1d8eeef33024aafdc649da6475ab99000026b3bb740a34b4db3a1c689d471b5a7834bb27fd57108bca8cd88d12bb81eae5f8ad807e5de758978ab51d0146017100c31d5c7971576e79eec56286dc2602025cb1d0a01e1acdc23a8aece6e9f3c3ecf97d0a9667634e31e79f653a93c13b01000480a4961fec1c734055a80af91b3fd7405a2cdb741e1867aa7f783ca0959a077110").expect("Decode failed");
    }: call(
        RawOrigin::Signed(caller),
        T::Lookup::unlookup(addr),
        0_u32.into(),
        Weight::max_value(),
        data
    )

    bls12_381_verify {
        let (addr, caller) = addr!();
        let data = hex::decode("99a06637088501e60f6032ed7f170ba3481c5d99c0890554c808ad380a7c9aefc9e62eeecf0363ecef5e59ba8eaf6dd0af40e08d7cfd024314f90080a1461a207aa3098308149837bc6322a8f4b499671c507203e798f90a9aa8fafaebb806c8708c3207a93316008501ea7f398187aecf2721441218e54645b17acd35bdf65886255e772a58353ef2e25c92cf7f872bea5afca6081d03e6b115e74befd85720c241ee5ef038621a1cc72e329db9aebec0faf480e83cbd9e4b735650ab0cce2e321d64311bd34bca9e0200910a9094c864d37f890720969430f2682ccc76dc83f9234694590bd334b40dbfa7f28de40e914571d8ae2a88712844f86f0d6dd2f4875120f66304bfcedd55c9a805310155a5345ddc626cbb6fe5ce7026be174a08c798e4d1f1010f97138c30b10a0e23d3cde62852c7b2aabcd7603f87fc54cb0c2fcfa48a1a6e07ea917eec248284a1409dd21ae7bd64b862e7df91fd0af1f36857260a843ea18cd393172b91196ca17bc5f37efb126d585abfcf23b8438d72f90365c382b661256d7caa361d07000c06c5cc02155140b97283cc024daf57023a781a9115bb6e17350264f8b52784a6008921402af8c05dd39f4b789bd50435d2c5aefeb6dc9817af41f45779e3a2d80ab98ef923723b938566747c10a4a169ca5097c5d5073449c3d2e65e16291850a5721416e102df69729ffab67cf948ca51c4ac4c2b54c521552fd4a5a5462d6d3b77ba9932aba0fda8ea8da76e5b0b9166ba600f9c5cc5e442b64ed560c126f8b88e2761c0c6a17a4dcbc1a132a975683ed0b6018ac45d7f4942ad7831790e00408697568bc15871da494a2fe12199531130a8a5b96f07c69020dc01bf9f3ed043a6e1bd08f8d7826c7d5c0fc5c771195ab91bf6eeaa33cbd89cd7dce7930b383c2453d0cca5c227e1b97b1ae253e40cadf4468d6d86f0bf69a0b0bb55774219005b6d86316ea7c1dc4df3e3396a9f78aadf416d5af1619883f5dbbf0a39c6743b194e5adedda54d7cb3ae33e556602e19dd43a31bca7bdb15ad5245dc1ad0affb97edc8f35bb00d1c5b417a132d5be9b72a39870193b6a142198a888c0fe3760fdf1467c7208394270f2aff044c5c002d6cf189a28e34851dc434933e73e07482dad6c7ab931bfbdf29f5d51a6a866d028b91cd89abc9728cf2f8e139f612b5a248ca0758365d1d83c6bc299152b15a89b136e804ae71174d4f35970be570fd14000d06fcafa9753aee681f006d7bf6fc885dcc1a70c33bbfe92bb581ac51e1325b782b1128b1586767cb2ea27555e243abd213634b87418421a65002e06fd9347b23205761d7e35a2a83ffcd35b16b6bf23982700597f2ed9706ae5e039542e8eabd1800443cc8d6ffec17c44207dd41e8ab2f2f2c663f5ed9ab724321de43cbe2ea81908c13643c42a41be1adbc5076e4927b13046d3cdaf5ef775ee48d842599508178276cd41d30e289cbd1d2bb9f92f59b520138fef3e9529139f3961f40f6ea92113ed1701b0eb62a8b21b0852f5ae0d23b4d8cf7a02a69b04abbf0b9213cc1cd16b0afed74a09f1d965f72e0faaad60215c2788b9672a8f53fb90d44b47fb16e4f97f910226d8ad46ddc1dfa6eb4f709a3c95ccfa7f4cd1b9ea5f3cf3d10c5dd0200de094abc5e25fac259efcfd79a97401359069cde3b48eaa9d3a4d848959eb269d788a415d9c22fb418459474f1f0fc16f4572ace2876cf1170d400d1d12d588f816023602a077b8fa66293ac19c951b6c572fad4481fa576e362d9f3a0ffa0120004801d6c7850edbac8a5281ab93d2ed245d47b64f20c21950926d595624b488c291c").expect("Decode failed");
    }: call(
        RawOrigin::Signed(caller),
        T::Lookup::unlookup(addr),
        0_u32.into(),
        Weight::max_value(),
        data
    )

    bn254_verify {
        let (addr, caller) = addr!();
        let data = hex::decode("e028249708050103c2e806d904d8789f0caf1276fc4897bbf2bbb2e7243df2e4bf941ce616790479b410872e3c6f73f481d3e37b861fc1521d4a70457ac41efbe874fe0bf6572f0005014d609323ed564943ba7dfd3b362cc85abf7691dd9957c613fc0d282f258f4d17cd4ddfa713c0a6d4d0dcbfc245847bd6a96fa8603a0c0d55389279cdf2967f06001107f00c5d68983d7c029a66ae498a1cc6a2423bf67760bbf922ea7a937a87705e158db79bbff4eb9516d5342ce8cd8d09f68fa094434edceebbb43383fc415d9b235c64a2bc82ea6fa740441af956bc764eefb6be4067948d2d7afc08d7dc114a2e1c0c46208e6590bed192c336b7d89f55c2eca918e6f4eb88e1c729a10612690900f70bfb6ec84456af219bcd1d682133db3db80b0ae54cf31879e74a0ae547ec0ef92f56757f4a7976c50dc3d538cb1b4ed1736d4e8e8734d5cbf85b2ac8a93d0b654ff74df7e792b9cf0fd9d1b07b6ce4d0675f57bc7e9740e64b56c9f866b309eb428ffdf69b419fc53b9d76f1de62559aedc6db7e8b6ac2f3dd0cc963ca041300cc165291944a7b617bad2e6f1560f89f4e7eb42f154333b730e2f2da7828911d7cd0a28314d647bae049c72d4e0b4f31c1f09177fcab13014e3060a70024cb210094b31d9ebcca3957fff3d94960b0eba2197fabaaa68776e767e8785f63b4541d456d755240d72dae2ee0452c0cd705d7312b9a1d17cff916e87c763d459a550438b765cd8877bf753869e1852a7c0d9ac4520abb2b935e2f3ec12423d9edc5289eba05bed12e9fe16017c86791d682d4ae86d94eeab1d7888845c0fb0af85805000d04cf3bb657065cd0878eabc53790c5fa3680cad24db586bab799e3ae790217c61ba5050a0024f8761028a11dd22dfc030227b0a6c353ef5e7e1afad863c13a3304000227ad22191396e6ffe7aff14de91d7801e943f836de399b5fd5c8312086440c036648b695ffa27790f2f2a63fcf4c0b130e6b88ac786e3351fc6f13452d18034bee420c244cf7c63db4761f2feb7c14e409c8c550a68940cc0387e3812ccb10c4eb591ff3a67a87e794e2e76e25126b823ea9f38f95730951f13784e6e10f26007a922013b86567d6d48ff65baf8cdd0558c52db98f27b6c216df96f8aa1d4d2df7b1ead51d93e93ef5abdff83acaabd54736546f6995083127be1ae281ca5200000480829d3d572251103bac02b39e397c657c0b1f372bfc4d1b74f6c23de4993b4824").expect("Decode failed");
    }: call(
        RawOrigin::Signed(caller),
        T::Lookup::unlookup(addr),
        0_u32.into(),
        Weight::max_value(),
        data
    )

    bw6_761_verify {
        let (addr, caller) = addr!();
        let data = hex::decode("4399fe0608050329f6308fb1faee384c2deae7744b65d79eafa98a3947df7e35330c73ad0165a2f92844dab884fec1e15e52ac08f9306a1d8e7b4b66f718cece36a90f9aafe59588aafcd6588a55b9c05c3177904e5cbd1e3b59e4077d1caff6329268b5882200e349c8213aacabd50f11abe7f59b68fdd37f17b7fe6cee7ec3d04dca42812df1854c6c14234dd6fb7f64220949439c660ede18a5e79c8a115b942bbb7812b89be516469237a138752f6f798bcaf6afdf261e471f6c808aacd4274293cb8aff00000503639a14c4e08b97d97166d09eff9b57abf973b487fd5f5007c6ccc8adc31dbac74372477c035964e06f06d93e1483fe81ef1b51e525ffad28a053af20271dd363353e7ccd34cd599ac7c8c97fc175e4d9db8b9399db1647f52564757781209b001cc3d1e9665bc2a8b7e82d2d074e595f297df3dff536046353057a150f6a453521182a7219ef5d38ae85efd3b505f16549ed4bd63b7166e3a4f82142f584b3ad3e27c371db05cc0ba8a8f870fefccd9eb6e5635d6684193aa4b60082a0b2ac0000110c73174e4bb9c9b03c5ee9ddf21056832e01546744fffc64a5221e7e5cee09d37e534b27ae3725db7d3706c85b3932b269078f55cf1dd54c6e3247ad9a7619ef71315f5fee37909c44a216246c2ba2889b7b00b7f1298f5fb2776562527f4014008864e0733bfdd0d8f599c8bccd50d2204e3dab1863fc52e8a30476c836c16e3e6f7d098f0b34fd9edde526f9039323dcb0228a5e7c6841dcda703efe88d7331ba7d096b5753f442647e643d8ac9dfdd9d85b44824bbcc29ee8b4596e1cee520000d1e9ebd9d4d430b34703ac55be08c6d63e791dd2da3f7543b5d6db64e6ce9e0cfb031c168e2bede9cbd39070ecb4e1678d41cd0d48b087fa8a324964fded508df2ef48a7a6c21f1fe95e89aa98b00fb59bee1713e4ffaba216f00e5f15d12201f23da7e61729bc8866bcdd39bcd7d74e6cc9f9192b94c45c6753b60cef11c3b0474eaa53b3beaf3ee9f3885a547ae21385b5a5d5f15be620b9f2669573461b56e74dd326b48662cd271d1a21bb0c120f45d6bbdc632261908bc6e3ab4356480000be5dddc202ba74caa75135e9f81338910969729484a89a438090564689177b491e67f8cfa06ecfcee4acfd7cd364e094c1a4d028f7c7e0677d066cdf1355123e976fa498af59e308954054051fb3fabdda4b0d7a165cd454a71c53ff26546d000b193a142d557066f0a66964c249e4ee2d0247969ecfe861014dd4f36956da4c870ff3fbd6e16999cd43dc27c2fb2a5d72c7317dbc22d2f32541c1489a7bf716e698f898299c0ddfd09e3a892d2aa2940eec0b9f0cf63e52d83cd5dba7be50000021a8ba5d55fabd1999a5a9d717008e812486fd4309d9b4fb75ce4d22199360d6f8855d2e55259220392a9a67b9104adfa1424633f0c527b5516535d43c2632e594a118c05394ea735234b998018ac2906d7f24f89226bb07d1229c6000b13900b565ae54006fc9ef616aa866efa0e85b4c8d5a13ee55d60a478a0ea0b0cc29bc1fcab4045b6f980297a1815dcfbf5ad3031a0bb4429ad863f8695719b68208d1cd4e515a2b3129142a4bbdca3c3cde033c6256daa1fb79f133f28147a40c5200000d09cf5ea12a32dc313ec0791efdfca4ffa5dca127faeb57027754f0fcdd0e868623082e4543cae1bb11078c359dabbd4abbb38640c75081dca4aa2ef35bdb5ee4c856465c7b75956b88ab36d940a8df7e13adaa709ee349ed9454c7d6705211020087f361ca8cb2e83018809872a71853f0eb99a989362a3a753edbe4f58feec45c19be03b06567b357ec6779f3756b1494b74d98503d5c67ef3548ca9464d7525a3f92cdf335702a76eaaff83330f9da2c4a84b02a99ce06e0b04a1a0f1fd2f80000335b53718581ba3e14b48e381078551ff535a3d7a1671dc42898e742857f158a27d249a4d3bbe060016acdf3e9bde0095b23e073e40d56027474673321542ab30641f50f61452762d66984e33745d9456c5eba2cdafc07a52525ef1d3e221e00c92be8c3030de0338c27d281e887a63ab927478464faa7e7b0bbc68de44734c20cfd24763b8384fc6a15b2f82bf0349b86392aaa2e6fc2c95c1d0891f83a92d3b2c1deb230316eb43d9ddcbd069e600aff23ef13dd1f7b9cfade33db4872300000d1367d1907cd90da19e25e3241fb83393731c0d6183300675275017cc6287f2154abf2320cdc2cefc9df0383f98688e2e22914bef4fc32136222fb9dde45840de012e91cce1d02fbb6250a342b72c8c4155e0ff2ffa7ee71211f2641e816f200f8e725a65be98dafc395c538b2c1a009af190aedb99ecae9410257b5b295fb757dc7af138db54d06c582021442342f8924e092ae2fbf375fe94d6480910f7154b14aec033bd5cf27a2cdd225282b7a2fcaab6ac338a89d994c45976d9b7fa1000004c028933fc1b649ded455bd7739026be2bf6f7e1c0acde6dee164724218bbf1391e318017e5a6ab8248bda0bdd1f40b8301").expect("Decode failed");
    }: call(
        RawOrigin::Signed(caller),
        T::Lookup::unlookup(addr),
        0_u32.into(),
        Weight::max_value(),
        data
    )

    wasm_bls12_377_add {
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller.clone()))
        verify {}

    wasm_bls12_377_mul {
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller.clone()))
        verify {}

    wasm_bls12_377_pairing_two {
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller.clone()))
        verify {}

    wasm_bls12_377_pairing_six {
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller.clone()))
        verify {}

    wasm_bls12_377_verify {
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller.clone()))
        verify {}

    native_bls12_377_add {
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller.clone()))
        verify {}

    native_bls12_377_mul {
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller.clone()))
        verify {}

    native_bls12_377_pairing_two {
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller.clone()))
        verify {}

    native_bls12_377_pairing_six {
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller.clone()))
        verify {}

    native_bls12_377_verify {
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller.clone()))
        verify {}

    // bls12_381

    wasm_bls12_381_add {
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller.clone()))
        verify {}

    wasm_bls12_381_mul {
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller.clone()))
        verify {}

    wasm_bls12_381_pairing_two {
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller.clone()))
        verify {}

    wasm_bls12_381_pairing_six {
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller.clone()))
        verify {}

    wasm_bls12_381_verify {
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller.clone()))
        verify {}

    native_bls12_381_add {
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller.clone()))
        verify {}

    native_bls12_381_mul {
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller.clone()))
        verify {}

    native_bls12_381_pairing_two {
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller.clone()))
        verify {}

    native_bls12_381_pairing_six {
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller.clone()))
        verify {}

    native_bls12_381_verify {
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller.clone()))
        verify {}

    // bn254

    wasm_bn254_add {
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller.clone()))
        verify {}

    wasm_bn254_mul {
        let caller: T::AccountId = whitelisted_caller();
    }: wasm_bn254_mul(RawOrigin::Signed(caller.clone()))
        verify {}

    wasm_bn254_pairing_two {
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller.clone()))
        verify {}

    wasm_bn254_pairing_six {
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller.clone()))
        verify {}

    wasm_bn254_verify {
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller.clone()))
        verify {}

    native_bn254_add {
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller.clone()))
        verify {}

    native_bn254_mul {
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller.clone()))
        verify {}

    native_bn254_pairing_two {
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller.clone()))
        verify {}

    native_bn254_pairing_six {
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller.clone()))
        verify {}

    native_bn254_verify {
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller.clone()))
        verify {}

    // bw6_761

    wasm_bw6_761_add {
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller.clone()))
        verify {}

    wasm_bw6_761_mul {
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller.clone()))
        verify {}

    wasm_bw6_761_pairing_two {
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller.clone()))
        verify {}

    wasm_bw6_761_pairing_six {
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller.clone()))
        verify {}

    wasm_bw6_761_verify {
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller.clone()))
        verify {}

    native_bw6_761_add {
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller.clone()))
        verify {}

    native_bw6_761_mul {
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller.clone()))
        verify {}

    native_bw6_761_pairing_two {
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller.clone()))
        verify {}

    native_bw6_761_pairing_six {
        let caller: T::AccountId = whitelisted_caller();
    }:  _(RawOrigin::Signed(caller.clone()))
        verify {}

    native_bw6_761_verify {
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller.clone()))
        verify {}
}
