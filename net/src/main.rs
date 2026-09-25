use std::{env, io::{BufRead, BufReader, BufWriter, Write}, net::{self, Ipv4Addr, TcpStream}};
fn main() {
    // for now
    let port = 4032;
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 || (args[1] != "--server" && args[1] != "--client") {
        let program_name = &args[0];
        eprintln!("Usage: {program_name} --client|--server");
    }
    else if args[1] == "--server" {
        server(port).unwrap();
    } else if args[1] == "--client" {
        client(port).unwrap();
    }
}

fn server(port: u16) -> std::io::Result<()> {
    let localhost = Ipv4Addr::new(127, 0, 0, 1);
    let sock_addr = (localhost, port);
    let listener = net::TcpListener::bind(sock_addr)?;

    println!("Listening to {sock_addr:?}");
    let (stream, client_addr) = listener.accept()?;
    println!("Client connected. Client address: {client_addr}");

    let mut stream_writer = BufWriter::new(stream.try_clone()?);
    let mut stream_reader = BufReader::new(stream);
    println!("{stream_reader:?}");

    let mut buf = String::new();
    while stream_reader.read_line(&mut buf)? > 0 {
        println!("Client sent: {buf}");
        if buf == "oi\n" {
            stream_writer.write_all(b"oi cliente\n")?;
            stream_writer.flush()?;
        } else if buf == "tchau\n" {
            println!("Client wants to close the connection. Closing.");
            return Ok(());
        } else {
            stream_writer.write_all(&buf.as_bytes())?;
            stream_writer.flush()?;
        }
        buf.clear();
    }

    println!("Client closed the connection.");
    Ok(())
}

fn client(port: u16) -> std::io::Result<()> {
    println!("Trying to connect to server.");
    let stream = TcpStream::connect(format!("127.0.0.1:{port}"))?;
    println!("Connection stablished.");
    let mut stream_writer = BufWriter::new(stream.try_clone()?);
    let mut stream_reader = BufReader::new(stream);
    let mut input = String::new();
    println!("Type in your message: ");
    while std::io::stdin().read_line(&mut input)? > 0 {
        println!("Message: {input}");
        let buf = input.as_bytes();
        stream_writer.write_all(&buf)?;
        stream_writer.flush()?;
        let mut answer = String::new();
        if stream_reader.read_line(&mut answer)? == 0 {
            break;
        };
        println!("Server responded: {answer}");
        input.clear();
        println!("Type in your message: ");
    }
    Ok(())
}