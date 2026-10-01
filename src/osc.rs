//! OSC packet encoding for VRChat.

/// Appends an OSC 1.0 string to `buffer`, padded with null bytes to a 4-byte boundary.
fn push_osc_string(buffer: &mut Vec<u8>, text: &str) {
    let bytes = text.as_bytes();
    buffer.extend_from_slice(bytes);

    // Calculate how many null bytes we must append.
    // OSC needs at least one null terminator byte at the end of every string.
    let padding = 4 - (bytes.len() % 4);
    buffer.resize(buffer.len() + padding, 0);
}

/// Encodes an OSC chatbox message packet.
///
/// VRChat limits `text` to 144 bytes and 9 lines.
/// Set `direct` to `true` to send directly to chat, or `false` to open the in-game UI.
pub fn encode_chatbox_mesage(text: &str, direct: bool, play_sound: bool) -> Vec<u8> {
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
