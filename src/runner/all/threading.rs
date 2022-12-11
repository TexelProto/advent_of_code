use spmc;
use std::{
    future::Future,
    num::NonZeroUsize,
    ops::Deref,
    ptr::drop_in_place,
    sync::{Arc, RwLock},
    task::Poll,
    thread::{self, JoinHandle},
};

pub trait Job: 'static + Send + Sized {
    type Output: 'static + Sync + Send + Sized;
    fn run(self) -> Self::Output;
}

struct JobQueue(spmc::Receiver<Runner>);

type Runner = Box<dyn 'static + FnOnce() + Send>;

#[derive(Debug)]
pub struct JobHandle<T>(Arc<RwLock<Option<T>>>);

pub struct JobHandleFutureView<T>(Arc<RwLock<Option<T>>>);

impl<T> Future for JobHandle<T> {
    type Output = JobHandleFutureView<T>;
    fn poll(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        if self.0.read().unwrap().is_some() {
            Poll::Ready(JobHandleFutureView(self.0.clone()))
        } else {
            Poll::Pending
        }
    }
}

impl<T> Clone for JobHandle<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> JobHandle<T> {
    fn new() -> Self {
        Self(Arc::new(RwLock::new(None)))
    }
    fn submit(&self, value: T) {
        let mut write = self.0.write().unwrap();
        assert!(write.is_none());
        *write = Some(value);
    }
    fn is_set(&self) -> bool {
        self.0.read().unwrap().is_some()
    }
    fn get<'a>(&'a self) -> impl 'a + Deref<Target = Option<T>> {
        self.0.read().unwrap()
    }
}

pub struct ThreadPool {
    _job_queue: Arc<JobQueue>,
    handles: Vec<JoinHandle<()>>,
    sender: spmc::Sender<Runner>,
}

impl ThreadPool {
    pub fn new() -> Self {
        Self::with_threads(std::thread::available_parallelism().unwrap())
    }
    pub fn with_threads(count: NonZeroUsize) -> Self {
        let (send, rec) = spmc::channel::<Runner>();
        let queue = Arc::new(JobQueue(rec));
        let handles = Vec::from_iter(
            std::iter::repeat_with(|| {
                let queue = queue.clone();
                thread::spawn(move || worker(queue))
            })
            .take(count.get()),
        );

        Self {
            _job_queue: queue,
            handles,
            sender: send,
        }
    }

    pub fn queue<T: Job>(&mut self, job: T) -> Option<JobHandle<T::Output>> {
        let handle = JobHandle::new();
        let runner = runner(job, handle.clone());
        let box_runner = Box::new(runner);
        match self.sender.send(box_runner) {
            Ok(_) => Some(handle),
            Err(_) => None,
        }
    }
}

fn runner<T: Job>(job: T, result: JobHandle<T::Output>) -> impl FnOnce() {
    move || {
        let out = job.run();
        result.submit(out)
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        unsafe {
            // drop the sender, which will tell the receivers no more jobs will come
            drop_in_place(&mut self.sender as *mut _);
        }

        for handle in self.handles.drain(..) {
            let _ignore = handle.join();
        }
    }
}

fn worker(queue: Arc<JobQueue>) {
    loop {
        let runner: Runner = {
            match queue.0.recv() {
                Ok(job) => job,
                Err(_err) => return,
            }
        };
        runner();
    }
}
