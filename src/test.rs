#![cfg(test)]

use crate::make_auth;
use parity_crypto::publickey::{KeyPair, Message, Secret};
use std::str::FromStr;

#[test]
fn test() {
    let node_kp = KeyPair::from_secret(
        Secret::copy_from_str("b71c71a67e1177ad4e901695e1b4b9ee17ae16c6668d313eac2f96dbcda3f291")
            .unwrap(),
    )
    .unwrap();

    let kp = KeyPair::from_secret(
        Secret::copy_from_str("49a7b37aa6f6645917e7b807e9d1c00d4fa71f18343b0d4122a4d2df64dd6fee")
            .unwrap(),
    )
    .unwrap();

    let ephemeral = KeyPair::from_secret(
        Secret::copy_from_str("869d6ecf5211f1cc60418a13b9d870b22959d0c16f02bec714c960dd2298a32d")
            .unwrap(),
    )
    .unwrap();

    let nonce =
        Message::from_str("7e968bba13b6c50e2c4cd7f241cc0d64d1ac25c7f5952df231ac6a2bda8ee5d6")
            .unwrap();

    dbg!(node_kp.public());
    dbg!(kp.public());
    dbg!(ephemeral.public());
    dbg!(ephemeral.secret());
    dbg!(nonce);

    let auth = make_auth(node_kp.public(), &kp, &ephemeral, nonce).unwrap();
}
