extern crate u_bbs;

use curve25519_dalek::scalar::Scalar;
use u_bbs::{groupgen, mac, mac_verify, secret_keygen};
use rand_core::OsRng;
fn random_message_gen() -> Result<Vec<Scalar>, String> {


    let mut test_message = Vec::new();
    let mut i = 0;
    let mut csprng = OsRng;

    while i < 8 {
        test_message.push(Scalar::random(&mut csprng));
        i = i + 1;
    }
    Ok(test_message)
}
#[test]
fn random_mac_verify() {
    let test_group = groupgen(8).unwrap();
    let test_sk = secret_keygen().unwrap();
    let test_message = random_message_gen().unwrap();
    let (test_a, test_e) = mac(test_sk, test_message.clone(), test_group.clone()).unwrap();
    assert_eq!(mac_verify(test_e, test_a, test_sk, test_group.clone(), test_message.clone()).unwrap(), true);
}

#[test]
fn random_mac_fails() {
    let test_group = groupgen(8).unwrap();
    let test_sk = secret_keygen().unwrap();
    let test_message = random_message_gen().unwrap();
    let test_other_message = random_message_gen().unwrap();
    let (test_a, test_e) = mac(test_sk, test_message.clone(), test_group.clone()).unwrap();
    assert_eq!(mac_verify(test_e, test_a, test_sk, test_group.clone(), test_other_message.clone()).unwrap(), false);

}