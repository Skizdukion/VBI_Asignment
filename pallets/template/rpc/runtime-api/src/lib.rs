#![cfg_attr(not(feature = "std"), no_std)]

use codec::Codec;

// pub use pallet_transaction_payment::{FeeDetails, InclusionFee, RuntimeDispatchInfo};

sp_api::decl_runtime_apis! {
	pub trait SumStorageApi {
		fn get_sum() -> u32;
	}
}