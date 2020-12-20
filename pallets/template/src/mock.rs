use crate::Config;
use frame_support::{impl_outer_event, impl_outer_origin, parameter_types, weights::Weight};
use frame_system as system;
use sp_runtime::{
    testing::{Header, H256},
    traits::{BlakeTwo256, Convert, IdentityLookup},
    AccountId32, Perbill,
};

impl_outer_origin! {
    pub enum Origin for Test {}
}

// Configure a mock runtime to test the pallet.

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Test;

impl Convert<Weight, contract::BalanceOf<Self>> for Test {
    fn convert(w: Weight) -> contract::BalanceOf<Self> {
        w as u128
    }
}

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const MaximumBlockWeight: Weight = 1024;
    pub const MaximumBlockLength: u32 = 2 * 1024;
    pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
}

impl system::Config for Test {
    type BaseCallFilter = ();
    type Origin = Origin;
    type Call = ();
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = AccountId32;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = ();
    type BlockHashCount = BlockHashCount;
    type DbWeight = ();
    type Version = ();
    type PalletInfo = ();
    type AccountData = pallet_balances::AccountData<u128>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type BlockWeights = ();
    type BlockLength = ();
}

parameter_types! {
    pub const MinimumPeriod: u64 = 1;
    pub static ExistentialDeposit: u128 = 0;
}

impl pallet_timestamp::Config for Test {
    type Moment = u64;
    type OnTimestampSet = ();
    type MinimumPeriod = MinimumPeriod;
    type WeightInfo = ();
}

impl_outer_event! {
    pub enum MetaEvent for Test {
        frame_system<T>,
        pallet_balances<T>,
        contract<T>,
    }
}

impl Config for Test {
    type Event = ();
}

type System = system::Module<Test>;

impl pallet_balances::Config for Test {
    type MaxLocks = ();
    type Balance = u128;
    type Event = ();
    type DustRemoval = ();
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type WeightInfo = ();
}

pub type Balances = pallet_balances::Module<Test>;
type Timestamp = pallet_timestamp::Module<Test>;
type Randomness = pallet_randomness_collective_flip::Module<Test>;

parameter_types! {
    pub const SignedClaimHandicap: u64 = 2;
    pub const TombstoneDeposit: u128 = 16;
    pub const StorageSizeOffset: u32 = 8;
    pub const RentByteFee: u128 = 4;
    pub const RentDepositOffset: u128 = 10_000;
    pub const SurchargeReward: u128 = 150;
    pub const MaxDepth: u32 = 100;
    pub const MaxValueSize: u32 = 16_384;
}

impl contract::Config for Test {
    type Time = Timestamp;
    type Randomness = Randomness;
    type Currency = Balances;
    type Event = ();
    type RentPayment = ();
    type SignedClaimHandicap = SignedClaimHandicap;
    type TombstoneDeposit = TombstoneDeposit;
    type StorageSizeOffset = StorageSizeOffset;
    type RentByteFee = RentByteFee;
    type RentDepositOffset = RentDepositOffset;
    type SurchargeReward = SurchargeReward;
    type MaxDepth = MaxDepth;
    type MaxValueSize = MaxValueSize;
    type WeightPrice = Self;
    type WeightInfo = ();
    type ChainExtension = jupiter_chain_extension::JupiterExt;
}

pub type Contracts = contract::Module<Test>;

pub const ALICE: AccountId32 = AccountId32::new([1u8; 32]);

pub struct ExtBuilder {
    existential_deposit: u128,
}
impl Default for ExtBuilder {
    fn default() -> Self {
        Self {
            existential_deposit: 1,
        }
    }
}

impl ExtBuilder {
    pub fn set_associated_consts(&self) {
        EXISTENTIAL_DEPOSIT.with(|v| *v.borrow_mut() = self.existential_deposit);
    }

    pub fn existential_deposit(mut self, existential_deposit: u128) -> Self {
        self.existential_deposit = existential_deposit;
        self
    }

    pub fn build(self) -> sp_io::TestExternalities {
        self.set_associated_consts();
        let mut t = frame_system::GenesisConfig::default()
            .build_storage::<Test>()
            .unwrap();
        pallet_balances::GenesisConfig::<Test> { balances: vec![] }
            .assimilate_storage(&mut t)
            .unwrap();

        let mut current_schedule = contract::Schedule::<Test>::default();
        current_schedule.enable_println = true;
        contract::GenesisConfig::<Test> {
            current_schedule,
        }
        .assimilate_storage(&mut t)
        .unwrap();
        let mut ext = sp_io::TestExternalities::new(t);
        ext.execute_with(|| System::set_block_number(1));
        ext
    }
}
