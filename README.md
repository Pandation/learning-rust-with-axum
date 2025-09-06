# 🦀 Rust Core Learning With Axum

Un projet d'apprentissage pour maîtriser Rust, Axum et les concepts avancés du développement backend moderne (architecture DDD et tout type de tests).  (EN COURS :) )

## 🎯 Objectifs

Ce projet vise à explorer et comprendre :

- **Rust** : Syntaxe, ownership, lifetimes, et écosystème
- **Axum** : Framework web moderne et performant
- **Domain-Driven Design (DDD)** : Architecture propre et maintenable
- **Testing** : Tests unitaires, d'intégration et mocks
- **Crates essentielles** : Serde, Tokio, SQLx, et autres outils du backend Rust

## 🏗️ Architecture

Le projet suit les principes du **Domain-Driven Design** :

```
src/
├── domain/           # Couche Domaine (logique métier)
│   ├── task/         # Sous-domaine Task
│   └── user/         # Sous-domaine User
├── interfaces/        # Couche Interface (API/Controllers)
├── infrastructure/    # Couche Infrastructure (DB, services externes)
└── application/       # Couche Application (orchestration)
```

## 🚀 Technologies

- **Rust** : Langage système moderne et sûr
- **Axum** : Framework web asynchrone
- **Tokio** : Runtime asynchrone
- **Serde** : Sérialisation/désérialisation
- **SQLx** : ORM asynchrone pour PostgreSQL
- **PostgreSQL** : Base de données relationnelle
- **Testcontainers** : Tests d'intégration avec containers

## 📚 Concepts Appris

### Rust
- Ownership et borrowing
- Pattern matching
- Traits et génériques
- Gestion d'erreurs avec `Result<T, E>`
- Async/await et futures

### Architecture
- Séparation des couches (Domain, Application, Infrastructure)
- Repository pattern
- Dependency injection
- Value objects et entities
- Domain services vs Application services

### Testing
- Tests unitaires avec mocks
- Tests d'intégration avec testcontainers
- Organisation des tests par couche
- Stratégies de test DDD

## 🧪 Tests

Le projet inclut une stratégie de test complète :

- **Tests unitaires** : Logique métier pure
- **Tests d'application** : Services avec mocks
- **Tests d'infrastructure** : Repositories avec vraie DB
- **Tests d'intégration** : API complète

## 🔧 Développement

```bash
# Installation des dépendances
cargo build

# Exécution des tests
cargo test

# Lancement du serveur
cargo run
```

## 📖 Ressources

- [The Rust Book](https://doc.rust-lang.org/book/)
- [Axum Documentation](https://docs.rs/axum/)
- [Domain-Driven Design](https://martinfowler.com/bliki/DomainDrivenDesign.html)
- [Rust Testing Guide](https://doc.rust-lang.org/book/ch11-00-testing.html)

---

*Projet d'apprentissage personnel - Exploration des technologies backend modernes avec Rust* 🚀
