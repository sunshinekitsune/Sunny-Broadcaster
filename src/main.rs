use std::{error::Error, net::UdpSocket, thread::sleep, time::Duration};

const OSC_ADDRESS: &str = "127.0.0.1:9000";
const BROADCAST_INTERVAL: Duration = Duration::from_millis(1_500);

// Appends a string to a byte vector.
// This function follows the OSC 1.0 string spec.
fn push_osc_string(buffer: &mut Vec<u8>, text: &str) {
    let bytes = text.as_bytes();
    buffer.extend_from_slice(bytes);

    // Calculate how many null bytes we must append.
    // OSC needs at least one null terminator byte at the end of every string.
    let padding = 4 - (bytes.len() % 4);
    buffer.resize(buffer.len() + padding, 0);
}

// Encode a OSC chat input message.
// text: The message to display. Max length: 144. Max Lines: 9.
// direct: True sends directly as a chat message, false opens the ingame chat UI.
// play_sound: True plays the notification sound, false plays no sound.
fn encode_chatbox_mesage(text: &str, direct: bool, play_sound: bool) -> Vec<u8> {
    // Create a vector with pre allocated memory to avoid reallocations.
    let mut packet = Vec::with_capacity(64 + text.len());

    // OSC address pattern for chat messages.
    push_osc_string(&mut packet, "/chatbox/input");

    // OSC type tag string.
    // Always starts with ',' and 's' tells the receiver the first argument is a string.
    let mut tags = String::from(",s");

    // Add bool parameters inside the type tag string.
    tags.push(if direct { 'T' } else { 'F' });
    tags.push(if play_sound { 'T' } else { 'F' });
    push_osc_string(&mut packet, &tags);

    // OSC arguments.
    // Bools dont have any data in OSC so only the string goes here.
    push_osc_string(&mut packet, text);

    packet
}

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
        let packet = encode_chatbox_mesage(&message_text, true, false);
        socket.send(&packet)?;

        println!("Broadcasted: {}", message_text);

        // Handle ratelimiting.
        sleep(BROADCAST_INTERVAL);
    }
}
