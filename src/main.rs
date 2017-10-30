extern crate ip2region;

use std::net::Ipv4Addr;
use std::env;
use ip2region::Ip2Region;


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("usage: ip2region [ip]");
    } else {
        let mut current_path = env::current_dir().unwrap();
        current_path.push("ip2region.db");
        if current_path.exists() && current_path.is_file() {
            if let Ok(mut ip2region) = Ip2Region::new(current_path.to_str().unwrap()) {
                match args[1].parse::<Ipv4Addr>() {
                    Ok(ip) => match ip2region.get_region(&ip) {
                        Ok(result) => println!("{},{}", result.city_id(), result.region()),
                        Err(e) => println!("{}", e),
                    },
                    Err(_) => println!("IP地址输入不正确"),
                }
            } else {
                println!("初始化失败");
            }
        }else{
            println!("数据文件ip2region.db丢失");
        }
    }
}
