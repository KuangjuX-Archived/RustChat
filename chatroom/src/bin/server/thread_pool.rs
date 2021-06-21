use std::thread;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc::Receiver;

use self::Signal::Task;
use self::Signal::Terminate;

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>
}

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Signal {
	Task(Job), 
	Terminate, 
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Signal>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let signal = receiver.lock().unwrap().recv().unwrap();
            
            match signal {
                Task(job) => job(),
                Terminate => break
            };
        });

        Worker {id: id, thread: Some(thread)}
    }
}


use std::sync::mpsc::Sender;

pub struct ThreadPool {
    threads: Vec<Worker>,
    sender: Sender<Signal>
}

const MAX_THREADS:usize = 16;

use std::sync::mpsc;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0 && size < MAX_THREADS);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut threads = Vec::with_capacity(size);

        for id in 0..size {
            let worker = Worker::new(id, Arc::clone(&receiver));

            threads.push(worker);
        }

        ThreadPool { threads, sender }
    }

    pub fn execute<F>(&self, f: F) -> Result<(), String> 
        where F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);
        match self.sender.send(Task(job)) {
            Ok(_) => Ok(()),
            Err(_) => Err(String::from("fail to execute")),
        }
    }
}