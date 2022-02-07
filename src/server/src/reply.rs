pub(super) fn reply_simple(content: &[u8]) -> Vec<u8> {
    let mut buff = Vec::with_capacity(3 + content.len());
    buff.extend_from_slice(b"+");
    buff.extend_from_slice(&content);
    buff.extend_from_slice(b"\r\n");
    buff
}
