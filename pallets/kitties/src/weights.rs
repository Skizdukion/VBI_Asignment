#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight},};
use sp_std::marker::PhantomData;

pub trait KittiesWeightInfo{
	fn create_kitty() -> Weight;
}

/// Weight functions for `pallet_kitties`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> KittiesWeightInfo for WeightInfo<T> {
	// Storage: RandomnessCollectiveFlip RandomMaterial (r:1 w:0)
	// Storage: Kitties KittyCnt (r:1 w:1)
	// Storage: Kitties KittiesOwned (r:1 w:1)
	// Storage: Kitties Kitties (r:0 w:1)
	fn create_kitty() -> Weight {
		(35_901_000 as Weight)
			// Standard Error: 30_000
			.saturating_add((21_000 as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
}

impl KittiesWeightInfo for (){
	fn create_kitty() -> Weight {
		(35_901_000 as Weight)
			// Standard Error: 30_000
			.saturating_add((21_000 as Weight))
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
}
