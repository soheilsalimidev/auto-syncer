use crate::server::codec::{Codec, Massage};
use anyhow::Result;
use futures::{SinkExt, StreamExt};
use log::{error, info};
use tokio::net::TcpListener;
use tokio_util::codec::Decoder;
mod codec;

async fn handle_connection(stream: tokio::net::TcpStream) -> Result<()> {
    let codec = Codec::new();
    let (mut sink, mut input) = codec.framed(stream).split();

    while let Some(Ok(event)) = input.next().await {
        #[cfg(test)]
        sink.send(event).await?;
    }
    info!("Connection closed");

    Ok(())
}

async fn listen() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("0.0.0.0:9123").await?;

    info!("Listening on {}", listener.local_addr()?);

    loop {
        let (stream, addr) = listener.accept().await?;

        info!("Accepted connection from {}", addr);

        tokio::spawn(async move {
            if let Err(e) = handle_connection(stream).await {
                error!("Error handling connection from {}: {}", addr, e);
            }
        });
    }
}

#[cfg(test)]
mod test {

    use crate::server::codec::Massage;
    use crate::server::listen;
    use bytes::{BufMut, BytesMut};
    use rmp_serde::Serializer;
    use serde::Serialize;

    #[tokio::test]
    async fn run_server() {
        flexi_logger::Logger::try_with_str("info")
            .unwrap()
            .start()
            .unwrap();
        let mut bytes = Vec::new();
        let mut dst = BytesMut::new();
        let item = Massage::Message("asdasd".to_string());
        item.serialize(&mut Serializer::new(&mut bytes)).unwrap();
        dst.put_u32(bytes.len() as u32);
        dst.put(&bytes[..]);

        println!("{:0x?}", &dst[..]);

        listen().await.unwrap();
    }
}
