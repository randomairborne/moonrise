use http::{Request, Response};
use hyper::{server::conn::Http, service::service_fn, Body};
use std::{convert::Infallible, net::SocketAddr};
use tokio::net::TcpListener;

pub struct Server {
    cmds: crate::router::CommandHierarchy,
};

impl Server {
    pub async fn serve(addr: SocketAddr) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let tcp_listener = TcpListener::bind(addr).await?;
        loop {
            let (tcp_stream, _) = tcp_listener.accept().await?;
            tokio::task::spawn(async move {
                if let Err(http_err) = Http::new()
                    .http1_only(true)
                    .http1_keep_alive(true)
                    .serve_connection(tcp_stream, service_fn(Self::handle_discord_message))
                    .await
                {
                    eprintln!("Error while serving HTTP connection: {}", http_err);
                }
            });
        }
    }

    async fn handle_discord_message(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
        Ok(Response::new(Body::from("Hello World!")))
    }
    pub async fn new(router: crate::router::Router) -> Self {

    }
}
