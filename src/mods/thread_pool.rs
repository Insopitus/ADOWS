use std::{thread, sync::{mpsc, Arc, Mutex}};

type Job = Box<dyn FnOnce() + Send + 'static>;
pub struct ThreadPool {
  workers:Vec<Worker>,
  sender:mpsc::Sender<Job>
}
impl ThreadPool {
  /// create a thread pool
  /// 
  /// # Panics
  /// 
  /// when size == 0
    pub fn new(size: usize)->Self {
      // TODO returns a Result<ThreadPool,ThreadPoolCreationError>
      assert!(size>0);
      let (sender,receiver) = mpsc::channel();
      let receiver = Arc::new(Mutex::new(receiver));
      let mut workers = Vec::with_capacity(size);

      for id in 0..size {
        workers.push(Worker::new(id,Arc::clone(&receiver)));
      };
      ThreadPool{
        workers,
        sender,
      }
    }
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
      let job = Box::new(f);
      self.sender.send(job).unwrap(); // TODO error handling
    }
}

struct Worker{
  id:usize,
  handle:thread::JoinHandle<()>
}
impl Worker {
  fn new(id:usize,receiver:Arc<Mutex<mpsc::Receiver<Job>>>)->Self{
    let handle = thread::spawn(move||{
      loop{
        let job = receiver.lock().unwrap().recv().unwrap(); //TODO error handling
        job();
      }
    });
    Worker { id, handle }
  }
}





