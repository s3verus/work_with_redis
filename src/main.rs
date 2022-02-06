use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:1337").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        work_with_redis::handle_connection(stream);
    }
}
