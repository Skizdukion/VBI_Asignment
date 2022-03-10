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

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_template::Config{
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
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
		// SomethingStored(u32, T::AccountId),
		AccessStore(u32),
	}

	#[pallet::error]
	pub enum Error<T> {
		NoneValue,

		StorageOverflow,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn access_storage_template_pallet(origin: OriginFor<T>) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			let data = pallet_template::Pallet::<T>::something().unwrap();

			// Self::deposit_event(Event::SomethingStored(something));

			Self::deposit_event(Event::AccessStore(data));

			Ok(())
		}
	}
}