#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::dispatch::DispatchResult;
pub use pallet::*;

// use sp_runtime::traits::Saturating;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn something1)]
	pub type Something1<T> = StorageValue<_, u32>;

	#[pallet::storage]
	#[pallet::getter(fn something2)]
	pub type Something2<T> = StorageValue<_, u32>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		SomethingStored(u32, T::AccountId),
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
		pub fn set_something_1(origin: OriginFor<T>, something: u32) -> DispatchResult {
			let who = ensure_signed(origin)?;

			<Something1<T>>::put(something);

			Self::deposit_event(Event::SomethingStored(something, who));

			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn set_something_2(origin: OriginFor<T>, something: u32) -> DispatchResult {
			let who = ensure_signed(origin)?;

			<Something2<T>>::put(something);

			Self::deposit_event(Event::SomethingStored(something, who));

			Ok(())
		}
	}
}

impl<T: Config> Pallet<T>{
	pub fn update_storage1(value: u32) -> DispatchResult{
		<Something1<T>>::put(value);
		Ok(())
	}

	pub fn update_storage2(value: u32) -> DispatchResult{
		<Something2<T>>::put(value);
		Ok(())
	}

	pub fn sum_storage() -> u32{
		<Something1<T>>::get().unwrap() + <Something2<T>>::get().unwrap()
	}
}

pub trait DoSomeActivity{
	fn increase_value(value: u32) -> u32;
}

impl <T:Config> DoSomeActivity for Pallet<T> {
	fn increase_value(value: u32) -> u32{
		value.saturating_add(5)
	}
}