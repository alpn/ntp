use std::io::{Cursor, Result};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use chrono::{DateTime};

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