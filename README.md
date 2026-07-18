# Ed25519-Signature-Rust

[![Rust](https://img.shields.io/badge/Langage-Rust-blue.svg)](https://www.rust-lang.org/)
[![MIT License](https://img.shields.io/badge/Licence-MIT-orange.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/ed25519-signature-rust/ed25519-signature-rust/actions/workflows/ci.yml/badge.svg)](https://github.com/ed25519-signature-rust/ed25519-signature-rust/actions/workflows/ci.yml)

## Description détaillée

Cette implémentation d'une signature numérique Ed25519 en Rust fournit une manière sécurisée de signer des messages et de les vérifier. Ed25519 est un algorithme de signature numérique basé sur les courbes elliptiques, conçu pour être rapide et sûr.

## Fonctionnalités

*   Génération de paires de clés privée/publique
*   Signature de messages
*   Vérification de signatures
*   Fonctions utilitaires pour la gestion des clés et des signatures

## Installation

Pour installer ce projet, vous devez avoir Rust installé sur votre système. Vous pouvez le télécharger depuis le site officiel de Rust.

Une fois Rust installé, exécutez la commande suivante pour cloner ce projet :
```bash
git clone https://github.com/ed25519-signature-rust/ed25519-signature-rust.git
```
Puis, accédez au répertoire du projet et exécutez la commande suivante pour installer les dépendances :
```bash
cargo build
```

## Usage avec exemples

Pour utiliser cette implémentation, vous pouvez suivre les exemples fournis dans le répertoire `examples`. Le fichier `keygen.rs` montre comment générer une paire de clés privée/publique, tandis que le fichier `signature.rs` montre comment utiliser la signature numérique pour signer un message.

### Génération de clés

```rust
// src/examples/keygen.rs
use ed25519::KeyPair;

fn main() {
    let key_pair = KeyPair::generate();
    println!("Clé publique : {}", key_pair.public_key());
    println!("Clé privée : {}", key_pair.secret_key());
}
```

### Signature d'un message

```rust
// src/examples/signature.rs
use ed25519::Signature;
use ed25519::KeyPair;

fn main() {
    let key_pair = KeyPair::generate();
    let message = "Bonjour, monde !";
    let signature = key_pair.sign(message.as_bytes());
    println!("Signature : {}", signature);
    let verified = key_pair.verify(message.as_bytes(), &signature);
    println!("Signature vérifiée : {}", verified);
}
```

## Architecture du projet

Ce projet est structuré en trois fichiers principaux :

*   `src/main.rs` : Fichier principal, implémentation de la signature numérique.
*   `src/utils.rs` : Fonctions utilitaires pour la gestion des clés et des signatures.
*   `src/test.rs` : Tests unitaires pour la vérification de la signature numérique.

## Contribuer

Si vous souhaitez contribuer à ce projet, veuillez suivre les étapes suivantes :

1.  Cloner le projet en utilisant la commande `git clone`.
2.  Créer une nouvelle branche pour votre modification en utilisant la commande `git checkout -b`.
3.  Effectuer les modifications nécessaires.
4.  Exécuter la commande `cargo test` pour vérifier que les tests passent.
5.  Soumettre vos modifications en utilisant la commande `git push`.

## Licence

Ce projet est sous licence MIT. Vous pouvez le télécharger et l'utiliser librement, mais vous devez respecter les termes de la licence.