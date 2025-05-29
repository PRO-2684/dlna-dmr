use std::{io::Result, sync::{atomic::{AtomicBool, Ordering}, Arc}};
use dlna_dmr::{DMR, DMROptions};

fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    let options = DMROptions::default();
    let running = Arc::new(AtomicBool::new(true));
    let dmr = DMR::new(options, running.clone());

    // Set up Ctrl-C handler before starting the servers
    ctrlc::set_handler(move || {
        running.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");

    // Start the DMR, which will block until stopped
    dmr.start()?;
    Ok(())
}
