use u_bbs::secret_keygen;

fn main() {
    let result = secret_keygen();
    println!("{:?}", result.unwrap().to_bytes());
}