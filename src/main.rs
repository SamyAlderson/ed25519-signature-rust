//! Implémentation d'une signature numérique Ed25519 en Rust

use crate::utils::{generate_private_key, generate_public_key};
use crate::error::{Ed25519Error, Result};
use nalgebra::Vector2;
use rand::Rng;
use serde::{Serialize, Deserialize};
use crypto_mac::{Mac, Key};
use crypto_hash::{Digest, Hasher};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

/// Structure représentant une paire de clés Ed25519
#[derive(Serialize, Deserialize, Clone)]
pub struct PublicKey {
    /// Coordonnées x de la clé publique
    pub x: u64,
    /// Coordonnées y de la clé publique
    pub y: u64,
}

/// Structure représentant une paire de clés Ed25519
#[derive(Serialize, Deserialize, Clone)]
pub struct PrivateKey {
    /// Coordonnées x de la clé privée
    pub x: u64,
    /// Coordonnées y de la clé privée
    pub y: u64,
}

/// Implémentation de la signature numérique Ed25519
pub mod ed25519 {
    use super::*;

    /// Génère une paire de clés Ed25519
    pub fn generate_keypair() -> Result<(PrivateKey, PublicKey)> {
        let mut rng = rand::thread_rng();
        let private_key = generate_private_key(&mut rng)?;
        let public_key = generate_public_key(&private_key)?;
        Ok((private_key, public_key))
    }

    /// Signe un message avec une clé privée Ed25519
    pub fn sign(message: &[u8], private_key: &PrivateKey) -> Result<Vec<u8>> {
        let mut hash = crypto_hash::Blake2b::hasher();
        hash.update(message);
        let hash = hash.finalize();
        let signature = edwards_curve_25519::sign(&private_key.x, &private_key.y, &hash)?;
        Ok(signature)
    }

    /// Vérifie une signature numérique Ed25519
    pub fn verify(message: &[u8], signature: &[u8], public_key: &PublicKey) -> Result<bool> {
        let mut hash = crypto_hash::Blake2b::hasher();
        hash.update(message);
        let hash = hash.finalize();
        let result = edwards_curve_25519::verify(&public_key.x, &public_key.y, &hash, signature)?;
        Ok(result)
    }
}

/// Structure représentant un résultat d'erreur
pub enum Ed25519Error {
    /// Erreur lors de la génération de la clé privée
    KeyGenerationError,
    /// Erreur lors de la génération de la clé publique
    PublicKeyGenerationError,
    /// Erreur lors de la signature numérique
    SignatureError,
    /// Erreur lors de la vérification de la signature numérique
    VerificationError,
}

/// Type alias pour les résultats de la fonction
pub type Result<T> = std::result::Result<T, Ed25519Error>;

fn main() {
    let (private_key, public_key) = ed25519::generate_keypair().unwrap();
    let message = b"Hello, World!";
    let signature = ed25519::sign(message, &private_key).unwrap();
    assert!(ed25519::verify(message, &signature, &public_key).unwrap());
    println!("Clé privée : {:?}", private_key);
    println!("Clé publique : {:?}", public_key);
}