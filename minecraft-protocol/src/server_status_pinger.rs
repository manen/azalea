use crate::{
    connection::Connection,
    packets::{
        handshake::client_intention_packet::ClientIntentionPacket,
        status::serverbound_status_request_packet::ServerboundStatusRequestPacket,
        ConnectionProtocol, PacketTrait,
    },
    resolver, ServerAddress,
};

pub async fn ping_server(address: &ServerAddress) -> Result<(), String> {
    let resolved_address = resolver::resolve_address(&address).await?;

    let mut conn = Connection::new(&resolved_address).await?;

    println!("resolved_address {}", &resolved_address.ip);
    println!("writing intention packet {}", address.host);

    // send the client intention packet and switch to the status state
    conn.send_packet(
        ClientIntentionPacket {
            protocol_version: 757,
            hostname: address.host.clone(),
            port: address.port,
            intention: ConnectionProtocol::Status,
        }
        .get(),
    )
    .await;
    conn.switch_state(ConnectionProtocol::Status);

    // send the empty status request packet
    conn.send_packet(ServerboundStatusRequestPacket {}.get())
        .await;

    conn.read_packet().await.unwrap();

    Ok(())

    // let data = mc_buf::read_varint(conn.stream);
    // println!("data {}", data);

    // // log what the server sends back
    // loop {
    //     if 0 == conn.stream.read_buf(&mut conn.buffer).await.unwrap() {
    //         // The remote closed the connection. For this to be a clean
    //         // shutdown, there should be no data in the read buffer. If
    //         // there is, this means that the peer closed the socket while
    //         // sending a frame.

    //         // log conn.buffer
    //         println!("{:?}", conn.buffer);
    //         if conn.buffer.is_empty() {
    //             println!("buffer is empty ok");
    //             return Ok(());
    //         } else {
    //             return Err("connection reset by peer".into());
    //         }
    //     }
    // }
}
