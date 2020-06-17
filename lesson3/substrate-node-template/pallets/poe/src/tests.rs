// Tests to be written here

use crate::{Error, mock::*};
use super::*;  //引入所有存储项依赖，也就是lib.rs里面暴露的所有内容
use frame_support::{assert_ok, assert_noop};

//创建存证错误
#[test]
fn create_claim_works(){
    //初始化存储环境以及必要的功能组件
    new_test_ext().execute_with(||{
        let claim=vec![0,1];
        //断言claim是正确的,origin:交易发送方
        assert_ok!(PoeModule::create_claim(Origin::signed(1),claim.clone()));
        //断言存储项里面的内容是预设
        assert_eq!(Proofs::<Test>::get(&claim),(1,system::Module::<Test>::block_number()));
    })

}

//创建存证->claim存在错误
#[test]
fn create_cliam_failed_when_cliam_already_exist(){
    new_test_ext().execute_with(||{
        let claim=vec![0,1];
        let _=PoeModule::create_claim(Origin::signed(1),claim.clone());
        assert_noop!(
            PoeModule::create_claim(Origin::signed(1),claim.clone()),
            Error::<Test>::ProofAlreadyExist
          );
    })
}


//创建存证->claim长度错误
#[test]
fn create_cliam_failed_when_cliam_is_too_long(){
    new_test_ext().execute_with(||{
        let claim=vec![0,1,2,3,4,5,6];
        assert_noop!(
            PoeModule::create_claim(Origin::signed(1),claim.clone()),
            Error::<Test>::ProofTooLong
          );
    })
}

//撤销存证错误
#[test]
fn revoke_claim_works(){
    new_test_ext().execute_with(||{
        let claim=vec![0,1];
        let _=PoeModule::create_claim(Origin::signed(1),claim.clone());
        assert_ok!(PoeModule::revoke_claim(Origin::signed(1),claim.clone()));
    })
}


//撤销存证->claim不存在错误
#[test]
fn revoke_claim_failed_when_is_not_exist(){
    new_test_ext().execute_with(||{
        let claim=vec![0,1];
        assert_noop!(
            PoeModule::revoke_claim(Origin::signed(1),claim.clone()),
            Error::<Test>::ClaimNotExist
        );
    })
}

//撤销存证->发送交易方不一致错误
#[test]
fn revoke_claim_failed_with_wrong_owner(){
    new_test_ext().execute_with(||{
        let claim=vec![0,1];
        let _=PoeModule::create_claim(Origin::signed(1),claim.clone());
        assert_noop!(
            PoeModule::revoke_claim(Origin::signed(2),claim.clone()),
            Error::<Test>::NotClaimOwner
        );
    })
}


//转移存证错误
#[test]
fn transfer_claim_works(){
    new_test_ext().execute_with(||{
        let claim=vec![0,1];
        let _=PoeModule::create_claim(Origin::signed(1),claim.clone());
        assert_ok!(PoeModule::transfer_claim(Origin::signed(1),claim.clone(),2));
    })
}

//转移存证->claim不存在错误
#[test]
fn transfer_claim_failed_when_is_not_exist(){
    new_test_ext().execute_with(||{
        let claim=vec![0,1];
        assert_noop!(
            PoeModule::transfer_claim(Origin::signed(1),claim.clone(),2),
            Error::<Test>::ClaimNotExist
        );
    })
}

//转移存证->转移交易方不一致错误
#[test]
fn transfer_claim_failed_with_wrong_owner(){
    new_test_ext().execute_with(||{
        let claim=vec![0,1];
        let _=PoeModule::create_claim(Origin::signed(1),claim.clone());
        assert_noop!(
            PoeModule::transfer_claim(Origin::signed(2),claim.clone(),2),
            Error::<Test>::NotClaimOwner
        );
    })
}

