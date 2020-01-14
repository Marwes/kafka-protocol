use std::{convert::TryFrom, io, time::Duration};

use tokio::io::{AsyncRead, AsyncWrite};

use crate::{
    client::Client,
    parser::{FetchRequest, FetchResponse},
    Record,
};

pub struct Consumer<I> {
    client: Client<I>,
    fetch_offset: i64,
}

impl Consumer<tokio::net::TcpStream> {
    pub async fn connect(addr: impl tokio::net::ToSocketAddrs) -> io::Result<Self> {
        Ok(Self {
            client: Client::connect(addr).await?,
            fetch_offset: 0,
        })
    }
}

impl<I> Consumer<I>
where
    I: AsyncRead + AsyncWrite + std::marker::Unpin,
{
    pub async fn fetch<'a>(
        &'a mut self,
        topics: impl IntoIterator<Item = &'a str>,
    ) -> io::Result<FetchResponse<'a, Vec<Record<'a>>>> {
        let fetch_offset = self.fetch_offset;
        self.client
            .fetch(FetchRequest {
                replica_id: -1,
                session_epoch: 0,
                forgotten_topics_data: Vec::new(),
                isolation_level: 0,
                session_id: 0,
                min_bytes: 1,
                max_bytes: 1024 * 1024,
                rack_id: "",
                max_wait_time: i32::try_from(Duration::from_millis(10).as_millis()).unwrap(),
                topics: topics
                    .into_iter()
                    .map(|topic| crate::parser::fetch_request::Topics {
                        topic,
                        partitions: vec![crate::parser::fetch_request::Partitions {
                            current_leader_epoch: 0,
                            fetch_offset,
                            log_start_offset: 0,
                            partition: 0,
                            partition_max_bytes: 1024 * 128,
                        }],
                    })
                    .collect(),
            })
            .await
    }

    pub fn commit(&mut self) -> io::Result<()> {
        Ok(())
    }
}
