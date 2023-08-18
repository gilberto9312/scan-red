use pnet::datalink::{self};

fn main() {
    let interfaces = datalink::interfaces();
    for interface in interfaces {
        println!("name {}", interface.name);
        println!("Índice: {}", interface.index);
        println!("Dirección MAC: {:?}", interface.mac.unwrap());
        println!("Direcciones IP: {:?}", interface.ips);
        println!(); 
    }
}
