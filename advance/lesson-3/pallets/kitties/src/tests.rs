use super::*;
use crate::mock::new_test_ext;
use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};
use frame_support::{
    pallet_prelude::*,
    traits::{Currency, ExistenceRequirement, ReservableCurrency},
};
#[test]
fn it_works_for_default_value() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        run_to_block(2);
    });
}
#[test]
fn it_works_create_kitty() {
    new_test_ext().execute_with(|| {
        run_to_block(1);

        let caller = RuntimeOrigin::signed(0);
        assert_eq!(crate::NextKittyId::<Test>::get(), 0);
        assert_ok!(PalletKitties::create(caller));
        assert_eq!(crate::NextKittyId::<Test>::get(), 1);
    });
}

#[test]
fn breed_works() {
    new_test_ext().execute_with(|| {
        let caller = RuntimeOrigin::signed(0);
        assert_eq!(crate::NextKittyId::<Test>::get(), 0);
        assert_ok!(PalletKitties::create(caller.clone()));

        assert_eq!(crate::NextKittyId::<Test>::get(), 1);
        assert_ok!(PalletKitties::create(caller.clone()));

        assert_ok!(PalletKitties::breed(caller.clone(), 0, 1));
    });
}

#[test]
fn transfer_works() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        let from = RuntimeOrigin::signed(0);
        let to = 1;
        let kitty_id = 0;
        assert_eq!(crate::NextKittyId::<Test>::get(), 0);
        assert_ok!(PalletKitties::create(from.clone()));
        assert_eq!(crate::NextKittyId::<Test>::get(), 1);

        assert_ok!(PalletKitties::transfer(from, kitty_id, to));
        assert_eq!(crate::KittyOwner::<Test>::get(kitty_id), Some(to));
    });
}

#[test]
fn sale_works() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        let caller = RuntimeOrigin::signed(0);
        let kitty_id = 0;
        assert_eq!(crate::NextKittyId::<Test>::get(), 0);
        assert_ok!(PalletKitties::create(caller.clone()));
        assert_eq!(crate::NextKittyId::<Test>::get(), 1);
        assert_ok!(PalletKitties::sale(caller, kitty_id, 5, 1000));
        run_to_block(2);
        assert_eq!(crate::KittySale::<Test>::get(kitty_id), Some((0, 5, 1000)));
    });
}

#[test]
fn bid_works() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        let caller = RuntimeOrigin::signed(0);
        let kitty_id = 0;
        assert_eq!(crate::NextKittyId::<Test>::get(), 0);
        assert_ok!(PalletKitties::create(caller.clone()));
        assert_eq!(crate::NextKittyId::<Test>::get(), 1);
        assert_ok!(PalletKitties::sale(caller.clone(), kitty_id, 5, 1000));
        run_to_block(2);
        assert_eq!(crate::KittySale::<Test>::get(kitty_id), Some((0, 5, 1000)));

        run_to_block(3);
        let bidder = RuntimeOrigin::signed(2);

        assert_ok!(PalletKitties::bid(bidder, 0, 1001));
        let balance2 = <Test as Config>::Currency::reserved_balance(2);
        assert_eq!(balance2, 1001);
    });
}
