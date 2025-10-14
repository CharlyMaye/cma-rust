//! Module de gestion des logs vers fichier avec rotation automatique
//!
//! # Architecture
//!
//! - `handler.rs` : Façade publique (FileTraceHandler)
//! - `writer.rs` : Thread d'écriture asynchrone
//! - `rotation.rs` : Logique de rotation des fichiers
//! - `file_opener.rs` : Ouverture cross-platform (Unix/Windows)
//!
//! # Utilisation
//!
//! ```no_run
//! use loggerd::trace::file::FileTraceHandler;
//! use loggerd::trace::{Trace, TraceLevel};
//!
//! let handler = FileTraceHandler::new("app.log")?.start()?;
//! handler.log(TraceLevel::Info, "Application started");
//! ```

mod file_opener;
mod handler;
mod rotation;
mod writer;

// Ré-exports publics
pub use handler::FileTraceHandler;
pub use rotation::RotationConfig; // API publique pour config custom
