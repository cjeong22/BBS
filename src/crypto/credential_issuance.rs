use curve25519_dalek::Scalar;
use curve25519_dalek::ristretto::RistrettoPoint;
use okamoto::{prove_linear, verify_linear, prove_dleq, verify_dleq};


// message = attributes in this case
// In this case, the attribute we are simply going to check that each value is greater than 18.
// This function sends a ZKP from the user to the server
// witness = (m, s)
// FOR NOW: ignoring phi
// WTS : c' - sinv G_0 = sum
pub fn user_to_server_zkp_iss(group_params : Vec<RistrettoPoint>, message: Vec<Scalar>, s : Scalar) -> Result<Vec<Scalar>, String> {
    assert!(group_params.len() == message.len() + 1);

    let mut cp = group_params[0];
    let mut i = 0;

    while i < message.len() {
        cp = cp + message[i] * group_params[i + 1];
        i = i + 1;
    }

    cp = s.invert() * cp;

    let mut witness:Vec<Scalar> = Vec::new();

    witness.push(s.invert());

    let mut i = 1;

    while i < group_params.len() {
        witness.push(s.invert() * message[i - 1]);
        i = i + 1;
    }

    let proof = prove_linear(&group_params, &witness, &[cp]).unwrap();

    Ok(proof)
}

// This function sends a ZKP from the server to the user
pub fn server_to_user_zkp_iss(cp : RistrettoPoint, sk : Scalar, witness : Scalar) -> Result<Vec<Scalar>, String>{
    let proof = prove_dleq(&[cp], &(sk + witness).invert(), &[(sk + witness).invert()* cp]).unwrap();
    Ok(proof)
}

// This function is used by the server to verify the ZKP sent from the user
pub fn server_zkp_verify_iss(group_params : Vec<RistrettoPoint>, statement:Vec<RistrettoPoint>, proof: Vec<Scalar>) -> bool {
    match verify_linear(&group_params, &statement, &proof) {
        Ok(_) => true,
        Err(_) => false,
    }
}   

// This function is used by the user to verify the ZKP sent from the server
pub fn user_zkp_verify_iss(cp: RistrettoPoint, ap: Vec<RistrettoPoint>, proof: Vec<Scalar>) -> bool {
    match verify_dleq(&[cp], &ap, &proof) {
        Ok(_) => true,
        Err(_) => false,
    }   
}