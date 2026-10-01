mod osc;

use std::{error::Error, net::UdpSocket, thread::sleep, time::Duration};

const OSC_ADDRESS: &str = "127.0.0.1:9000";
const BROADCAST_INTERVAL: Duration = Duration::from_millis(1_500);

fn main() -> Result<(), Box<dyn Error>> {
    // Bind to port 0 to ask for any free outgouing UDP port.
    let socket = UdpSocket::bind("127.0.0.1:0")?;
    println!("Socket bound to local address: {}", socket.local_addr()?);

    socket.connect(OSC_ADDRESS)?;
    println!("Broadcasting messages to VRChat on {}", OSC_ADDRESS);

    let mut ticks: u64 = 0;
    loop {
        ticks += 1;

        let message_text = format!("Ticks: {}\nThis is sent automagically!", ticks);

        // Build a packet and send to VRChat.
        let packet = osc::encode_chatbox_mesage(&message_text, true, false);
        socket.send(&packet)?;

        println!("Broadcasted: {}", message_text);

        // Handle ratelimiting.
        sleep(BROADCAST_INTERVAL);
    }
}
