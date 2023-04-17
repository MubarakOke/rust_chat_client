use std::net::TcpStream;
#[allow(unused_imports)]
use std::io::{Read, Write, ErrorKind, stdin};
#[allow(unused_imports)]
use std::thread::{self, sleep};
use std::time::Duration;
use std::sync::mpsc::{self, TryRecvError};

const LOCAL: &str= "127.0.0.1:7007";

fn main() {
    println!("welcome");
    println!("connecting to server.........");
    let mut client= TcpStream::connect(LOCAL).expect("unable to connect,\nserver probably not up");
    client.set_nonblocking(true).expect("unable to set non blocking");
    println!("connected");

    let (tx , rx)= mpsc::channel::<String>();

    thread::spawn(move ||{
        loop {
            let mut msg= String::new();
            match client.read_to_string(&mut msg){
                Ok(_)=>{
                    println!("recieved message: {}", msg);
                },
                Err(ref e) if e.kind() == ErrorKind::WouldBlock => {},
                Err(_)=>{
                    println!("connection to server was severed");
                    break;
                }
            }
            match rx.try_recv() {
                Ok(msg)=>{
                    println!("message came");
                    println!("i sent {}", msg);
                    client.write_all(msg.as_bytes()).expect("unable to write message");
                },
                Err(TryRecvError::Empty)=> {},
                Err(TryRecvError::Disconnected) => break,
            }
            sleep_func()
        }
    });

    loop{
        println!("Enter your Message: ");
        let mut msg= String::new();
        stdin().read_line(&mut msg).expect("unable to read input");
        let msg= msg.trim().to_string();
        if msg==":quit"{break}
        match tx.send(msg) {
            Ok(_)=> {println!("message sent")},
            Err(e)=>{
                println!("error sending message");
                println!("{}", e);
                break;
            }
        }
    }

    println!("good bye")
}

fn sleep_func(){
    sleep(Duration::from_millis(100));
}