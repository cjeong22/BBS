use curve25519_dalek::scalar::Scalar;
use curve25519_dalek::ristretto::RistrettoPoint;
use rand_core::OsRng;


// message = attributes in this case
// In this case, the attribute we are simply going to check that each value is greater than 18.
// This function sends a ZKP from the user to the server
// witness = (m, s)
// FOR NOW: ignoring phi
pub fn user_to_server_zkp(pp: RistrettoPoint, message: Vec<Scalar>) {
    let mut csprng = OsRng;
    let s = Scalar::random(&mut csprng);
    let mut witness : Vec<Scalar> = Vec::new();

    witness.append(&mut message.clone());
    witness.push(s);
}

// This function sends a ZKP from the server to the user
pub fn server_to_user_zkp() {

}

// This function is used by the server to verify the ZKP sent from the user
pub fn server_zkp_verify() {

}

// This function is used by the user to verify the ZKP sent from the server
pub fn user_zkp_verify() {

}