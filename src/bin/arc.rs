// Topic: Channels
//
// Summary:
// * Create a multi-threaded program to simulate the internal logic of a copy machine.
//   The copy machine has two components:
//   - Control panel
//   - Scanner/Printer
//   The copy machine operation works as such:
//   1. User places paper onto machine
//   2. User presses "Copy" button on a panel
//   3. Copier will begin scanning pages
//   4. As pages get scanned, they will get printer
//
// Requirements:
// * The user must be notified if an error occurs
// * The control interface must display the number of pages scanned & copied
// * The user must be able to stop the copier
// * The copier only has enough memory for 3 pages, so only 3 pages may be
//   in the scanner buffer at one time
// * Use the println! macro to display messages for the simulation
//
// Notes:
// * Ensure the crossbeam-channel crate is added to your Cargo.toml file
// * Use VecDeque from the standard library as a scan data buffer

use crossbeam_channel::{unbounded};
use std::thread;
use std::collections::VecDeque;

/// Messages for the Control (main) thread
#[derive(Debug)]
enum ControlMsg {
    PrinterReady,
    PrinterJobComplete,
    PrinterJobError,

    IncrementCopiedCount,
    IncrementScanCount,

    ScannerJobComplete,
    ScannerJobError,
    ScannerReady,
}

/// Messages for the Printer thread
#[derive(Debug)]
enum PrinterMsg {
    Print(String),
}

fn main() {
    let (printer_recv, printer_send) = bounded(3);
    let (control_recv, control_send) = unbounded();
    let printer_thread = thread::spawn(||move {
        loop {
            loop {
                match printer_recv.recv() {
                    Ok(msg) => match msg {
                        PrinterMsg::Print(data) => {
                            while jobs.len() == 3 {

                            }
                            jobs.push_back(data);
                        }
                        PrinterMsg::Stop => {

                        }
                    }
                }
            }
        }
    });

    printer_thread.join();
}