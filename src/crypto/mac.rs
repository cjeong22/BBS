use curve25519_dalek::scalar::Scalar;
use curve25519_dalek::ristretto::RistrettoPoint;
use rand_core::OsRng;

pub fn mac(sk : Scalar, message : Vec<Scalar>, group_params : Vec<RistrettoPoint>) -> Result<(RistrettoPoint, Scalar), String> {
    assert!(message.len() == group_params.len() - 1);
    
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
    Ok((a, e))
}

pub fn mac_verify(e : Scalar, a : RistrettoPoint, sk : Scalar, group_params: Vec<RistrettoPoint>, message: Vec<Scalar>) -> Result<bool, String> {
    let lhs = (sk + e) * a;
    let mut i = 1;
    let mut rhs = group_params[0];
    let n = group_params.len();

    while i < n {
        rhs = rhs + message[i - 1] * group_params[i];
        i = i + 1;
    }
    let res = lhs.eq(&rhs);
    println!("x");
    println!("{:?}", lhs);
    println!("y");
    println!("{:?}", rhs);
    Ok(res)
}