// Topic: Arc, Mutex, and Threads
//
// Summary:
// * Create a multi-threaded program to simulate a copy machine.
//
//   A partial program has already been written as a guide. Implement
//   the remaining functionality to complete the simulation.
//
// Requirements:
// * The total number of pages must be displayed after each copy
// * The user must be able to cancel a print job
// * The copier has the following limitations:
//   - The size of the print buffer only has enough capacity for 3 pages
//   - The printer takes 500ms to print a page
// * Use the println! macro to display messages for the simulation
//
// Notes:
// * Ensure following crates are added to your Cargo.toml file:
//   - crossbeam-channel
//   - colored
//   - parking_lot

use colored::Colorize;
use crossbeam_channel::{unbounded, Sender};
use parking_lot::Mutex;
use std::collections::VecDeque;
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::time::Duration;

/// Message copier can receive through a channel
enum CopierMsg {
    Shutdown,
}

/// Copy machine
struct Copier {
    buffer: Arc<Mutex<VecDeque<String>>>,
    /// Use .join() on this to wait for the thread to terminate
    thread: JoinHandle<()>,
    /// Use this to send messages to the copier thread through a channel
    tx: Sender<CopierMsg>,
}

impl Copier {
    /// Print out the next page from the buffer
    fn print(buffer: Arc<Mutex<VecDeque<String>>>) -> bool {
        let mut buffer = buffer.lock();
        if let Some(page) = buffer.pop_front() {
            println!("{} {}", "Print:".black().on_yellow(), page);
            true
        } else {
            false
        }
    }

    /// Create a new copier
    fn new() -> Self {
        let (tx, rx) = unbounded();
        let buffer = Arc::new(Mutex::new(VecDeque::new()));

        let buffer_arc = Arc::clone(&buffer);
        let thread = thread::spawn(move || {
            let mut num_copies = 0;
            loop {
                if Self::print(Arc::clone(&buffer_arc)) {
                    num_copies += 1;
                    println!("Pages copied: {}", num_copies);
                }

                thread::sleep(Duration::from_millis(500));

                if let Ok(msg) = rx.try_recv() {
                    match msg {
                        CopierMsg::Shutdown => {
                            println!("{}", "Copier shutting down...".blue());
                            break;
                        }
                    }
                }
            }
        });

        Self { buffer, thread, tx }
    }

    /// Add a new job to the buffer
    fn add_job(&self, page: &str) -> bool {
        let mut buffer = self.buffer.lock();
        if buffer.len() < 3 {
            buffer.push_back(page.to_string());
            true
        } else {
            false
        }
    }

    /// Cancel jobs in the buffer
    fn cancel(&self) {
        let mut buffer = self.buffer.lock();
        buffer.clear();
        println!("{}", "Jobs canceled".black().on_red());
    }

    /// Waits until all pages in the buffer are copied
    fn wait_until_done(&self) {
        loop {
            let buffer = self.buffer.lock();
            if buffer.is_empty() {
                break;
            } else {
                thread::sleep(Duration::from_millis(100));
            }
        }
    }

    /// Turn off copier
    fn shutdown(self) {
        self.tx.send(CopierMsg::Shutdown);
        self.thread.join();
    }
}

/// Possible jobs the copier can handle
enum Job<'a> {
    Cancel,
    Copy(&'a str),
}

/// Helper function to ensure all jobs eventually get sent to the copier
fn send_jobs(copier: &Copier, mut jobs: VecDeque<Job>) {
    loop {
        // Get next job
        if let Some(job) = jobs.pop_front() {
            match job {
                Job::Copy(page) => {
                    // Try to add the copy job
                    if !copier.add_job(page) {
                        println!("{}", "Buffer full".red());
                        // Put the job back into the queue
                        jobs.push_front(Job::Copy(page));
                        thread::sleep(Duration::from_millis(100));
                    } else {
                        println!("{}", "Job added".cyan());
                    }
                }
                Job::Cancel => {
                    copier.cancel();
                    return;
                }
            }
        } else {
            println!("{}", "All jobs added".bright_green());
            break;
        }
    }
}

fn main() {
    let mut jobs = VecDeque::new();
    jobs.push_back(Job::Copy("page A"));
    jobs.push_back(Job::Copy("page B"));
    jobs.push_back(Job::Copy("page C"));
    jobs.push_back(Job::Copy("page D"));
    jobs.push_back(Job::Copy("page E"));
    jobs.push_back(Job::Copy("page F"));
    // jobs.push_back(Job::Cancel); // uncomment this line to test job cancelation

    let copier = Copier::new();

    send_jobs(&copier, jobs);

    copier.wait_until_done();
    copier.shutdown();
}
