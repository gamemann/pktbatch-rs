use pnet::packet::PacketData;

pub struct BatchBuffer<'a> {
    pub pkt: PacketData<'a>,

    pub pkt_len: usize,
}

impl<'a> BatchBuffer<'a> {
    pub fn new(pkt: PacketData<'a>) -> Self {
        let pkt_len = pkt.len();
        Self { pkt, pkt_len }
    }
}
