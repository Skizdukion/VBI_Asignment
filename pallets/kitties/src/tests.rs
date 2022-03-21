use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok, log};

#[test]
fn create_kitty_exceed_max_value() {
	new_test_ext().execute_with(|| {
		// log::info!("testing create kitty");
		for i in 1..100 {
			if i <= KittiesPallet::get_max_kitties() {
				assert_ok!(KittiesPallet::create_kitty(Origin::signed(1)));
			} else {
				assert_noop!(
					KittiesPallet::create_kitty(Origin::signed(1)),
					Error::<Test>::ExceedMaxKittyOwned
				);
			}
		}
	});
}

#[test]
fn create_and_transfer_kitty() {
	new_test_ext().execute_with(|| {
		assert_ok!(KittiesPallet::create_kitty(Origin::signed(1)));
		// assert_ok!(KittiesPallet::create_kitty(Origin::signed(1)));
		let kitties_list_acc1 = KittiesPallet::kitties_owned(1);
		assert_eq!(kitties_list_acc1.len(), 1);
		let kt1_hash = match kitties_list_acc1.get(0) {
			Some(t) => t,
			None => panic!("Could not find just-created-kitty-hash"),
		};
		assert_noop!(
			KittiesPallet::transfer(Origin::signed(2), 1, *kt1_hash),
			Error::<Test>::NotKittyOwner
		);
		assert_ok!(KittiesPallet::transfer(Origin::signed(1), 2, *kt1_hash));
		assert_eq!(KittiesPallet::is_kitty_owner(kt1_hash, &1).unwrap(), false);
		assert_ok!(KittiesPallet::is_kitty_owner(kt1_hash, &2));
	});
}

#[test]
fn buy_just_created_kitty() {
	new_test_ext().execute_with(|| {
		assert_ok!(KittiesPallet::create_kitty(Origin::signed(1)));
		let kitties_list_acc1 = KittiesPallet::kitties_owned(1);
		assert_eq!(kitties_list_acc1.len(), 1);
		let just_created_kitty_hash = match kitties_list_acc1.first() {
			Some(t) => t,
			None => panic!("Could not find just-created-kitty-hash"),
		};
		// set price
		assert_ok!(KittiesPallet::set_price(
			Origin::signed(1),
			*just_created_kitty_hash,
			Some(150)
		));
		let just_created_kitty = match KittiesPallet::kitties(just_created_kitty_hash) {
			Some(t) => t,
			None => panic!("Could not find just-created-kitty"),
		};
		assert_eq!(just_created_kitty.price, Some(150));
		// buy
		assert_noop!(
			KittiesPallet::buy_kitty(Origin::signed(2), *just_created_kitty_hash, 100),
			Error::<Test>::KittyBidPriceTooLow
		);
		assert_ok!(KittiesPallet::buy_kitty(Origin::signed(2), *just_created_kitty_hash, 200));
		assert_eq!(KittiesPallet::kitties_owned(1).len(), 0);
		assert_eq!(KittiesPallet::kitties_owned(2).len(), 1);
		let just_created_kitty = match KittiesPallet::kitties(just_created_kitty_hash) {
			Some(t) => t,
			None => panic!("Could not find just-created-kitty"),
		};
		assert_eq!(just_created_kitty.owner, 2);
	});
}

#[test]
fn breed_kitty() {
	new_test_ext().execute_with(|| {
		assert_ok!(KittiesPallet::create_kitty(Origin::signed(1)));
		assert_ok!(KittiesPallet::create_kitty(Origin::signed(1)));
		let kitties_list_acc1 = KittiesPallet::kitties_owned(1);
		assert_eq!(kitties_list_acc1.len(), 2);
		let kt1_hash = match kitties_list_acc1.get(0) {
			Some(t) => t,
			None => panic!("Could not find just-created-kitty-hash"),
		};
		let kt2_hash = match kitties_list_acc1.get(1) {
			Some(t) => t,
			None => panic!("Could not find just-created-kitty-hash"),
		};
		assert_noop!(
			KittiesPallet::breed_kitty(Origin::signed(2), *kt1_hash, *kt2_hash),
			Error::<Test>::NotKittyOwner
		);
		assert_ok!(KittiesPallet::breed_kitty(Origin::signed(1), *kt1_hash, *kt2_hash));
	});
}

#[test]
fn buy_breed_kitty_not_set_price() {
	new_test_ext().execute_with(|| {
		assert_ok!(KittiesPallet::create_kitty(Origin::signed(1)));
		assert_ok!(KittiesPallet::create_kitty(Origin::signed(1)));
		let kitties_list_acc1 = KittiesPallet::kitties_owned(1);
		assert_eq!(kitties_list_acc1.len(), 2);
		let kt1_hash = match kitties_list_acc1.get(0) {
			Some(t) => t,
			None => panic!("Could not find just-created-kitty-hash"),
		};
		let kt2_hash = match kitties_list_acc1.get(1) {
			Some(t) => t,
			None => panic!("Could not find just-created-kitty-hash"),
		};
		assert_ok!(KittiesPallet::breed_kitty(Origin::signed(1), *kt1_hash, *kt2_hash));
		let kitties_list_acc1 = KittiesPallet::kitties_owned(1);
		let breeded_kitty = kitties_list_acc1.get(2).expect("not found breeded kitty");
		assert_noop!(
			KittiesPallet::buy_kitty(Origin::signed(2), *breeded_kitty, 100),
			Error::<Test>::KittyNotForSale
		);
		assert_noop!(
			KittiesPallet::buy_kitty(Origin::signed(1), *breeded_kitty, 100),
			Error::<Test>::BuyerIsKittyOwner
		);
	});
}
