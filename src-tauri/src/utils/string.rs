pub fn from_utf8_or_gbk(v: &[u8]) -> String {
    use encoding::{all::GBK, DecoderTrap, Encoding};
    match String::from_utf8(v.to_vec()) {
        // utf8格式
        Ok(s) => s,
        // 可能GBK
        Err(_) => match GBK.decode(v, DecoderTrap::Strict) {
            Ok(s) => s,
            // 非GBK输出未检验utf8
            Err(_) => unsafe { String::from_utf8_unchecked(v.to_vec()) },
        },
    }
}
