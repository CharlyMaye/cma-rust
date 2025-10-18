use base64::{Engine as _, engine::general_purpose};

// Fonction pour encoder le mot de passe en Base64 (comme côté front-end)
pub fn encode_password(password: &str) -> String {
    general_purpose::STANDARD.encode(password.as_bytes())
}

// Fonction pour valider les credentials
pub fn validate_credentials(user: &str, password: &str) -> bool {
    // Validation simple : user = "test", password = hash de "test"
    let expected_password_hash = encode_password("test");
    user == "test" && password == expected_password_hash
}
