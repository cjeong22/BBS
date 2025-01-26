extern crate u_bbs;

use u_bbs::{groupgen, user_to_server_zkp, server_zkp_verify};
use curve25519_dalek::scalar::Scalar;
use rand_core::OsRng;


const GROUP_SIZE : u64 = 8;

fn random_message_gen() -> Result<Vec<Scalar>, String> {
    let mut test_message = Vec::new();
    let mut i = 0;
    let mut csprng = OsRng;

    while i < GROUP_SIZE {
        test_message.push(Scalar::random(&mut csprng));
        i = i + 1;
    }
    Ok(test_message)
}

#[test]
fn user_to_server_zkp_verifies() {
    let mut csprng = OsRng;

    let test_group = groupgen(GROUP_SIZE).unwrap();
    let test_message = random_message_gen().unwrap();
    let s = Scalar::random(&mut csprng);
    let mut cp = test_group[0];
    let mut i = 0;

    while i < test_message.len() {
        cp = cp + test_message[i] * test_group[i + 1];
        i = i + 1;
    }

    cp = s.invert() * cp;

    let proof = user_to_server_zkp(test_group.clone(), test_message, s).unwrap();
    server_zkp_verify(test_group.clone(),[cp].to_vec(), proof);
}