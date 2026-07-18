/// Exemple d'utilisation de la signature numérique Ed25519 pour signer un message.
///
/// Ce code démontre comment générer une paire de clés privée et publique, signer un message
/// avec la clé privée et vérifier la signature avec la clé publique.
///
/// # Utilisation
///
/// Pour utiliser ce code, assurez-vous que vous avez créé une paire de clés privée et publique
/// à l'aide de l'exemple `keygen.rs`.
///
/// Puis, vous pouvez utiliser ce code pour signer un message et vérifier la signature.

use ed25519_signature::Ed25519KeyPair;
use rand::Rng;
use serde_json::{json, Value};

fn generate_random_message() -> String {
    let mut rng = rand::thread_rng();
    let mut message = String::new();
    for _ in 0..255 {
        message.push(rng.gen::<u8>() as char);
    }
    message
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Générer une paire de clés privée et publique.
    let key_pair = Ed25519KeyPair::generate()?;

    // Générer un message aléatoire.
    let message = generate_random_message();

    // Signer le message avec la clé privée.
    let signature = key_pair.sign(message.as_bytes())?;

    // Vérifier la signature avec la clé publique.
    let verified = key_pair.verify(message.as_bytes(), &signature)?;

    println!("Signature validée : {}", verified);

    Ok(())
}