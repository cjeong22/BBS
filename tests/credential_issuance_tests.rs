extern crate u_bbs;

use u_bbs::{groupgen, secret_keygen, server_to_user_zkp_iss, server_zkp_verify_iss, user_to_server_zkp_iss, user_zkp_verify_iss};
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
fn zkp_verifies() {
    let mut csprng = OsRng;

    let test_group = groupgen(GROUP_SIZE).unwrap();
    let test_message = random_message_gen().unwrap();
    let test_sk = secret_keygen().unwrap();
    

    let s = Scalar::random(&mut csprng);
    let e = Scalar::random(&mut csprng);
    let other_e = secret_keygen().unwrap();

    let mut cp = test_group[0];
    let mut i = 0;

    while i < test_message.len() {
        cp = cp + test_message[i] * test_group[i + 1];
        i = i + 1;
    }

    cp = s.invert() * cp;

    let ap = (test_sk + e).invert() * cp;

    let proof = user_to_server_zkp_iss(test_group.clone(), test_message, s).unwrap();
    assert!(server_zkp_verify_iss(test_group.clone(),[cp].to_vec(), proof) == true);
    let invalid_proof = user_to_server_zkp_iss(test_group.clone(), random_message_gen().unwrap(), s).unwrap();
    assert!(server_zkp_verify_iss(test_group.clone(), [cp].to_vec(), invalid_proof) == false);

    let proof = server_to_user_zkp_iss(cp, test_sk, e).unwrap();
    assert!(user_zkp_verify_iss(cp, [ap].to_vec(), proof) == true);
    let invalid_proof = server_to_user_zkp_iss(cp, test_sk,other_e).unwrap();
    assert!(user_zkp_verify_iss(cp, [ap].to_vec(), invalid_proof) == false);
}