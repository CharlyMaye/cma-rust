use std::fmt::Display;

// L'énuméré est simple (sans données heap) et stocké dans la stack.
// On utilise Copy au lieu de passer par référence (&TraceLevel) car :
// - Copy d'un enum simple = copie de quelques octets (taille d'un discriminant)
// - Référence = copie d'un pointeur (8 octets sur 64-bit) + indirection mémoire
// - Copy est plus performant : pas de déréférencement, accès direct à la valeur
// - Copy est plus idiomatique en Rust pour les types primitifs/simples
// - Simplifie le code : pas de & partout, pas de gestion de lifetime
#[derive(Debug, Copy, Clone)]
pub enum TraceLevel {
    Verbose,
    Debug,
    Info,
    Warning,
    Error,
    Critical,
    None,
}
impl Display for TraceLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let level_str = match self {
            TraceLevel::Verbose => "VERBOSE",
            TraceLevel::Debug => "DEBUG",
            TraceLevel::Info => "INFO",
            TraceLevel::Warning => "WARNING",
            TraceLevel::Error => "ERROR",
            TraceLevel::Critical => "CRITICAL",
            TraceLevel::None => "NONE",
        };
        write!(f, "[{}]", level_str)
    }
}
