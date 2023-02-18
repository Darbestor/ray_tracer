use std::{
    sync::{
        atomic::{AtomicBool, AtomicUsize},
        Arc,
    },
    thread::{self},
    time::Duration,
};

//** Simple progress watcher */
pub struct ProgressObserver {
    total: usize,
}

impl ProgressObserver {
    pub fn new(total: usize) -> Self {
        Self { total }
    }

    /// Spawn thread and start looking for progress
    pub fn start(&self) -> ProgressGuard {
        ProgressGuard::new(self.total)
    }
}

/// Observer thread
pub struct ProgressGuard {
    progress: Arc<AtomicUsize>,
    stop_flag: Arc<AtomicBool>,
}

impl ProgressGuard {
    pub fn new(total: usize) -> Self {
        let progress = Arc::new(AtomicUsize::new(0));
        let stop_flag = Arc::new(AtomicBool::new(false));
        rayon::spawn({
            let progress = progress.clone();
            let stop_flag = stop_flag.clone();
            move || {
                let mut current = 0;
                while !stop_flag.load(std::sync::atomic::Ordering::Acquire) {
                    print!("\x1B[2J\x1B[1;1H");
                    println!(
                        "Progress: {:.2}%...{}%",
                        current as f32 / total as f32 * 100.,
                        100
                    );
                    thread::sleep(Duration::from_millis(500));
                    current = progress.load(std::sync::atomic::Ordering::Relaxed);
                }
            }
        });
        Self {
            progress,
            stop_flag,
        }
    }

    /// Increase progress
    pub fn increase(&self, current: usize) {
        self.progress
            .fetch_add(current, std::sync::atomic::Ordering::Relaxed);
    }
}

impl Drop for ProgressGuard {
    fn drop(&mut self) {
        self.stop_flag
            .store(true, std::sync::atomic::Ordering::Release);
    }
}
