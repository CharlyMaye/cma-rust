# Plan d'implémentation I18n

## Phase 1 : Structure de base
- [ ] Créer le dossier `locales/` à la racine du projet
- [ ] Créer les fichiers JSON de base (`en.json`, `fr.json`)
- [ ] Ajouter des traductions de test dans les fichiers JSON
- [ ] Ajouter `serde` et `serde_json` dans `Cargo.toml`

## Phase 2 : Struct I18nState (état interne)
- [ ] Créer la struct `I18nState` avec :
  - `translations: HashMap<String, HashMap<String, String>>`
  - `current_locale: String`
  - `default_locale: String`
- [ ] Implémenter `I18nState::new(default_locale: &str)`

## Phase 3 : Chargement des traductions
- [ ] Implémenter `I18nState::load_locale(&mut self, lang: &str) -> Result<(), Error>`
  - Lire le fichier JSON depuis `locales/{lang}.json`
  - Parser avec `serde_json`
  - Stocker dans `translations`
  - Gérer les erreurs (fichier absent, JSON invalide)

## Phase 4 : Récupération des traductions (sans interpolation)
- [ ] Implémenter `I18nState::get(&self, key: &str) -> String`
  - Récupérer depuis `translations[current_locale][key]`
  - Fallback : retourner la clé si non trouvée
  - Fallback : essayer `default_locale` si clé absente dans `current_locale`

## Phase 5 : Changement de langue
- [ ] Implémenter `I18nState::set_locale(&mut self, lang: &str) -> Result<(), Error>`
  - Vérifier que la langue est chargée
  - Sinon, charger automatiquement
  - Changer `current_locale`

## Phase 6 : Thread-safety avec Arc<RwLock>
- [ ] Créer la struct `I18n` wrapper :
  - `state: Arc<RwLock<I18nState>>`
- [ ] Implémenter `I18n::new(default_locale: &str) -> Self`
- [ ] Implémenter `I18n::clone()` (clone de l'Arc)
- [ ] Wrapper les méthodes principales :
  - `load_locale(&self, lang: &str)`
  - `set_locale(&self, lang: &str)`
  - `t(&self, key: &str) -> String`

## Phase 7 : Interpolation
- [ ] Implémenter `I18nState::interpolate(text: &str, params: &HashMap<&str, &str>) -> String`
  - Remplacer `{key}` par la valeur correspondante
  - Gérer les clés manquantes (garder `{key}` ou remplacer par `""`)
- [ ] Ajouter `I18n::t_with(&self, key: &str, params: HashMap<&str, &str>) -> String`

## Phase 8 : Tests
- [ ] Tests unitaires pour `I18nState::get()`
- [ ] Tests pour le fallback (clé manquante)
- [ ] Tests pour l'interpolation
- [ ] Tests pour le changement de langue
- [ ] Tests pour le thread-safety (spawn plusieurs threads)

## Phase 9 : Améliorations (optionnel)
- [ ] Support des clés imbriquées (`user.profile.name`)
- [ ] Méthode `available_locales() -> Vec<String>` (lister les langues chargées)
- [ ] Lazy loading des locales (charger uniquement à la demande)
- [ ] Cache pour éviter les allocations répétées
- [ ] Logger les clés manquantes pour debug

## Phase 10 : Documentation
- [ ] Documenter les structs et méthodes publiques
- [ ] Créer un exemple d'utilisation dans `examples/i18n_demo.rs`
- [ ] Documenter le format JSON attendu
- [ ] Ajouter un README dans `locales/`

## Notes d'implémentation
- Utiliser `std::fs::read_to_string` pour lire les fichiers
- `serde_json::from_str` pour parser
- Pattern `if let Ok(guard) = self.state.read()` pour accéder au state
- Pattern `if let Ok(mut guard) = self.state.write()` pour modifier
- Considérer `thiserror` pour les erreurs custom

## Ordre de développement conseillé
1. Phase 1 → 2 → 3 (avoir le chargement qui marche)
2. Phase 4 (traductions basiques)
3. Phase 6 (thread-safety avant d'aller plus loin)
4. Phase 5 (changement de langue)
5. Phase 7 (interpolation)
6. Phase 8 (tests)