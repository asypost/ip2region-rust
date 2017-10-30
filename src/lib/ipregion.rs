use std::rc::Rc;

pub struct IpRegion {
    city_id: u32,
    region: Rc<String>,
}

impl IpRegion {
    pub fn new(city_id: u32, region: String) -> Self {
        Self {
            city_id,
            region: Rc::new(region),
        }
    }

    pub fn city_id(&self) -> u32 {
        self.city_id
    }

    pub fn region(&self) -> String {
        self.region.as_str().into()
    }
}
