use std::io;
use std::sync::Arc;
use std::{collections::HashMap, net::SocketAddr};
use tokio::sync::Mutex;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    stream,
};

use crate::net::session;

use super::session::Session;

pub struct Listener {
    sessions: Arc<Mutex<HashMap<SocketAddr, Session>>>,
}

impl Listener {
    pub fn new() -> Self {
        Listener {
            sessions: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn bind(&mut self, addr: &str) -> io::Result<()> {
        let listener = tokio::net::TcpListener::bind(addr).await?;
        let sessions = self.sessions.clone();
        let mut buf = [0u8; 1024];

        println!("loop start");
        loop {
            let (stream, addr) = listener.accept().await?;

            let mut session = session::Session::new_session(addr, stream);

            tokio::spawn(async move {
                session.epoll_read_packet().await;
            });
        }

        Ok(())
    }

    pub async fn handle_message() {}
}
