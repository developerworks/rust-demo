use pnet::datalink::{self, NetworkInterface};
use pnet::datalink::Channel::Ethernet;


fn main() {
    // 获取网络接口
    let interfaces = datalink::interfaces();
    println!("{:?}", interfaces);

    let filtered_interfaces = interfaces.iter().filter(|&interface| interface.name == "en0").collect::<Vec<&NetworkInterface>>();

    println!("filtered_interfaces {:?}", filtered_interfaces);
    if filtered_interfaces.len() == 1 {
        let en0 = filtered_interfaces.first().unwrap();
        // let ip4 = en0.ips.iter().filter(|ip| ip.is_ipv4()).collect::<Vec<&IpNetwork>>();
        // println!("ip4 {:?}", ip4);
        // let a = ip4.first().unwrap();
        // let ipstring = a.ip();

        let _interface = filtered_interfaces.iter().next().expect("No network interface found");

        // println_interface_info(interface);

        // 打开网络接口
        let channel = datalink::channel(&en0, Default::default()).expect("Failed to open channel");

        // let (_tx, mut rx) = match datalink::channel(&interface, Default::default()) {
        //     Ok(Channel::Ethernet(tx, rx)) => (tx, rx),
        //     _ => panic!("Unknown channel type")
        // };

        // 获取以太网通道
        if let Ethernet(_, mut rx) = channel {
            // 接收数据包
            while let Ok(packet) = rx.next() {
                println!("packet: {:?}", packet);
            }
        } else {
            println!("Not an Ethernet channel");
        }
    }
}

#[allow(unused)]
fn println_interface_info(interface: &NetworkInterface) {
    println!("interface: {:?}", interface);
    println!("interface name: {}", interface.name);
    println!("interface mac: {:?}", interface.mac);
    println!("interface ip: {:?}", interface.ips);
    println!("interface flags: {:?}", interface.flags);
    println!("interface index: {}", interface.index);
    println!("interface description: {}", interface.description);
}