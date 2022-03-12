#![cfg_attr(not(feature = "std"), no_std)]

pub mod weights;

use frame_support::traits::{OnTimestampSet, Time, UnixTime};
use sp_runtime::traits::{AtLeast32Bit, SaturatedConversion, Scale, Zero};
use sp_std::{cmp, result};
use sp_timestamp::{InherentError, InherentType, INHERENT_IDENTIFIER};
use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;
pub use weights::WeightInfo;
use sp_arithmetic;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Moment: Parameter
			+ Default
			+ AtLeast32Bit
			+ Scale<Self::BlockNumber, Output = Self::Moment>
			+ Copy
			+ MaxEncodedLen
			+ scale_info::StaticTypeInfo;

		type OnTimestampSet: OnTimestampSet<Self::Moment>;

		#[pallet::constant]
		type MinimumPeriod: Get<Self::Moment>;

		type WeightInfo: WeightInfo;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(PhantomData<T>);

	#[pallet::storage]
	#[pallet::getter(fn now)]
	pub type Now<T: Config> = StorageValue<_, T::Moment, ValueQuery>;

	#[pallet::storage]
	pub(super) type DidUpdate<T: Config> = StorageValue<_, bool, ValueQuery>;

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_initialize(_n: BlockNumberFor<T>) -> Weight {
			T::WeightInfo::on_finalize()
		}

		fn on_finalize(_n: BlockNumberFor<T>) {
			assert!(DidUpdate::<T>::take(), "Timestamp must be updated once in the block");
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight((
			T::WeightInfo::set(),
			DispatchClass::Mandatory
		))]
		pub fn set(origin: OriginFor<T>, #[pallet::compact] now: T::Moment) -> DispatchResult {
			ensure_none(origin)?;
			assert!(!DidUpdate::<T>::exists(), "Timestamp must be updated only once in the block");
			let prev = Self::now();
			assert!(
				prev.is_zero() || now >= prev + T::MinimumPeriod::get(),
				"Timestamp must increment by at least <MinimumPeriod> between sequential blocks"
			);
			Now::<T>::put(now);
			DidUpdate::<T>::put(true);

			<T::OnTimestampSet as OnTimestampSet<_>>::on_timestamp_set(now);

			Ok(())
		}
	}

	#[pallet::inherent]
	impl<T: Config> ProvideInherent for Pallet<T> {
		type Call = Call<T>;
		type Error = InherentError;
		const INHERENT_IDENTIFIER: InherentIdentifier = INHERENT_IDENTIFIER;

		fn create_inherent(data: &InherentData) -> Option<Self::Call> {
			let inherent_data = data
				.get_data::<InherentType>(&INHERENT_IDENTIFIER)
				.expect("Timestamp inherent data not correctly encoded")
				.expect("Timestamp inherent data must be provided");
			let data = (*inherent_data).saturated_into::<T::Moment>();

			let next_time = cmp::max(data, Self::now() + T::MinimumPeriod::get());
			Some(Call::set { now: next_time.into() })
		}

		fn check_inherent(
			call: &Self::Call,
			data: &InherentData,
		) -> result::Result<(), Self::Error> {
			const MAX_TIMESTAMP_DRIFT_MILLIS: sp_timestamp::Timestamp =
				sp_timestamp::Timestamp::new(30 * 1000);

			let t: u64 = match call {
				Call::set { ref now } => now.clone().saturated_into::<u64>(),
				_ => return Ok(()),
			};

			let data = data
				.get_data::<InherentType>(&INHERENT_IDENTIFIER)
				.expect("Timestamp inherent data not correctly encoded")
				.expect("Timestamp inherent data must be provided");

			let minimum = (Self::now() + T::MinimumPeriod::get()).saturated_into::<u64>();
			if t > *(data + MAX_TIMESTAMP_DRIFT_MILLIS) {
				Err(InherentError::TooFarInFuture)
			} else if t < minimum {
				Err(InherentError::ValidAtTimestamp(minimum.into()))
			} else {
				Ok(())
			}
		}

		fn is_inherent(call: &Self::Call) -> bool {
			matches!(call, Call::set { .. })
		}
	}
}

