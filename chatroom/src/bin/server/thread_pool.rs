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
    pub fn new(id: usize, receiver: Arc<Mutex<Receiver<Signal>>>) -> Worker {
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