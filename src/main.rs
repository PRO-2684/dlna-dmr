use std::{io::Result, sync::{atomic::{AtomicBool, Ordering}, Arc}};
use dlna_dmr::{DMR, DMROptions, SOCKET_READ_TIMEOUT};

fn main() -> Result<()> {
    let options = DMROptions::default();
    let running = Arc::new(AtomicBool::new(true));
    let dmr = DMR::new(options, running.clone());

    // Set up Ctrl-C handler before starting the servers
    ctrlc::set_handler(move || {
        // Exit after 1 second
        std::thread::sleep(std::time::Duration::from_millis(SOCKET_READ_TIMEOUT));
        running.store(false, Ordering::SeqCst);
        std::process::exit(0);
    }).expect("Error setting Ctrl-C handler");

    // Start the DMR, which will block until stopped
    dmr.start()?;
    Ok(())
}
