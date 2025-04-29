use std::net::TcpListener;
use tokio;

use criterion::{Criterion, black_box, criterion_group, criterion_main};

use feature_flag_endpoint::run;
use reqwest::Client;

pub fn check_flag_benchmark(c: &mut Criterion) {
    let address = spawn_app();
    let client = Client::new();


    // c.bench_function("check_flag benchmark", move |b| {
    //     b.to_async(FuturesExecutor);
    // });
}

async fn check_flag(address: &str, client: &Client) {
    client
        .get(format!("{}/check_flag", address))
        .body("name=Khang")
        .send();
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind");
    let port = listener.local_addr().unwrap().port();
    let server = run(listener).expect("Failed to run server");
    tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}

criterion_group!(benches, check_flag_benchmark);
criterion_main!(benches);
