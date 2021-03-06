extern mod http;
use http::client::RequestWriter;
use http::method::Get;
use std::str;
use std::rt::io::Reader;
use std::rt::io::net::ip::{SocketAddr, Ipv4Addr};

fn main() {
    let mut request = ~RequestWriter::new(Get, FromStr::from_str("http://localhost/example")
                                                .expect("Uh oh, that's *really* badly broken!"));
    // Temporary measure, as hostname lookup is not yet supported in std::rt::io.
    request.remote_addr = Some(SocketAddr { ip: Ipv4Addr(127, 0, 0, 1), port: 8001 });
    let mut response = match request.read_response() {
        Ok(response) => response,
        Err(_request) => fail!("This example can progress no further with no response :-("),
    };
    println("Yay! Started to get the response.");
    printfln!("Status: %s", response.status.to_str());
    println("Headers: [extension headers only; TODO HeaderCollection.iter()]");
    for (name, value) in response.headers.extensions.iter() {
        printfln!(" - %s: %s", *name, *value);
    }
    print("\n");
    println("First 1024 bytes of response:");
    let mut buf = [0, ..1024];
    match response.read(buf) {
        Some(len) => printfln!("%?", str::from_bytes(buf.slice_to(len))),
        None => println("uh oh, got None :-("),
    }
    // TODO: read it *all*, correctly
}
