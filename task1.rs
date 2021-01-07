use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};

#[test]
fn new_claim_worked() {
	new_test_ext().execute_with(|| {
		let proof = vec![1,2];
		assert_ok!(TemplateModule::new_claim(Origin::signed(1), proof.clone()));
		// Read pallet storage and assert an expected result.
		assert_eq!(TemplateModule::get_claim(&proof), (1, frame_system::Module::<Test>::block_number()));
	})
}

#[test]
fn claim_twice() {
	new_test_ext().execute_with(|| {
		let proof = vec![1,2];
		let _= TemplateModule::new_claim(Origin::signed(1), proof.clone());
		assert_noop!(TemplateModule::new_claim(Origin::signed(1), proof.clone()),
			Error::<Test>::ClaimAlreadyExisted);

	})
}

#[test]
fn revoke_claim_worked() {
	new_test_ext().execute_with(|| {
		let proof = vec![1,2];
		assert_ok!(TemplateModule::new_claim(Origin::signed(1), proof.clone()));
		assert_ok!(TemplateModule::revoke_claim(Origin::signed(1), proof.clone()));
		// Read pallet storage and assert an expected result.
		assert_eq!(TemplateModule::get_claim(&proof), (0,0));
	})
}

#[test]
fn revoke_after_revoked() {
	new_test_ext().execute_with(|| {
		let proof = vec![1,2];
		assert_noop!(TemplateModule::revoke_claim(Origin::signed(1),proof.clone()),
			Error::<Test>::ClaimNotExisted
	);

	})
}



#[test]
fn revoke_use_diff_account() {
	new_test_ext().execute_with(|| {
		let proof = vec![1,2];
		let _ = TemplateModule::new_claim(Origin::signed(1), proof.clone());
		assert_noop!(TemplateModule::revoke_claim(Origin::signed(2), proof.clone()),
			Error::<Test>::NotTheOwner
	);
	})
}

#[test]
fn transfer_worked() {
	new_test_ext().execute_with(|| {
		let proof = vec![1,2];
		let account = 2;
		let _ = TemplateModule::new_claim(Origin::signed(1), proof.clone());
		assert_ok!(TemplateModule::transfer_claim(Origin::signed(1), proof.clone(),account ));
		assert_eq!(TemplateModule::get_claim(&proof), (2, frame_system::Module::<Test>::block_number()));

	})
}

#[test]
fn transfer_other_claim() {
	new_test_ext().execute_with(|| {
		let proof = vec![1,2];
		let _ = TemplateModule::new_claim(Origin::signed(1), proof.clone());
		assert_noop!(TemplateModule::transfer_claim(Origin::signed(2),proof.clone(), 3),
			Error::<Test>::NotTheOwner
	);
	})
}
