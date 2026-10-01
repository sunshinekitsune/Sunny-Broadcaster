#![allow(dead_code)]
fn push_osc_string(buffer: &mut Vec<u8>, text: &str) {
    let bytes = text.as_bytes();
    buffer.extend_from_slice(bytes);

    let padding = 4 - (bytes.len() % 4);
    buffer.resize(buffer.len() + padding, 0);
}

pub fn encode_chatbox_message(text: &str, direct: bool, play_sound: bool) -> Vec<u8> {
    let mut packet = Vec::with_capacity(64 + text.len());

    push_osc_string(&mut packet, "/chatbox/input");

    let mut tags = String::from(",s");
    tags.push(if direct { 'T' } else { 'F' });
    tags.push(if play_sound { 'T' } else { 'F' });
    push_osc_string(&mut packet, &tags);

    push_osc_string(&mut packet, text);

    packet
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_osc_string_pads_to_four_byte_boundary() {
        let mut buffer = Vec::new();
        push_osc_string(&mut buffer, "hello");

        // "hello" is 5 bytes, the function appends 3 null bytes to reach 8 bytes.
        assert_eq!(buffer.as_slice(), b"hello\0\0\0");
    }

    #[test]
    fn encode_chatbox_message_builds_expected_packet() {
        let packet = encode_chatbox_message("hi", true, false);

        // Address "/chatbox/input" (14 bytes + 2 nulls = 16 bytes)
        // Tags ",sTF" (4 bytes + 4 nulls = 8 bytes)
        // Text "hi" (2 bytes + 2 nulls = 4 bytes)
        let expected = b"/chatbox/input\0\0,sTF\0\0\0\0hi\0\0";

        assert_eq!(packet.as_slice(), expected);
    }
}
