use server::Server;
use tokio::runtime::Builder;
use tracing_subscriber::fmt::Subscriber;

fn main() {
    Subscriber::builder()
        .with_file(true)
        .with_line_number(true)
        .with_level(true)
        .init();

    Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            let addr = "127.0.0.1:9694".parse().unwrap();
            let server = Server::new(addr);
            server.run().await
        });
}
