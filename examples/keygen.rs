// examples/keygen.rs

// Importations des dépendances
use ed25519_signature_rust::{KeyPair, KeyPairGenerator};
use rand::Rng;
use std::error::Error;

/// Génération d'une paire de clés Ed25519
///
/// Cette fonction utilise le générateur de clés intégré pour générer une paire de clés aléatoires.
///
/// # Résultats
///
/// La fonction retourne une paire de clés Ed25519 avec une clé privée et une clé publique.
fn generate_key_pair() -> Result<KeyPair, Box<dyn Error>> {
    // Création d'un générateur de clés
    let mut generator = KeyPairGenerator::new();
    
    // Génération d'une clé privée aléatoire
    let private_key = generator.generate_private_key()?;
    
    // Génération d'une clé publique à partir de la clé privée
    let public_key = generator.generate_public_key(&private_key)?;
    
    // Création d'une paire de clés
    let key_pair = KeyPair::new(private_key, public_key);
    
    // Retour de la paire de clés
    Ok(key_pair)
}

/// Exemple de génération d'une paire de clés Ed25519
///
/// Cette fonction génère une paire de clés Ed25519 aléatoires et les imprime dans la console.
fn main() -> Result<(), Box<dyn Error>> {
    // Génération d'une paire de clés
    let key_pair = generate_key_pair()?;
    
    // Affichage de la clé publique
    println!("Clé publique : {}", key_pair.public_key());
    
    // Affichage de la clé privée
    println!("Clé privée : {}", key_pair.private_key());
    
    // Retour de la fonction principale
    Ok(())
}

```

```rust
// examples/keygen.rs (suite)

// Importations des dépendances
use ed25519_signature_rust::{KeyPair, KeyPairGenerator};
use rand::Rng;
use std::error::Error;

#[cfg(test)]
mod tests {
    use super::*;
    
    // Tests de la fonction generate_key_pair
    #[test]
    fn test_generate_key_pair() {
        // Génération d'une paire de clés
        let key_pair = generate_key_pair().unwrap();
        
        // Vérification de la présence de clés privée et publique
        assert!(key_pair.private_key().is_some());
        assert!(key_pair.public_key().is_some());
    }
}