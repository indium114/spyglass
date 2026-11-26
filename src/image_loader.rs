use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::sync::{Arc, Condvar, Mutex, mpsc};
use std::thread;

use ratatui_image::picker::Picker;
use ratatui_image::protocol::Protocol;
use spyglass::load_image_protocol;

pub struct ImageLoader {
    queue: Arc<(Mutex<Vec<String>>, Condvar)>,
    rx: mpsc::Receiver<(String, Option<Protocol>)>,
    pub cache: HashMap<String, Option<Protocol>>,
    pending: HashSet<String>,
}

impl ImageLoader {
    pub fn new(picker: Picker, workers: usize) -> Self {
        let queue = Arc::new((Mutex::new(Vec::<String>::new()), Condvar::new()));
        let (tx, rx) = mpsc::channel();

        for _ in 0..workers {
            let q = Arc::clone(&queue);
            let tx = tx.clone();
            let picker = picker.clone();

            thread::spawn(move || {
                loop {
                    let path = {
                        let (lock, cvar) = &*q;
                        let mut stack = lock.lock().unwrap();
                        while stack.is_empty() {
                            stack = cvar.wait(stack).unwrap();
                        }
                        stack.pop().unwrap()
                    };

                    let proto = load_image_protocol(&PathBuf::from(&path), &picker);
                    if tx.send((path, proto)).is_err() {
                        // spyglass closed
                        break;
                    }
                }
            });
        }

        Self {
            queue,
            rx,
            cache: HashMap::new(),
            pending: HashSet::new(),
        }
    }

    /// collect finished images
    pub fn poll(&mut self) -> bool {
        let mut updated = false;
        while let Ok((path, proto)) = self.rx.try_recv() {
            self.pending.remove(&path);
            self.cache.insert(path, proto);
            updated = true;
        }
        updated
    }

    /// get image from cache or queue it to be loaded
    pub fn get_or_queue(&mut self, path: &str) -> Option<&Protocol> {
        if let Some(cached) = self.cache.get(path) {
            return cached.as_ref();
        }
        if self.pending.insert(path.to_owned()) {
            let (lock, cvar) = &*self.queue;
            lock.lock().unwrap().push(path.to_owned());
            cvar.notify_one();
        }
        None
    }
}
