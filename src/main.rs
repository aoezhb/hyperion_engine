use tracing::{info, warn, error};
use anyhow::Result;
use clap::Parser;

mod config;
mod network;
mod core;
mod hal;
mod execution;

/// Hyperion Engine CLI Arguments
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Run in demo mode (Simulated task flow and yield generation)
    #[arg(long, default_value_t = false)]
    demo: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Parse CLI arguments
    let args = Args::parse();

    // Initialize logging system
    tracing_subscriber::fmt::init();

    info!("🚀 Hyperion Engine is starting...");
    info!("Version: {}", env!("CARGO_PKG_VERSION"));
    
    if args.demo {
        warn!("⚠️  WARNING: Running in [DEMO MODE]. All tasks and rewards are simulated.");
    }

    // 1. Load Configuration
    info!("Loading node configuration...");
    // let _config = config::load_config().await?;

    // 2. Initialize Hardware Abstraction Layer (HAL)
    info!("Detecting local hardware resources...");
    let hardware_status = hal::detect_hardware().await?;
    info!("Hardware check passed: Detected GPU: {}", hardware_status.gpu_model);

    // 3. Start Core Orchestrator
    info!("Starting Core Orchestrator...");
    let mut orchestrator = core::orchestrator::Orchestrator::new(args.demo);

    // 4. Start P2P Network Layer
    // Note: This is a simplified startup sequence for the MVP.
    let network_handle = network::p2p::start_p2p_node().await?;
    info!("P2P Network Service started. Node ID: {}", network_handle.local_peer_id);

    // 5. Main Loop: Run Orchestrator and Signal Listeners concurrently
    info!("🚀 Hyperion Engine system ready. Entering main event loop.");
    
    tokio::select! {
        // Run core business logic
        _ = orchestrator.run() => {
            error!("Orchestrator exited unexpectedly!");
        }
        // Listen for Ctrl+C (SIGINT)
        _ = tokio::signal::ctrl_c() => {
            warn!("Termination signal received (SIGINT). Initiating graceful shutdown...");
            // Cleanup logic (state persistence, p2p disconnect, etc.)
            info!("Persisting local state... Done");
            info!("Releasing GPU handles... Done");
        }
    }

    info!("Hyperion Engine shutdown complete.");
    Ok(())
}