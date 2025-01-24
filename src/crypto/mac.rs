use curve25519_dalek::scalar::Scalar;
use curve25519_dalek::ristretto::RistrettoPoint;
use rand_core::OsRng;

pub fn mac(sk : Scalar, message : Vec<Scalar>, group_params : Vec<RistrettoPoint>) -> Result<(RistrettoPoint, Scalar), String> {
    let mut csprng = OsRng;
    let e = Scalar::random(&mut csprng);
    
    let scalar_coeff = (sk + e).invert();

    let mut i = 1;
    let mut a : RistrettoPoint = group_params[0];
    let n = group_params.len();

    while i < n {
        a = a + message[i - 1] * group_params[i];
        i = i + 1;
    }

    a = a * scalar_coeff;
    return Ok((a, e))
}