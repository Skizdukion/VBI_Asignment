#![cfg_attr(not(feature = "std"), no_std)]

sp_api::decl_runtime_apis! {
	pub trait KittiesStorageApi{
		fn get_total_kitties() -> u64;
		// fn get_all_kitties() -> String;
		// fn get_kitties_info() -> String;
		// fn create_kitty_rpc()-> DispatchResult;
		// fn get_info() -> pallet_kitties::Kitty<_>;
	}
}
