mod crypto;

use crypto::keygen::secret_keygen as secret_keygen;

fn main() {
    let result = secret_keygen();
    println!("The result is: {}", result.unwrap());
}
