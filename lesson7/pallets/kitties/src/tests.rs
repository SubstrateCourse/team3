// Tests to be written here
use crate::{Error, mock::*};
use super::*;
use frame_support::{assert_ok, assert_noop};
// use frame_system::Origin;

// test cases for create_claim
// #[test]
// fn create_claim_works() {
//     new_test_ext().execute_with(|| {
//         let claim = vec![0, 1];
//         assert_ok!(PoeModule::create_claim(Origin::signed(1),claim.clone()));
//         assert_eq!(Proofs::<Test>::get(&claim),(1,system::Module::<Test>::block_number()));
//     })
// }
#[test]
fn owned_kitties_can_append_values() {
		new_test_ext().execute_with(|| {
			OwnedKittiesTest::append(&0, 1);

			assert_eq!(OwnedKittiesTest::get(&(0, None)), Some(KittyLinkedItem {
				prev: Some(1),
				next: Some(1),
			}));

			assert_eq!(OwnedKittiesTest::get(&(0, Some(1))), Some(KittyLinkedItem {
				prev: None,
				next: None,
			}));

			OwnedKittiesTest::append(&0, 2);

			assert_eq!(OwnedKittiesTest::get(&(0, None)), Some(KittyLinkedItem {
				prev: Some(2),
				next: Some(1),
			}));

			assert_eq!(OwnedKittiesTest::get(&(0, Some(1))), Some(KittyLinkedItem {
				prev: None,
				next: Some(2),
			}));

			assert_eq!(OwnedKittiesTest::get(&(0, Some(2))), Some(KittyLinkedItem {
				prev: Some(1),
				next: None,
			}));

			OwnedKittiesTest::append(&0, 3);

			assert_eq!(OwnedKittiesTest::get(&(0, None)), Some(KittyLinkedItem {
				prev: Some(3),
				next: Some(1),
			}));

			assert_eq!(OwnedKittiesTest::get(&(0, Some(1))), Some(KittyLinkedItem {
				prev: None,
				next: Some(2),
			}));

			assert_eq!(OwnedKittiesTest::get(&(0, Some(2))), Some(KittyLinkedItem {
				prev: Some(1),
				next: Some(3),
			}));

			assert_eq!(OwnedKittiesTest::get(&(0, Some(3))), Some(KittyLinkedItem {
				prev: Some(2),
				next: None,
			}));
		});
}

#[test]
fn owned_kitties_can_remove_values() {
	// 作业
	OwnedKittiesTest::append(&0, 1);

	assert_eq!(OwnedKittiesTest::remove(&0,1), Some(KittyLinkedItem {
		prev: None,
		next: None,
	}));
}