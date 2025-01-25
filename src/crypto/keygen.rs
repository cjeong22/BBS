// This file handles the generation of the secret key and public key
use curve25519_dalek::scalar::Scalar;
use curve25519_dalek::ristretto::RistrettoPoint;
use rand_core::OsRng;

pub fn secret_keygen() -> Result<Scalar, String> { 
    let mut csprng = OsRng;
    let secret_key = Scalar::random(&mut csprng);
    Ok(secret_key)
}

pub fn public_keygen(sk: Scalar) -> Result<RistrettoPoint, String> {
    let public_parameter = sk * RistrettoPoint::default();
    Ok(public_parameter)
}