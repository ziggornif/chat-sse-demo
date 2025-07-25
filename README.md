# Chat SSE Demo 💬

Une application de chat en temps réel utilisant **Server-Sent Events (SSE)** et **HTMX** pour une expérience de messagerie fluide sans JavaScript complexe.

## 🚀 Technologies utilisées

- **Backend** : [Rust](https://www.rust-lang.org/) avec [Axum](https://github.com/tokio-rs/axum)
- **Frontend** : [HTMX](https://htmx.org/) + [HTMX SSE Extension](https://htmx.org/extensions/server-sent-events/)
- **Templates** : [Askama](https://github.com/djc/askama)
- **Temps réel** : Server-Sent Events (SSE)
- **Styling** : CSS pur (pas de framework)
- **Composants** : Web Components vanilla pour les messages

## ✨ Fonctionnalités

- 🔐 **Authentification simple** par nom d'utilisateur avec cookies *(démo uniquement, non sécurisé)*
- 💬 **Chat en temps réel** via Server-Sent Events
- 🎨 **Interface moderne** avec design responsive
- 📱 **Compatible mobile** et desktop
- ⚡ **Pas de JavaScript complexe** (uniquement HTMX + Web Components)
- 🔄 **Auto-scroll** vers les nouveaux messages
- 🗃️ **Base de données en mémoire** pour la démo
- 🧩 **Web Components** pour les messages (encapsulation et réutilisabilité)

## 🏗️ Architecture

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
└── feeder.rs            # Initialisation de données de test
templates/
├── index.html           # Template de la page de chat
├── login.html           # Template de la page de connexion
└── message.html         # Template pour les messages (Web Component)
static/
├── css/
│   ├── chat.css         # Styles spécifiques au chat
│   └── login.css        # Styles spécifiques au login
└── js/
    └── chat-message.js  # Web Component pour les messages
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

## 🔧 Installation et lancement

### Prérequis

- [Rust](https://rustup.rs/) (édition 2021+)
- [Cargo](https://doc.rust-lang.org/cargo/) (inclus avec Rust)

### Dépendances principales

```toml
[dependencies]
axum = "0.8"
axum-extra = { version = "0.9", features = ["cookie"] }
askama = { version = "0.12", features = ["with-axum"] }
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
ulid = "1.0"
tower-http = { version = "0.5", features = ["fs"] }
```

### Lancement

1. **Cloner le projet**
```bash
git clone <votre-repo>
cd chat-sse-demo
```

2. **Lancer l'application**
```bash
cargo run
```

3. **Ouvrir dans le navigateur**
```
http://127.0.0.1:3000
```

## 🏃‍♂️ Utilisation

1. **Première visite** : Entrez votre nom d'utilisateur
2. **Chat** : Tapez vos messages et appuyez sur "Envoyer"
3. **Temps réel** : Les messages des autres utilisateurs apparaissent automatiquement
4. **Multi-onglets** : Ouvrez plusieurs onglets pour simuler plusieurs utilisateurs

## 🔐 Authentification

Le système utilise des **cookies HTTP** pour maintenir les sessions :

- Cookie `username` stocké côté navigateur
- Validation automatique sur chaque requête
- Redirection vers login si non authentifié
- Session persistante (pas de logout automatique)

⚠️ **ATTENTION** : Cette authentification est **uniquement pour la démo** et n'est **pas adaptée à la production**

## ⚡ Temps réel avec SSE

### Comment ça fonctionne

1. **Connexion SSE** : Chaque client se connecte à `/sse`
2. **Canal de diffusion** : Utilise `tokio::sync::broadcast` pour diffuser les messages
3. **Envoi de message** : 
   - POST `/send` sauvegarde le message
   - Le message HTML est généré avec Askama
   - Diffusion à tous les clients connectés via SSE
4. **Réception** : HTMX écoute les événements et met à jour le DOM

### Configuration HTMX + Web Components

```html
<!-- Template principal avec Web Component -->
<div id="messages" 
     hx-ext="sse" 
     sse-connect="/sse" 
     sse-swap="message" 
     hx-swap="beforeend"
     hx-on::after-swap="this.scrollTop = this.scrollHeight">
  
  <!-- Messages existants -->
  {% for message in messages %}
  <chat-message 
    author="{{ message.author }}" 
    content="{{ message.content }}" 
    avatar="{{ message.avatar }}">
  </chat-message>
  {% endfor %}
</div>

<!-- Chargement du Web Component -->
<script src="/static/js/chat-message.js"></script>
```

Le **Web Component** est automatiquement initialisé par le navigateur quand HTMX injecte le nouveau HTML via SSE.

## 🧩 Web Components

Le projet utilise des **Web Components vanilla** pour les messages, offrant une meilleure encapsulation et réutilisabilité.

### Composant `<chat-message>`

```html
<chat-message 
  author="Alice" 
  content="Salut tout le monde !" 
  avatar="https://api.dicebear.com/9.x/shapes/svg?seed=Alice">
</chat-message>
```

### Implémentation :

**Template Askama (`templates/message.html`) :**
```html
<chat-message 
  author="{{ message.author }}" 
  content="{{ message.content }}" 
  avatar="{{ message.avatar }}">
</chat-message>
```

**Web Component (`static/js/chat-message.js`) :**
```javascript
class ChatMessage extends HTMLElement {
  constructor() {
    super();
    this.attachShadow({ mode: 'open' });
  }
  
  static get observedAttributes() {
    return ['author', 'content', 'avatar'];
  }
  
  connectedCallback() {
    this.render();
  }
  
  render() {
    this.shadowRoot.innerHTML = `
      <style>
        /* Styles encapsulés du composant */
      </style>
      <div class="message">
        <!-- Structure du message -->
      </div>
    `;
  }
}

customElements.define('chat-message', ChatMessage);
```

## 🔧 Développement

### Ajouter une nouvelle route

1. Créer le handler dans `src/handlers/`
2. Ajouter la route dans `main.rs`
3. Créer le template si nécessaire

### Modifier l'interface

- **Templates** : dans `templates/` (syntaxe Askama/Jinja2)
- **Styles** : dans `static/css/` (CSS pur, organisé par page)
- **Web Components** : dans `static/js/`

### Debug

Ajoutez `#[axum::debug_handler]` sur vos handlers pour de meilleurs messages d'erreur.

## 📚 Ressources

- [Axum Documentation](https://docs.rs/axum/)
- [HTMX Documentation](https://htmx.org/docs/)
- [Server-Sent Events Spec](https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events)
- [Askama Template Engine](https://askama.readthedocs.io/en/stable/)

## 📝 Licence

Ce projet est sous licence MIT. Voir le fichier `LICENSE` pour plus de détails.
