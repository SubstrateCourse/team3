// Tests to be written here

use crate::{mock::*, Error};
use frame_support::assert_ok;

#[test]
fn test_offchain() {
    let (mut t, _pool_state, _offchain_state) = ExtBuilder::build();
    t.execute_with(|| {
        assert_ok!(TemplateModule::store_eth_price(Ok(245)));
    });
}

#[test]
fn test_offchain_error() {
    let (mut t, _pool_state, _offchain_state) = ExtBuilder::build();
    t.execute_with(|| {
        let r = TemplateModule::store_eth_price(Err(Error::<Test>::HttpError));
        assert!(r.is_err());
    });
}
