use crate::*;

use std::fmt::Debug;
use std::str::FromStr;

def_id!(AccountId, "acct");
def_id!(UserId, "user");
def_id!(ChargeId, "ch" | "py");
def_id!(
    enum UserOrAccount {
        Account(AccountId),
        User(UserId),
    }
);
def_id!(
    enum UserOrCharge {
        Charge(ChargeId),
        User(UserId),
    }
);

#[test]
fn test_prefix() {
    let account_id = AccountId::generate().unwrap();
    assert_eq!(account_id.prefix(), "acct");
    assert_eq!(AccountId::prefixes(), ["acct"]);
    assert!(AccountId::is_valid_prefix("acct"));
    assert!(!AccountId::is_valid_prefix("acct_"));

    let user_id = UserId::generate().unwrap();
    assert_eq!(user_id.prefix(), "user");
    assert_eq!(UserId::prefixes(), ["user"]);
    assert!(UserId::is_valid_prefix("user"));
    assert!(!UserId::is_valid_prefix("user_"));

    let charge_id = ChargeId::generate().unwrap();
    assert_eq!(charge_id.prefix(), "ch");
    assert_eq!(ChargeId::prefixes(), ["ch", "py"]);
    assert!(ChargeId::is_valid_prefix("ch"));
    assert!(ChargeId::is_valid_prefix("py"));
    assert!(!AccountId::is_valid_prefix("ch_"));
    assert!(!AccountId::is_valid_prefix("py_"));
}

#[test]
fn test_enum() {
    let account_id: AccountId = "acct_C3M2XCLwa3LjkkH4V15muQ".parse().unwrap();
    let user_id: UserId = "user_C3M2XCLwa3LjkkH4V15muQ".parse().unwrap();

    let account_but_maybe_user: UserOrAccount = account_id.into();
    let user_but_maybe_account: UserOrAccount = user_id.into();

    assert_eq!(account_but_maybe_user.as_str(), account_id.as_str());
    assert_eq!(user_but_maybe_account.as_str(), user_id.as_str());

    assert_eq!(account_but_maybe_user, account_id);
    assert_eq!(account_but_maybe_user, "acct_C3M2XCLwa3LjkkH4V15muQ");

    let user_but_maybe_charge: UserOrCharge = user_id.into();

    assert_eq!(user_but_maybe_charge.as_str(), user_but_maybe_account.as_str());

    let bad_enum: Result<UserOrAccount, _> = "ch_C3M2XCLwa3LjkkH4V15muQ".parse();
    assert!(bad_enum.is_err())
}

#[test]
fn test_round_trips() {
    let account_id = AccountId::generate().unwrap();
    let account_id_str = account_id.to_string();
    let account_id2 = AccountId::from_str(&account_id_str).unwrap();
    assert_eq!(account_id, account_id2);

    let user_id = UserId::generate().unwrap();
    let user_id_str = user_id.as_str();
    let user_id2 = UserId::from_str(&user_id_str).unwrap();
    assert_eq!(user_id, user_id2);
}