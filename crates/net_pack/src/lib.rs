pub struct NetPack {
    buf: Vec<u8>,
    idx: u32
}

impl NetPack {
    pub fn new() -> NetPack {
        NetPack {
            buf: Vec::new(),
            idx: 0
        }
    }

    pub fn input(&mut self, data: &mut Vec<u8>) {
        self.buf.append(data)
    }

    pub fn try_get_pack(&mut self) -> Option<Vec<u8>> {
        let len0 = self.buf[0] as u32;
        let len1 = self.buf[1] as u32;
        let len2 = self.buf[2] as u32;
        let len3 = self.buf[3] as u32;
        let new_pack_len = len0 | len1 << 8 | len2 << 16 | len3 << 24;
        if new_pack_len > self.idx {
            None
        }
        else {
            let len = new_pack_len as usize;
            let idx = self.idx as usize;
            let mut buf = vec![0u8; len];
            buf.copy_from_slice(&self.buf[..len]);

            let remain = idx - len;
            if remain > 0 {
                let mut tmp = vec![0u8; remain];
                tmp.copy_from_slice(&self.buf[len..idx]);
                self.buf.clear();
                self.buf.copy_from_slice(&tmp[..]);
                self.idx = remain as u32;
            }
            else {
                self.buf.clear();
                self.idx = 0;
            }

            Some(buf)
        }
    }
}