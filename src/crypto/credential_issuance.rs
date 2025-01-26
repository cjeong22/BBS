use curve25519_dalek::Scalar;
use curve25519_dalek::ristretto::RistrettoPoint;
use okamoto::{prove_linear, verify_linear};

use rand_core::OsRng;


// message = attributes in this case
// In this case, the attribute we are simply going to check that each value is greater than 18.
// This function sends a ZKP from the user to the server
// witness = (m, s)
// FOR NOW: ignoring phi
// WTS : c' - sinv G_0 = sum
pub fn user_to_server_zkp(group_params : Vec<RistrettoPoint>, message: Vec<Scalar>, s : Scalar) -> Result<Vec<Scalar>, String> {
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

    
    // let mut cp = group_params[0];

    // let mut i = 0;

    // while i < message.len() {
    //     cp = cp + message[i] * group_params[i + 1];
    //     i = i + 1;
    // }

    // cp = cp * s.invert();

    // let mut matrix:Vec<RistrettoPoint> = Vec::new();
    // let mut witness:Vec<Scalar> = Vec::new();
    // let mut statement:Vec<RistrettoPoint> = Vec::new();

    // let mut i = 0;

    // while i < group_params.len() - 1 {
    //     matrix.push(group_params[i + 1]);
    //     i = i + 1;
    // }

    // let g0 = group_params[0];

    // // witness.push(s);
    // let mut i = 0;

    // while i < message.len() {
    //     witness.push(-message[i]);
    //     i = i + 1;
    // }

    // statement.push(g0 - s * cp);

    // let proof = prove_linear(&matrix, &witness, &statement).unwrap();

    // Ok(proof)
}

// This function sends a ZKP from the server to the user
pub fn server_to_user_zkp() {
    
}

// This function is used by the server to verify the ZKP sent from the user
pub fn server_zkp_verify(group_params : Vec<RistrettoPoint>, statement:Vec<RistrettoPoint>, proof: Vec<Scalar>) -> bool {
    verify_linear(&group_params, &statement, &proof).unwrap();
    true
}   

// This function is used by the user to verify the ZKP sent from the server
pub fn user_zkp_verify() {

}