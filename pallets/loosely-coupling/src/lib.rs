#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

// #[cfg(test)]
// mod mock;

// #[cfg(test)]
// mod tests;

// #[cfg(feature = "runtime-benchmarks")]
// mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use pallet_template::DoSomeActivity;

	#[pallet::config]
	pub trait Config: frame_system::Config{
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type DoSome: DoSomeActivity;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn something)]

	pub type Something<T> = StorageValue<_, u32>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		IncreaseSuccess(u32),
	}

	#[pallet::error]
	pub enum Error<T> {
		NoneValue,

		StorageOverflow,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn increase_value(origin: OriginFor<T>, value: u32) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			let result = T::DoSome::increase_value(value);

			Self::deposit_event(Event::IncreaseSuccess(result));

			Ok(())
		}
	}
}
