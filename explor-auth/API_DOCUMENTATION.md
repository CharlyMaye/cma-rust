# API Documents - Documentation

## 🎯 Fonctionnalités

- ✅ API REST complète (CRUD) pour la gestion de documents
- ✅ Documentation OpenAPI/Swagger intégrée
- ✅ Middleware d'authentification par cookie
- ✅ Structure de réponse standardisée (data + metadata)
- ✅ Conformité REST (codes HTTP, headers)

## 🚀 Démarrage

```bash
cargo run
```

Le serveur démarre sur `http://localhost:8080`

## 📖 Documentation Swagger UI

Accédez à la documentation interactive à :
**http://localhost:8080/swagger-ui/**

Swagger UI vous permet de :
- Voir tous les endpoints disponibles
- Tester les requêtes directement depuis le navigateur
- Consulter les schémas de données
- Voir les exemples de requêtes/réponses

## 🔐 Authentification

### Middleware d'authentification

Toutes les routes `/api/documents/*` sont protégées par un middleware d'authentification qui :
- Vérifie la présence d'un cookie `session_id`
- Valide la session dans l'AppState
- Retourne `401 Unauthorized` si la session est invalide

### Se connecter (obtenir un cookie de session)

Utilisez les routes d'authentification (à définir) pour obtenir un cookie de session valide.

## 📋 Endpoints API

### Structure de réponse

#### Succès avec données
```json
{
  "data": { ... },
  "metadata": {
    "status": "success",
    "count": 10  // optionnel pour les listes
  }
}
```

#### Erreur
```json
{
  "metadata": {
    "status": "error",
    "message": "Description de l'erreur"
  }
}
```

### GET /api/documents
Récupère tous les documents

**Headers requis:**
- Cookie: `session_id=<votre_session_id>`

**Réponse 200:**
```json
{
  "data": [
    {
      "id": "507f1f77bcf86cd799439011",
      "doc_id": "DOC-2025-001",
      "content": "{\"title\":\"Mon document\"}"
    }
  ],
  "metadata": {
    "status": "success",
    "count": 1
  }
}
```

### GET /api/documents/{id}
Récupère un document par son doc_id

**Paramètres:**
- `id` (path): Identifiant unique du document (doc_id)

**Réponse 200:**
```json
{
  "data": {
    "id": "507f1f77bcf86cd799439011",
    "doc_id": "DOC-2025-001",
    "content": "{\"title\":\"Mon document\"}"
  },
  "metadata": {
    "status": "success"
  }
}
```

**Réponse 404:**
```json
{
  "metadata": {
    "status": "error",
    "message": "Document not found"
  }
}
```

### POST /api/documents
Crée un nouveau document

**Body:**
```json
{
  "doc_id": "DOC-2025-001",
  "content": "{\"title\":\"Nouveau document\"}"
}
```

**Réponse 201 Created:**
Headers: `Location: /api/documents/DOC-2025-001`
```json
{
  "data": {
    "id": "507f1f77bcf86cd799439011",
    "doc_id": "DOC-2025-001",
    "content": "{\"title\":\"Nouveau document\"}"
  },
  "metadata": {
    "status": "success"
  }
}
```

**Réponse 409 Conflict:**
```json
{
  "metadata": {
    "status": "error",
    "message": "Document with doc_id 'DOC-2025-001' already exists"
  }
}
```

### PUT /api/documents/{id}
Met à jour un document existant

**Paramètres:**
- `id` (path): Identifiant unique du document (doc_id)

**Body:**
```json
{
  "content": "{\"title\":\"Document mis à jour\"}"
}
```

**Réponse 200:**
```json
{
  "data": {
    "id": "507f1f77bcf86cd799439011",
    "doc_id": "DOC-2025-001",
    "content": "{\"title\":\"Document mis à jour\"}"
  },
  "metadata": {
    "status": "success"
  }
}
```

### DELETE /api/documents/{id}
Supprime un document

**Paramètres:**
- `id` (path): Identifiant unique du document (doc_id)

**Réponse 204 No Content:**
Pas de corps de réponse

**Réponse 404:**
```json
{
  "metadata": {
    "status": "error",
    "message": "Document not found"
  }
}
```

## 🔒 Codes d'erreur

| Code | Description |
|------|-------------|
| 200 | OK - Requête réussie |
| 201 | Created - Ressource créée |
| 204 | No Content - Suppression réussie |
| 401 | Unauthorized - Session invalide ou absente |
| 404 | Not Found - Ressource non trouvée |
| 409 | Conflict - Conflit (doc_id existe déjà) |
| 500 | Internal Server Error - Erreur serveur |

## 🧪 Exemples avec curl

### Sans authentification (sera rejeté)
```bash
curl http://localhost:8080/api/documents
# Retourne: 401 Unauthorized
```

### Avec authentification (cookie de session)
```bash
# Récupérer tous les documents
curl -b "session_id=votre_session_id" http://localhost:8080/api/documents

# Créer un document
curl -b "session_id=votre_session_id" \
  -X POST \
  -H "Content-Type: application/json" \
  -d '{"doc_id":"DOC-001","content":"{\"title\":\"Test\"}"}' \
  http://localhost:8080/api/documents

# Mettre à jour un document
curl -b "session_id=votre_session_id" \
  -X PUT \
  -H "Content-Type: application/json" \
  -d '{"content":"{\"title\":\"Mis à jour\"}"}' \
  http://localhost:8080/api/documents/DOC-001

# Supprimer un document
curl -b "session_id=votre_session_id" \
  -X DELETE \
  http://localhost:8080/api/documents/DOC-001
```

## 🏗️ Architecture

```
src/
├── documents/
│   ├── control.rs         # Contrôleurs HTTP (handlers)
│   ├── service.rs         # Logique métier
│   ├── data_provider.rs   # Accès aux données MongoDB
│   ├── model.rs           # DTOs et modèles
│   ├── response.rs        # Structures de réponse API
│   ├── db.rs              # Initialisation collection
│   └── mod.rs             # Configuration des routes
├── middleware/
│   ├── auth.rs            # Middleware d'authentification
│   └── mod.rs
└── main.rs                # Configuration OpenAPI et serveur
```

## 📚 Technologies utilisées

- **actix-web 4.11** - Framework web
- **MongoDB 3.3** - Base de données
- **utoipa 5.2** - Génération OpenAPI
- **utoipa-swagger-ui 8.1** - Interface Swagger UI
- **serde** - Sérialisation JSON

## 🎨 Personnalisation

### Ajouter de nouveaux endpoints

1. Ajoutez la fonction dans `control.rs` avec les annotations `#[utoipa::path(...)]`
2. Ajoutez la route dans `configure_document_routes()` dans `mod.rs`
3. Ajoutez le path dans `#[openapi(paths(...))]` dans `main.rs`

### Modifier le middleware

Éditez `src/middleware/auth.rs` pour personnaliser la logique d'authentification.
