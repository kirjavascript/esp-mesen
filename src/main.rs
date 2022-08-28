use serial2::SerialPort;
use std::{thread, time};
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};

// AT+CWMODE?
// AT+CWMODE=1
// AT+CWLAP
// AT+CWJAP="TP-Link_AA10","password"
// AP+PING="kirjava.xyz"

fn handle_client(mut stream: TcpStream) {

    let port = SerialPort::open("/dev/ttyUSB0", 115200).expect("no esp connection");

    let mut data = [0 as u8; 256];
    while match stream.read(&mut data) {
        Ok(size) => {

            thread::sleep(time::Duration::from_millis(5));

            if data[0] == '@' as _ {
                port.write(&data[1..size]).unwrap();
                let read_debug = String::from_utf8(data[1..size].to_vec());
                println!("mesen read size: {} \n{:?}", size, read_debug.unwrap());
            } else { // ~ 126

                println!("mesen poll");

                let mut buffer = [0; 256];

                loop {
                    match port.read(&mut buffer) {
                        Ok(size) => {
                            let response = String::from_utf8(buffer[..size].to_vec());
                            println!("port read size: {} \n{}", size, response.unwrap());
                            stream.write(&buffer[..size]).unwrap();

                        },
                        Err(err) => {
                            stream.write("@END\r\n".as_bytes()).unwrap();
                            break;
                        },
                    }
                }

            }

            true
        },
            Err(_) => {
                println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
                stream.shutdown(Shutdown::Both).unwrap();
                false
            }
    } {}
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    match listener.accept() {
        Ok((stream, _addr)) => {
            println!("Connection established!");
            handle_client(stream);
        },
        Err(e) => println!("couldn't get client: {e:?}"),
    }
}
