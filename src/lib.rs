use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::Receiver;
use std::thread;
use std::thread::JoinHandle;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

// struct Job;

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {

    pub fn new(size: usize) -> ThreadPool {
        let mut workers = Vec::with_capacity(size);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }



        ThreadPool { workers, sender: Some(sender) }
    }

    pub fn execute<F>(&self, f: F)
    where F: FnOnce() + Send + 'static {
        let job = Box::new(f);
        // self.sender.send(job).unwrap();
        self.sender
            .as_ref()
            .unwrap()
            .send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());
        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }

        }

    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(job) => {
                    job();
                }
                Err(_) => {
                    break
                }
            }
            //
            // while let Ok(job) = receiver
            //     .lock()
            //     .unwrap()
            //     .recv() {
            //     job();
            // }
        });
        Worker { id, thread: Some(thread)}
    }
}