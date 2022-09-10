#[derive(Copy, Clone, Debug, PartialEq)]
enum PacketType {
    Text = 0,
    Integer = 1,
    Ping = 2,
}

#[derive(Debug, PartialEq)]
struct Packet {
    dest: String,
    sender: String,
    payload: String,
    packet_type: PacketType,
}

impl Packet {
    fn to_bytes(&self) -> Vec<(u8, Vec<u8>)> {
        let packet = vec![
            self.dest.clone(),
            self.sender.clone(),
            self.payload.clone(),
            format!("{:?}", self.packet_type),
        ];

        let to_bytes: Vec<_> = packet
            .iter()
            .map(|x| {
                let to_bytes = x.as_bytes().to_vec();
                let len = to_bytes.len() as u8;
                (len, to_bytes)
            })
            .collect();
        to_bytes
    }
}

fn from_bytes(bytes: Vec<(u8, Vec<u8>)>) -> Packet{
    let mut packets: Vec<String> = vec![];
    for byte in bytes {
        let len = byte.0 as usize;
        let to_bytes = byte.1;
        let packet = String::from_utf8((&to_bytes[0..len]).to_vec()).unwrap();
        packets.push(packet);

    }

        let last_index = packets.last().unwrap();
        let packet_type = match last_index as &str {
            "Text" => PacketType::Text,
           "Integer" => PacketType::Integer,
           _ => PacketType::Ping,
       };
    Packet {
        dest: packets[0].clone(),
        sender: packets[1].clone(),
        payload: packets[2].clone(),
        packet_type: packet_type,
    }

}

fn main() {
    let packet = Packet {
        dest: "xesan".to_string(),
        sender: "jean".to_string(),
        payload: "hello world".to_string(),
        packet_type: PacketType::Text,
    };
    assert_eq!(packet, from_bytes(packet.to_bytes()));

    let packet2 = Packet {
        dest: "".to_string(),
        sender: "".to_string(),
        payload: "".to_string(),
        packet_type: PacketType::Integer,
    };
    assert_eq!(packet2, from_bytes(packet2.to_bytes()));
}

