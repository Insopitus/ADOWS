use std::{thread, sync::{mpsc, Arc, Mutex}};

type Job = Box<dyn FnOnce() + Send + 'static>;
pub struct ThreadPool {
  _workers:Vec<Worker>,
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
        _workers: workers,
        sender,
      }
    }
    pub fn execute<F>(&self, f: F)->Result<(),mpsc::SendError<Box<dyn FnOnce()+Send>>>
    where
        F: FnOnce() + Send + 'static,
    {
      let job = Box::new(f);
      self.sender.send(job)?; // TODO error handling
      Ok(())
    }
}

struct Worker{
  _id:usize,
  _handle:thread::JoinHandle<()>
}
impl Worker {
  fn new(id:usize,receiver:Arc<Mutex<mpsc::Receiver<Job>>>)->Self{
    let handle = thread::spawn(move||{
      loop{
        let res = receiver.lock();
        if let Ok(guard) = res {
          let res = guard.recv();
          if let Ok(job) = res {
            job();
          }
        }
      }
    });
    Worker { _id: id, _handle: handle }
  }
}





