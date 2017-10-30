use std::net::Ipv4Addr;

pub trait IntoU32 {
    fn into_u32(&self) -> u32;
}

impl IntoU32 for Ipv4Addr {
    fn into_u32(&self) -> u32 {
        let mut result: u32 = 0;
        (0..self.octets().len()).for_each(|i| {
            result = result | (self.octets()[i] as u32) << (((3-i) * 8) as u32);
        });
        return result;
    }
}

impl IntoU32 for [u8] {
    fn into_u32(&self) -> u32 {
        let mut result: u32 = 0;
        (0..self.len()).for_each(|i| {
            result = result | (self[i] as u32) << ((i * 8) as u32);
        });
        return result;
    }
}
