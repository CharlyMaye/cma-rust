# Guide d'utilisation de Swagger UI avec authentification

## 🚀 Accès à Swagger UI

Ouvrez votre navigateur sur : **http://localhost:8080/swagger-ui/**

## 🔐 Test de l'authentification dans Swagger

### Étape 1 : Se connecter (Login)

1. Dans Swagger UI, trouvez la section **"Authentication"**
2. Cliquez sur **POST /api/auth/login**
3. Cliquez sur **"Try it out"**
4. Entrez les identifiants dans le JSON :
   ```json
   {
     "user": "test",
     "password": "password123"
   }
   ```
5. Cliquez sur **"Execute"**
6. ✅ Vous devriez voir une réponse **200 OK** avec :
   ```json
   {
     "data": {
       "user_id": "test",
       "expires_at": "2025-10-19T..."
     },
     "metadata": {
       "status": "success",
       "message": "Login successful"
     }
   }
   ```
7. 🍪 **Important** : Un cookie `session_id` a été automatiquement défini dans votre navigateur

### Étape 2 : Autoriser Swagger à utiliser le cookie

Dans Swagger UI, vous devriez voir un bouton **"Authorize"** 🔓 en haut à droite.

**Note** : Après le login via Swagger, le cookie est automatiquement stocké dans votre navigateur et sera envoyé avec toutes les requêtes suivantes.

### Étape 3 : Tester les endpoints protégés

Maintenant que vous êtes authentifié, vous pouvez tester les endpoints documents :

#### 📝 GET /api/documents - Récupérer tous les documents
1. Trouvez **GET /api/documents** dans la section "Documents"
2. Cliquez sur **"Try it out"**
3. Cliquez sur **"Execute"**
4. ✅ Vous devriez voir **200 OK** avec la liste des documents

#### 📄 POST /api/documents - Créer un document
1. Trouvez **POST /api/documents**
2. Cliquez sur **"Try it out"**
3. Entrez le JSON :
   ```json
   {
     "doc_id": "DOC-TEST-001",
     "content": "{\"title\": \"Mon premier document via Swagger\"}"
   }
   ```
4. Cliquez sur **"Execute"**
5. ✅ Vous devriez voir **201 Created** avec le document créé

#### 🔍 GET /api/documents/{id} - Récupérer un document
1. Trouvez **GET /api/documents/{id}**
2. Cliquez sur **"Try it out"**
3. Entrez `DOC-TEST-001` dans le champ `id`
4. Cliquez sur **"Execute"**
5. ✅ Vous devriez voir **200 OK** avec le document

#### ✏️ PUT /api/documents/{id} - Mettre à jour un document
1. Trouvez **PUT /api/documents/{id}**
2. Cliquez sur **"Try it out"**
3. Entrez `DOC-TEST-001` dans le champ `id`
4. Entrez le JSON :
   ```json
   {
     "content": "{\"title\": \"Document modifié via Swagger\"}"
   }
   ```
5. Cliquez sur **"Execute"**
6. ✅ Vous devriez voir **200 OK** avec le document mis à jour

#### 🗑️ DELETE /api/documents/{id} - Supprimer un document
1. Trouvez **DELETE /api/documents/{id}**
2. Cliquez sur **"Try it out"**
3. Entrez `DOC-TEST-001` dans le champ `id`
4. Cliquez sur **"Execute"**
5. ✅ Vous devriez voir **204 No Content** (pas de corps de réponse)

### Étape 4 : Vérifier la session

#### 🔍 GET /api/auth/verify - Vérifier la session active
1. Trouvez **GET /api/auth/verify**
2. Cliquez sur **"Try it out"**
3. Cliquez sur **"Execute"**
4. ✅ Vous devriez voir **200 OK** avec :
   ```json
   {
     "data": {
       "user_id": "test",
       "expires_at": "2025-10-19T..."
     },
     "metadata": {
       "status": "success"
     }
   }
   ```

### Étape 5 : Se déconnecter

#### 🚪 POST /api/auth/logout - Déconnexion
1. Trouvez **POST /api/auth/logout**
2. Cliquez sur **"Try it out"**
3. Cliquez sur **"Execute"**
4. ✅ Vous devriez voir **200 OK** avec :
   ```json
   {
     "data": {
       "session_found": true
     },
     "metadata": {
       "status": "success",
       "message": "Logout successful"
     }
   }
   ```
5. 🍪 Le cookie `session_id` a été supprimé

### Étape 6 : Tester sans authentification

Après le logout, essayez de récupérer les documents :

1. **GET /api/documents**
2. Cliquez sur **"Execute"**
3. ❌ Vous devriez voir **401 Unauthorized** avec :
   ```json
   {
     "metadata": {
       "status": "error",
       "message": "Unauthorized - Valid session required"
     }
   }
   ```

## 🎯 Workflow complet

```
1. POST /api/auth/login
   └─> 🍪 Cookie session_id créé

2. GET /api/documents
   └─> ✅ Authentifié via cookie

3. POST /api/documents
   └─> ✅ Créer un document

4. PUT /api/documents/{id}
   └─> ✅ Modifier un document

5. DELETE /api/documents/{id}
   └─> ✅ Supprimer un document

6. POST /api/auth/logout
   └─> 🍪 Cookie supprimé

7. GET /api/documents
   └─> ❌ 401 Unauthorized
```

## 🔧 Débogage

### Le cookie n'est pas envoyé ?

Swagger UI devrait automatiquement envoyer les cookies. Si ça ne fonctionne pas :
1. Vérifiez que vous êtes bien sur `localhost:8080`
2. Ouvrez les DevTools du navigateur (F12)
3. Allez dans l'onglet "Application" > "Cookies"
4. Vérifiez que le cookie `session_id` existe

### Erreur 401 même après login ?

1. Vérifiez que le cookie `session_id` est bien présent dans les DevTools
2. La session expire après 24h
3. Relancez le serveur si nécessaire

## 📚 Structure de réponse standardisée

Toutes les réponses suivent ce format :

### Succès
```json
{
  "data": { ... },
  "metadata": {
    "status": "success",
    "message": "...",  // optionnel
    "count": 10        // optionnel pour les listes
  }
}
```

### Erreur
```json
{
  "metadata": {
    "status": "error",
    "message": "Description de l'erreur"
  }
}
```

## 🎉 Fonctionnalités Swagger

- ✅ Documentation interactive complète
- ✅ Authentification par cookie intégrée
- ✅ Test des endpoints directement depuis le navigateur
- ✅ Schémas de données avec exemples
- ✅ Codes de réponse documentés
- ✅ Support des cookies pour l'authentification

Profitez de votre API ! 🚀
