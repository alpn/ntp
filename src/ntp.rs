use std::io::Cursor;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

#[derive(Debug)]
pub(crate) struct NtpMessage {
    li_vn_mode : u8,
    stratum: u8,
    poll: u8,
    precision: u8,
    root_delay: u32,
    root_dispersion: u32,
    ref_id: u32,
    pub ref_timestamp: u64,
    pub orig_timestamp: u64,
    pub recv_timestamp: u64,
    pub trans_timestamp: u64,
}

impl NtpMessage {
    pub fn new() -> Self {
        NtpMessage {
            li_vn_mode: 0x1B,  // LI=0, VN=3, Mode=3
            stratum: 0,
            poll: 0xF,         // max polling interval
            precision: 0xF8,   // max precision
            root_delay: 0,
            root_dispersion: 0,
            ref_id: 0,
            ref_timestamp: 0,
            orig_timestamp: 0,
            recv_timestamp: 0,
            trans_timestamp: 0,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buffer = Vec::with_capacity(48);

        buffer.push(self.li_vn_mode);

        buffer.push(self.stratum);
        buffer.push(self.poll);
        buffer.push(self.precision);

        buffer.write_u32::<BigEndian>(self.root_delay).unwrap();
        buffer.write_u32::<BigEndian>(self.root_dispersion).unwrap();

        buffer.write_u32::<BigEndian>(self.ref_id).unwrap();

        buffer.write_u64::<BigEndian>(self.ref_timestamp).unwrap();

        buffer.write_u64::<BigEndian>(self.orig_timestamp).unwrap();

        buffer.write_u64::<BigEndian>(self.recv_timestamp).unwrap();

        buffer.write_u64::<BigEndian>(self.trans_timestamp).unwrap();

        buffer
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut rdr = Cursor::new(bytes);

        NtpMessage {
            li_vn_mode: rdr.read_u8().unwrap(),
            stratum: rdr.read_u8().unwrap(),
            poll: rdr.read_u8().unwrap(),
            precision: rdr.read_u8().unwrap(),
            root_delay: rdr.read_u32::<BigEndian>().unwrap(),
            root_dispersion: rdr.read_u32::<BigEndian>().unwrap(),
            ref_id: rdr.read_u32::<BigEndian>().unwrap(),
            ref_timestamp: rdr.read_u64::<BigEndian>().unwrap(),
            orig_timestamp: rdr.read_u64::<BigEndian>().unwrap(),
            recv_timestamp: rdr.read_u64::<BigEndian>().unwrap(),
            trans_timestamp: rdr.read_u64::<BigEndian>().unwrap(),
        }
    }
}