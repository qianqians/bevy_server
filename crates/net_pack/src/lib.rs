pub struct NetPack {
    buf: Vec<u8>
}

impl NetPack {
    pub fn new() -> NetPack {
        NetPack {
            buf: Vec::new()
        }
    }

    pub fn input(&mut self, data: &mut Vec<u8>) {
        self.buf.append(data)
    }

    pub fn try_get_pack(&mut self) -> Option<Vec<u8>> {
        let len0 = self.buf[0] as usize;
        let len1 = self.buf[1] as usize;
        let len2 = self.buf[2] as usize;
        let len3 = self.buf[3] as usize;
        let new_pack_len: usize = len0 | len1 << 8 | len2 << 16 | len3 << 24;
        if new_pack_len > self.buf.len() {
            None
        }
        else {
            let len = new_pack_len as usize;
            let idx = self.buf.len();
            let mut buf = vec![0u8; len];
            buf.copy_from_slice(&self.buf[..len]);

            let remain = idx - len;
            if remain > 0 {
                let mut tmp = vec![0u8; remain];
                tmp.copy_from_slice(&self.buf[len..idx]);
                self.buf.clear();
                self.buf.copy_from_slice(&tmp[..]);
            }
            else {
                self.buf.clear();
            }

            Some(buf)
        }
    }
}