impl<T: Config> Pallet<T> {
	pub fn get() -> T::Moment {
		Self::now()
	}

	#[cfg(any(feature = "runtime-benchmarks", feature = "std"))]
	pub fn set_timestamp(now: T::Moment) {
		Now::<T>::put(now);
	}
}

impl<T: Config> Time for Pallet<T> {
	type Moment = T::Moment;

	fn now() -> Self::Moment {
		Self::now()
	}
}

pub trait MyCustomTime{
	type Moment: sp_arithmetic::traits::AtLeast32Bit + Parameter + Default + Copy + MaxEncodedLen;

	fn now() -> Self::Moment;
}

impl <T:Config> MyCustomTime for Pallet<T> {
	type Moment = T::Moment;

	fn now() -> Self::Moment {
		Self::now()
	}
}

impl<T: Config> UnixTime for Pallet<T> {
	fn now() -> core::time::Duration {
		let now = Self::now();
		sp_std::if_std! {
			if now == T::Moment::zero() {
				log::error!(
					target: "runtime::timestamp",
					"`pallet_timestamp::UnixTime::now` is called at genesis, invalid value returned: 0",
				);
			}
		}
		core::time::Duration::from_millis(now.saturated_into::<u64>())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate as pallet_timestamp;

	use frame_support::{
		assert_ok, parameter_types,
		traits::{ConstU32, ConstU64},
	};
	use sp_core::H256;
	use sp_io::TestExternalities;
	use sp_runtime::{
		testing::Header,
		traits::{BlakeTwo256, IdentityLookup},
	};

	pub fn new_test_ext() -> TestExternalities {
		let t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();
		TestExternalities::new(t)
	}

	type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
	type Block = frame_system::mocking::MockBlock<Test>;

	frame_support::construct_runtime!(
		pub enum Test where
			Block = Block,
			NodeBlock = Block,
			UncheckedExtrinsic = UncheckedExtrinsic,
		{
			System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
			Timestamp: pallet_timestamp::{Pallet, Call, Storage, Inherent},
		}
	);

	parameter_types! {
		pub BlockWeights: frame_system::limits::BlockWeights =
			frame_system::limits::BlockWeights::simple_max(1024);
	}
	impl frame_system::Config for Test {
		type BaseCallFilter = frame_support::traits::Everything;
		type BlockWeights = ();
		type BlockLength = ();
		type DbWeight = ();
		type Origin = Origin;
		type Index = u64;
		type BlockNumber = u64;
		type Call = Call;
		type Hash = H256;
		type Hashing = BlakeTwo256;
		type AccountId = u64;
		type Lookup = IdentityLookup<Self::AccountId>;
		type Header = Header;
		type Event = Event;
		type BlockHashCount = ConstU64<250>;
		type Version = ();
		type PalletInfo = PalletInfo;
		type AccountData = ();
		type OnNewAccount = ();
		type OnKilledAccount = ();
		type SystemWeightInfo = ();
		type SS58Prefix = ();
		type OnSetCode = ();
		type MaxConsumers = ConstU32<16>;
	}

	impl Config for Test {
		type Moment = u64;
		type OnTimestampSet = ();
		type MinimumPeriod = ConstU64<5>;
		type WeightInfo = ();
	}

	#[test]
	fn timestamp_works() {
		new_test_ext().execute_with(|| {
			Timestamp::set_timestamp(42);
			assert_ok!(Timestamp::set(Origin::none(), 69));
			assert_eq!(Timestamp::now(), 69);
		});
	}

	#[test]
	#[should_panic(expected = "Timestamp must be updated only once in the block")]
	fn double_timestamp_should_fail() {
		new_test_ext().execute_with(|| {
			Timestamp::set_timestamp(42);
			assert_ok!(Timestamp::set(Origin::none(), 69));
			let _ = Timestamp::set(Origin::none(), 70);
		});
	}

	#[test]
	#[should_panic(
		expected = "Timestamp must increment by at least <MinimumPeriod> between sequential blocks"
	)]
	fn block_period_minimum_enforced() {
		new_test_ext().execute_with(|| {
			Timestamp::set_timestamp(42);
			let _ = Timestamp::set(Origin::none(), 46);
		});
	}
}
