use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};
use std::net::Ipv4Addr;
use super::into_u32::IntoU32;
use super::ipregion::IpRegion;

pub struct Ip2Region {
    file: Box<File>,
    index_start: u32,
    index_end: u32,
}


impl Ip2Region {
    const INDEX_BLOCK_LENGTH: u8 = 12;

    pub fn new(file: &str) -> io::Result<Self> {
        let mut file = File::open(file)?;
        let mut super_block = [0; 8];
        file.read_exact(&mut super_block)?;
        let mut index_start: u32 = 0;
        let mut index_end: u32 = 0;
        (0..4).for_each(|i| {
            let rhs: u32 = ((i) * 8) as u32;
            index_start = index_start | (super_block[i] as u32) << rhs;
        });
        (0..4).for_each(|i| {
            let rhs: u32 = (i * 8) as u32;
            index_end = index_end | (super_block[i + 4] as u32) << rhs;
        });
        Ok(Ip2Region {
            file: Box::new(file),
            index_start,
            index_end,
        })
    }

    pub fn total_blocks(&self) -> u32 {
        return (self.index_end - self.index_start) / (Self::INDEX_BLOCK_LENGTH as u32) + 1;
    }

    pub fn get_region(&mut self, ip: &Ipv4Addr) -> io::Result<IpRegion> {
        let mut start: u32 = 0;
        let mut end = self.total_blocks();
        let ip = ip.into_u32();
        let mut data_ptr = 0;
        let mut buffer = [0; Self::INDEX_BLOCK_LENGTH as usize];
        while start < end {
            let middle: u32 = ((end + start)) >> 1;
            self.file.seek(SeekFrom::Start(
                self.index_start as u64 + middle as u64 * Self::INDEX_BLOCK_LENGTH as u64,
            ))?;
            self.file.read_exact(&mut buffer)?;
            let sip = buffer[..4].into_u32();
            if ip < sip {
                end = middle - 1;
            } else {
                let eip = buffer[4..8].into_u32();
                if ip > eip {
                    start = middle + 1;
                } else {
                    data_ptr = buffer[8..].into_u32();
                    break;
                }
            }
        }
        if data_ptr == 0 {
            return Err(io::Error::from(io::ErrorKind::NotFound));
        }
        let data_len = (data_ptr >> 24) & 0xFF;
        data_ptr = data_ptr & 0x00FFFFFF;
        self.file.seek(SeekFrom::Start(data_ptr as u64))?;
        let mut data_buffer = vec![];
        while (data_buffer.len() as u32) < data_len {
            let len = self.file.read(&mut buffer)?;
            if len == 0 {
                return Err(io::Error::from(io::ErrorKind::UnexpectedEof));
            } else {
                data_buffer.extend_from_slice(&buffer[..len]);
            }
        }
        //读出的数据可能比所需要的数据多
        match String::from_utf8(data_buffer[4..(data_len) as usize].to_vec()) {
            Ok(region) => {
                Ok(IpRegion::new (data_buffer[..4].into_u32(),region))
            }
            Err(_) => Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid UTF8"))
        }
    }
}
