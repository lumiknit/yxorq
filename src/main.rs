use std::convert::Infallible;
use std::net::SocketAddr;

use http_body_util::Full;
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use tokio::net::TcpListener;

mod arg;
mod config;

async fn hello(
    _: Request<hyper::body::Incoming>,
) -> Result<Response<Full<Bytes>>, Infallible> {
    Ok(Response::new(Full::new(Bytes::from("Hello, World!"))))
}

fn handle_change(change: config::ConfigChange) {
    println!(
        "Config change detected: {} -> {}",
        change.previous_contents, change.current_contents
    );
    unimplemented!();
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let opts = arg::parse();

    let mut config_set = config::ConfigSet::new(opts.config_directory);

    let addr: SocketAddr =
        opts.bind.parse().expect("Unable to parse bind address");
    
    let listener = TcpListener::bind(addr).await?;

    loop {
        let changes = config_set.scan();

        for change in changes {
            handle_change(change);
        }

        let (stream, _sockaddr) = listener.accept().await?;

        // Spawn a tokio task to serve multiple connections concurrently
        tokio::task::spawn(async move {
            // Finally, we bind the incoming connection to our `hello` service
            if let Err(err) = http1::Builder::new()
                // `service_fn` converts our function in a `Service`
                .serve_connection(stream, service_fn(hello))
                .await
            {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
}
