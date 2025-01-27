use curve25519_dalek::Scalar;
use curve25519_dalek::ristretto::RistrettoPoint;
use okamoto::{prove_linear, verify_linear};

pub fn user_to_server_zkp_pres(group_params: Vec<RistrettoPoint>, message: Vec<Scalar>, a : RistrettoPoint, e : Scalar, r : Scalar, rp : Scalar) -> Result<Vec<Scalar>, String> {
    assert!(group_params.len() == message.len() + 1);

    let ap = r * rp * a;
    let mut dp = group_params[0];

    let mut i = 1;
    while i < group_params.len() {
        dp = dp + message[i - 1] * group_params[i];
        i = i + 1;
    }
    dp = r * dp;

    let bp = rp * dp - e * ap;
    let mut witness: Vec<Scalar> = Vec::new();

    witness.push(r * rp);
    let mut i = 0;
    while i < message.len() {
        witness.push(rp * r * message[i]);
        i = i + 1;
    }
    let statement = [bp + e * ap];
    let proof = prove_linear(&group_params, &witness, &statement).unwrap();
    Ok(proof)
}

pub fn server_zkp_verify_pres(group_params : Vec<RistrettoPoint>, ap : RistrettoPoint, sk:Scalar, e:Scalar, proof : Vec<Scalar>) -> bool {
    let bp = sk * ap;
    match verify_linear(&group_params, &[bp + e * ap], &proof) {
        Ok(_) => true,
        Err(_) => false,
    }
}