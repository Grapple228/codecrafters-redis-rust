use std::{collections::HashMap, env, io::{BufWriter, Read, Write}, net::{TcpListener, TcpStream}};

use redis_starter_rust::ThreadPool;
use resp_protocol::SimpleString;

fn handle_client(mut stream: TcpStream){
    // READ REQUEST    
    let mut buffer: String = String::new();
    let request = stream.read_to_string(&mut buffer);

    let response = "PONG";

    let simple: SimpleString = SimpleString::new(response.as_bytes());

    // WRITING RESPONSE
    let mut writer = BufWriter::new(&mut stream);
    if writer.write_all(&simple.bytes()).is_err(){
        println!("Failed to response to stream!");
    }
    if writer.flush().is_err(){
        println!("Failed to flush stream!");
    }
}

fn main() {
    process_agrs();

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                pool.execute(||{
                    println!("accepted new connection");
                    handle_client(_stream);
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn process_agrs() {
    let args: Vec<String> = env::args().collect();
    println!("Processing args:\n{args:#?}");
    
    let mut hashmap: HashMap<String, String> = HashMap::new();

    let mut key: String = String::new();
    let mut is_has_key: bool = false;
    let mut iter = IntoIterator::into_iter(args);
    loop {
        match iter.next() {
            Some(i) => {
                let value = i.as_str();
                match &value[..2] {
                    "--" => {
                        is_has_key = true;
                        key = value.to_string().replace("--", "");
                    }
                    _ => {
                        if is_has_key{
                            hashmap.insert(key.to_string(), value.to_owned());
                            is_has_key = false;
                        }
                    }
                }
            },
            None => break
        }
    }
    
    for (key, value) in hashmap.into_iter() {
        env::set_var(key, value);
    }
}