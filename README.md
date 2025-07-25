# Chat SSE Demo

Une application de chat en temps réel utilisant **Server-Sent Events (SSE)** et **HTMX**.

## Technologies utilisées

- **Backend** : Rust avec [Axum](https://github.com/tokio-rs/axum)
- **Frontend** : [HTMX](https://htmx.org/) + [HTMX SSE Extension](https://htmx.org/extensions/sse/)
- **Templates** : [Askama](https://github.com/askama-rs/askama)
- **Temps réel** : [Server-Sent Events (SSE)](https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events)
- **Composants** : [Web Components vanilla](https://developer.mozilla.org/en-US/docs/Web/API/Web_components/Using_custom_elements) pour les messages

## Architecture

### Structure du projet

```
src/
├── main.rs              # Point d'entrée et configuration serveur
├── handlers/            # Gestionnaires de routes HTTP
│   ├── index.rs         # Page d'accueil (login/chat)
│   ├── login.rs         # Authentification utilisateur
│   ├── send.rs          # Envoi de messages
│   └── sse.rs           # Stream Server-Sent Events
├── state.rs             # État partagé de l'application
├── types.rs             # Types et structures de données
└── feeder.rs            # Initialisation des données de test
templates/               # Templates HTML
static/                  # Fichiers statiques css, js, ...
```

### Routes HTTP

| Route | Méthode | Description |
|-------|---------|-------------|
| `/` | GET | Page d'accueil (login si non connecté, chat sinon) |
| `/login` | POST | Authentification et création de session |
| `/send` | POST | Envoi d'un nouveau message |
| `/sse` | GET | Stream Server-Sent Events pour temps réel |

### État de l'application

```rust
pub struct AppState {
    pub db: Db,                              // Base de données en mémoire
    pub sse_sender: broadcast::Sender<SseMessage>, // Canal de diffusion SSE
}
```

## Installation et lancement

### Prérequis

- [Rust](https://www.rust-lang.org/fr/learn/get-started)

### Dépendances principales

Voir `Cargo.toml` pour la liste complète des dépendances.

### Lancement

1. **Cloner le projet**
```bash
git clone https://github.com/ziggornif/chat-sse-demo.git
cd chat-sse-demo
```

2. **Lancer l'application**
```bash
cargo run
```

3. **Ouvrir dans le navigateur**
```
http://127.0.0.1:8080
```

## Utilisation

1. **Première visite** : Entrez votre nom d'utilisateur
2. **Chat** : Tapez vos messages et appuyez sur "Envoyer"
3. **Temps réel** : Les messages des autres utilisateurs apparaissent automatiquement
4. **Multi-navigateurs** : Ouvrez plusieurs navigateurs pour simuler plusieurs utilisateurs

## Authentification

Le système utilise un **cookie** pour la session utilisateur :

- Cookie `username` stocké côté navigateur
- Validation automatique sur chaque requête
- Redirection vers login si non authentifié
- Session persistante (pas de logout automatique)

⚠️ **ATTENTION** : Cette authentification est **uniquement pour la démo** et n'est **pas adaptée à la production**

## Temps réel avec SSE

### Comment ça fonctionne

1. **Connexion SSE** : Chaque client se connecte à `/sse`
2. **Canal de diffusion** : Utilise `tokio::sync::broadcast` pour diffuser les messages
3. **Envoi de message** : 
   - POST `/send` sauvegarde le message
   - Le message HTML est généré avec Askama
   - Diffusion à tous les clients connectés via SSE
4. **Réception** : HTMX écoute les événements et met à jour le DOM

### Fonctionnement technique

- **HTMX** se connecte au stream SSE sur `/sse`
- Les nouveaux messages sont diffusés à tous les clients connectés
- Les **Web Components** gèrent l'affichage des messages avec encapsulation CSS

## Web Components

Le projet utilise des **Web Components vanilla** pour les messages :

```html
<chat-message 
  author="Alice" 
  content="Salut tout le monde !" 
  avatar="https://api.dicebear.com/9.x/shapes/svg?seed=Alice">
</chat-message>
```

Chaque message est rendu avec Shadow DOM pour l'encapsulation des styles.

## Ressources

- [Axum Documentation](https://docs.rs/axum/)
- [HTMX Documentation](https://htmx.org/docs/)
- [Server-Sent Events Spec](https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events)
- [Askama Template Engine](https://askama.readthedocs.io/en/stable/)

## Licence

Ce projet est sous licence MIT. Voir le fichier `LICENSE` pour plus de détails.
