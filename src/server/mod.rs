use std::{io::Result, sync::Arc};
use tokio::{
    net::{TcpListener, ToSocketAddrs},
    sync::Mutex,
};
use crate::base_types::auth::SessionId;

pub struct Server {
    listener: Arc<Mutex<TcpListener>>,
    /// User info caching can be done at webservers
    users: Vec<SessionId>,
}

impl Server {
    pub async fn new<A: ToSocketAddrs>(addr: A) -> Result<Self> {
        Ok(Server {
            listener: Arc::new(Mutex::new(TcpListener::bind(addr).await?)),
            users: Vec::new(),
        })
    }

    pub async fn destroy(&mut self) {
        drop(self.listener.lock().await);
        self.users.clear();
    }
}
