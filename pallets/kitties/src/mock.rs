use crate as pallet_kitties;
use frame_support::{traits::{ConstU16, ConstU64, ConstU32, ConstU128}, parameter_types};
use frame_system as system;
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		KittiesPallet: pallet_kitties::{Pallet, Call, Storage, Event<T>},
		Balance: pallet_balances::{Pallet, Call, Storage, Event<T>, Config<T>},
		Timestamp: pallet_timestamp::{Pallet, Call, Storage},
		RandomnessCollectiveFlip: pallet_randomness_collective_flip::{Pallet},
	}
);

impl pallet_randomness_collective_flip::Config for Test {}

impl pallet_balances::Config for Test {
	type MaxLocks = ConstU32<50>;
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	/// The type for recording an account's balance.
	type Balance = u128;
	/// The ubiquitous event type.
	type Event = Event;
	type DustRemoval = ();
	type ExistentialDeposit = ConstU128<500>;
	type AccountStore = System;
	type WeightInfo = ();
}

impl pallet_timestamp::Config for Test {
	/// A timestamp: milliseconds since the unix epoch.
	type Moment = u64;
	type OnTimestampSet = ();
	type MinimumPeriod = ();
	type WeightInfo = ();
}

impl system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = ConstU64<250>;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<u128>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ConstU16<42>;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

parameter_types! {
	pub const MaxKittiesOwned : u32 = 20;
}

impl pallet_kitties::Config for Test {
	type Event = Event;
	type KittyRandomness = RandomnessCollectiveFlip;
	type Currency = Balance;
	type MaxKittyOwned = MaxKittiesOwned;
	type WeightInfo = ();
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    let mut t = system::GenesisConfig::default().build_storage::<Test>().unwrap();
    pallet_balances::GenesisConfig::<Test> {
        balances: vec![
            (1, 1_000_000_000_000),
            (2, 1_000_000_000_000),
            (3, 1_000_000_000_000),
            (4, 1_000_000_000_000),
            (5, 1_000_000_000_000),
            (6, 1_000_000_000_000),
        ],
    }
    .assimilate_storage(&mut t)
    .unwrap();

    let mut ext = sp_io::TestExternalities::new(t);
    ext.execute_with(|| System::set_block_number(1));
    ext
}