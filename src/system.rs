use std::{future::Future, sync::Arc};
use smol::{lock::{Mutex, MutexGuardArc}, Task};

pub struct System<T> {
    item: Arc<Mutex<T>>,
    task: Option<Task<()>>,
}

pub type Lock<T> = MutexGuardArc<T>;

impl<T> System<T> {
    pub fn new(item: T) -> Self {
        Self {
            item: Arc::new(Mutex::new(item)),
            task: None,
        }
    }

    pub async fn exec<C, F>(&mut self, command: C)
    where 
        C: Fn(Lock<T>) -> F + Send + Sync + 'static,
        F: Future<Output = ()> + Send + Sync + 'static,
    {
        // cancel previous task
        if let Some(task) = self.task.take() {
            task.cancel().await;
        };

        // should never fail, because we just cancelled the task which locked it
        let guard = self.item.try_lock_arc().unwrap();

        // launch new task
        let task = smol::spawn(command(guard));

        // save it so that we can cancel it later
        self.task = Some(task);
    }
}
