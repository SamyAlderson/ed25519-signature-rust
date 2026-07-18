/// Module de fonctions utilitaires pour la gestion des clés et des signatures.
///
/// Cette module fournit des fonctions pour la génération, la vérification et la manipulation
/// de clés et de signatures Ed25519.
///
/// # Fonctions
///
/// - `generate_keypair()`: Génère une paire de clés privée et publique Ed25519.
/// - `verify_signature()` : Vérifie la signature d'un message en utilisant une clé publique.
/// - `sign_message()` : Génère une signature d'un message en utilisant une clé privée.
///
/// # Exemples
///
/// ```rust
/// use ed25519_signature::utils;
///
/// let (priv_key, pub_key) = utils::generate_keypair();
/// let signature = utils::sign_message(&priv_key, b"Hello, World!");
/// let is_valid = utils::verify_signature(&pub_key, b"Hello, World!", &signature);
/// assert!(is_valid);
/// ```

pub mod utils {
    use crate::{crypto_macros, CryptoError};
    use serde::{Serialize, Deserialize};

    /// Génère une paire de clés privée et publique Ed25519.
    ///
    /// Cette fonction utilise les algorithmes de génération de clés Ed25519 pour créer une paire de clés.
    ///
    /// # Erreurs
    ///
    /// Cette fonction peut échouer en cas d'erreur de génération de clés.
    ///
    /// # Exemples
    ///
    /// ```rust
    /// use ed25519_signature::utils;
    ///
    /// let (priv_key, pub_key) = utils::generate_keypair();
    /// ```
    pub fn generate_keypair() -> Result<(String, String), CryptoError> {
        crypto_macros::generate_keypair()
    }

    /// Vérifie la signature d'un message en utilisant une clé publique.
    ///
    /// Cette fonction utilise les algorithmes de vérification de signature Ed25519 pour vérifier si la signature d'un message est valide en utilisant une clé publique.
    ///
    /// # Erreurs
    ///
    /// Cette fonction peut échouer en cas d'erreur de vérification de signature.
    ///
    /// # Exemples
    ///
    /// ```rust
    /// use ed25519_signature::utils;
    ///
    /// let (priv_key, pub_key) = utils::generate_keypair();
    /// let signature = utils::sign_message(&priv_key, b"Hello, World!");
    /// let is_valid = utils::verify_signature(&pub_key, b"Hello, World!", &signature);
    /// assert!(is_valid);
    /// ```
    pub fn verify_signature(
        public_key: &str,
        message: &[u8],
        signature: &[u8],
    ) -> Result<bool, CryptoError> {
        crypto_macros::verify_signature(public_key, message, signature)
    }

    /// Génère une signature d'un message en utilisant une clé privée.
    ///
    /// Cette fonction utilise les algorithmes de génération de signature Ed25519 pour créer une signature d'un message en utilisant une clé privée.
    ///
    /// # Erreurs
    ///
    /// Cette fonction peut échouer en cas d'erreur de génération de signature.
    ///
    /// # Exemples
    ///
    /// ```rust
    /// use ed25519_signature::utils;
    ///
    /// let (priv_key, pub_key) = utils::generate_keypair();
    /// let signature = utils::sign_message(&priv_key, b"Hello, World!");
    /// ```
    pub fn sign_message(
        private_key: &str,
        message: &[u8],
    ) -> Result<Vec<u8>, CryptoError> {
        crypto_macros::sign_message(private_key, message)
    }
}