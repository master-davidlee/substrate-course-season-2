#[test]
fn use_too_lang_claim() {
	new_test_ext().execute_with(|| {
		let proof = vec![1,2,3,4,5,6,7,8,9,10,11];
		assert_noop!(TemplateModule::new_claim(Origin::signed(1), proof.clone()), 
			Error::<Test>::ClaimTooLang
	);
	})
}
