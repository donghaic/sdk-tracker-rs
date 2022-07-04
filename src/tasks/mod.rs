use std::borrow::Borrow;
use std::sync::Arc;
use std::time::Duration;

use chrono::{Local, Timelike};
use job_scheduler::{Job, JobScheduler};
use tokio::runtime;

use crate::models::ad_event::Status;
use crate::store::error::{Result, StorageError};
use crate::store::local_store;

pub fn start(tokio_runtime: tokio::runtime::Runtime, local_store: local_store::LocalStore) {
    tokio_runtime.block_on(async {
        let mut sched = JobScheduler::new();
        let store1 = local_store.clone();
        sched.add(Job::new("1/10 * * * * *".parse().unwrap(), move || {
            let time = Local::now().to_string();
            println!("I get executed every 10 seconds! {}", time);
            tokio::spawn(test(store1.clone()));
        }));

        let store2 = local_store.clone();
        sched.add(Job::new("1/30 * * * * *".parse().unwrap(), move || {
            let time = Local::now().to_string();
            println!("I get executed every 10 seconds! {}", time);
            tokio::spawn(test(store2.clone()));
        }));


        loop {
            sched.tick();
            std::thread::sleep(Duration::from_millis(500));
        }
    });
}

async fn test(local_store: local_store::LocalStore) {
    let now = Local::now();
    println!("hello");
    let x = local_store.set("2_min", &Status {
        sid: now.hour(),
        req: now.minute(),
        bid: now.second(),
        price: 0,
    }).await;
}