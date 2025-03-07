use curve25519_dalek::ristretto::RistrettoPoint;
use rand_core::OsRng;

pub fn groupgen(n : u64) -> Result<Vec<RistrettoPoint>, String> {
    let mut csprng = OsRng;

    let mut i = 0; 
    let mut result: Vec<RistrettoPoint> = Vec::new();

    while i < n + 1 {
        let gi = RistrettoPoint::random(&mut csprng);
        result.push(gi);
        i = i + 1;
    }
    Ok(result)
}