// Tests unitaires pour la vérification de la signature numérique

#[cfg(test)]
mod tests {
    use crate::ed25519::{generate_keypair, sign, verify};
    use rand::Rng;
    use serde_json::json;

    #[test]
    fn test_generate_keypair() {
        // Génération d'une paire de clés privée/publique
        let (priv_key, pub_key) = generate_keypair().unwrap();

        // Vérification de la longueur des clés
        assert_eq!(priv_key.len(), 32);
        assert_eq!(pub_key.len(), 32);
    }

    #[test]
    fn test_sign() {
        // Génération d'une paire de clés privée/publique
        let (priv_key, pub_key) = generate_keypair().unwrap();

        // Génération d'un message aléatoire
        let mut rng = rand::thread_rng();
        let message: Vec<u8> = (0..32).map(|_| rng.gen()).collect();

        // Signature du message
        let signature = sign(priv_key, &message).unwrap();

        // Vérification de la longueur de la signature
        assert_eq!(signature.len(), 64);
    }

    #[test]
    fn test_verify() {
        // Génération d'une paire de clés privée/publique
        let (priv_key, pub_key) = generate_keypair().unwrap();

        // Génération d'un message aléatoire
        let mut rng = rand::thread_rng();
        let message: Vec<u8> = (0..32).map(|_| rng.gen()).collect();

        // Signature du message
        let signature = sign(priv_key, &message).unwrap();

        // Vérification de la signature
        assert!(verify(&pub_key, &signature, &message));
    }

    #[test]
    fn test_verify_invalid_signature() {
        // Génération d'une paire de clés privée/publique
        let (priv_key, pub_key) = generate_keypair().unwrap();

        // Génération d'un message aléatoire
        let mut rng = rand::thread_rng();
        let message: Vec<u8> = (0..32).map(|_| rng.gen()).collect();

        // Signature du message
        let signature = sign(priv_key, &message).unwrap();

        // Modification de la signature
        let invalid_signature = signature.clone();
        invalid_signature[0] = 0;

        // Vérification de la signature invalidée
        assert!(!verify(&pub_key, &invalid_signature, &message));
    }
}