//! # Waydash - Wayland System Dashboard
//! 
//! A system metrics dashboard built with egui for Wayland environments.
//! This application displays real-time system information and connects to 
//! the loggerd daemon for logging metrics.
//! 
//! ## Features (Planned)
//! - Real-time system metrics (CPU, memory, disk)
//! - Integration with loggerd HTTP API
//! - Wayland native (xdg-shell and layer-shell support)
//! - Customizable dashboard layout
//! 
//! ## Architecture
//! - GUI Framework: egui + eframe
//! - Windowing: winit (Wayland backend)
//! - HTTP Client: reqwest (for loggerd integration)

use eframe::egui;

/// Main entry point for the waydash application
/// 
/// Initializes the egui application with default options and runs the main event loop.
/// This creates a native window using the default Wayland backend.
fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "waydash",
        options,
        Box::new(|_cc| Ok(Box::new(App))), // <- Ok(...)
    )
}

/// Main application state for waydash
/// 
/// Currently a minimal stub implementation. Future versions will include:
/// - HTTP client for loggerd API communication
/// - System metrics collection and display
/// - Dashboard configuration and theming
#[derive(Default)]
struct App;

impl eframe::App for App {
    /// Update the application UI
    /// 
    /// Called every frame to render the user interface. Currently displays
    /// a placeholder UI while the full dashboard functionality is being developed.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("waydash");
            ui.label("Minimal stub — will connect to loggerd later");
        });
    }
}
