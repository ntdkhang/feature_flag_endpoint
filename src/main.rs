use std::net::TcpListener;
use feature_flag_endpoint::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind");

    let port = listener.local_addr().unwrap().port();

    println!("Running server at: http://127.0.0.1:{}", port);

    run(listener)
        .expect("Failed to start server")
        .await
}


