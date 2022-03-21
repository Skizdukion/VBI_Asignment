//! Benchmarking setup for pallet-template

use super::*;

#[allow(unused)]
use crate::Pallet as Kitties;
use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_system::RawOrigin;

const ED_MULTIPLIER: u32 = 10;

benchmarks! {
	create_kitty {
		let s in 0 .. 100;
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))
	// verify {
	// 	assert_eq!(Something::<T>::get(), Some(s));
	// }
	// set_price {
	// 	let s in 0 .. 100;
	// 	let existential_deposit = Kitties::Currency::ExistentialDeposit::get();
	// 	let caller: T::AccountId = whitelisted_caller();
	// 	let kitties_list = Kitties::kitties_owned(caller);
	// 	let kt1_hash = kitties_list.get(0).unwrap();
	// 	let balance = existential_deposit.saturating_mul(ED_MULTIPLIER.into());
	// 	let transfer_amount = existential_deposit.saturating_mul((ED_MULTIPLIER - 1).into()) + 1u32.into();
	// }: _(RawOrigin::Signed(caller), *kt1_hash, Some(transfer_amount))
	impl_benchmark_test_suite!(Kitties, crate::mock::new_test_ext(), crate::mock::Test);
}
