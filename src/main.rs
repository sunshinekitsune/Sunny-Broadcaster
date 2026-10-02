mod osc;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let test_packet = osc::encode_chatbox_message("Sunny Broadcaster init", true, false);
    println!("Generated test packet ({} bytes)", test_packet.len());

    Ok(())
}
