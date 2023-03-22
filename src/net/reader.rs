use std::io::Read;

pub struct PackerParser {
    mark: usize,
    pk_id: Option<i32>,
    raw_data: Vec<u8>,
}

impl PackerParser {
    pub fn from(raw_data: Vec<u8>) -> Self {
        //let data = Vec::from(raw_data);
        let parser = PackerParser {
            mark: 0,
            raw_data,
            pk_id: None,
        };
        parser
    }

    fn verify_legality(&mut self, compressed: bool) -> Result<bool, ()> {
        let pk_len = self.read_varint()?;
        let pk_id = self.read_varint()?;
        let mark = self.mark;
        self.raw_data = self.raw_data[mark..(pk_len as usize)].to_vec();
        self.mark = 0;
        Ok(true)
    }

    fn verify_legality_compressed(&mut self) -> Result<bool, ()> {
        //压缩的包
        let pk_len = self.read_varint()?;
        let data_len = self.read_varint()?;
        let mark = self.mark;

        let mut new_raw_data: Vec<u8> = Vec::new();
        let mut zlib = flate2::read::ZlibDecoder::new(&self.raw_data[mark..(pk_len as usize)]);

        match zlib.read_to_end(&mut new_raw_data) {
            Ok(size) => {
                self.raw_data = new_raw_data[..size].to_vec();
                self.mark = 0;
                self.pk_id = Some(self.read_varint()?);
            }
            Err(err) => {}
        }

        Ok(true)
    }

    fn read_byte(&mut self) -> Result<i8, ()> {
        match self.next() {
            Some(b) => Ok(b as i8),
            None => Err(()),
        }
    }

    fn read_bytes(&mut self, size: i32) -> Result<&[u8], ()> {
        let mark = self.mark;
        let next_mark = mark + size as usize;

        if next_mark >= self.raw_data.len() {
            return Err(());
        }

        self.mark = next_mark;

        Ok(&self.raw_data[mark..next_mark])
    }

    fn read_string(&mut self) -> Result<String, ()> {
        //let data = self.read_bytes(size)
        let str_len = self.read_varint()?;

        let str_data = self.read_bytes(str_len)?.to_vec();

        match String::from_utf8(str_data) {
            Ok(string) => Ok(string),
            Err(_) => Err(()),
        }
    }

    fn read_varint(&mut self) -> Result<i32, ()> {
        let mut value: i32 = 0;
        let mut pos = 0;
        let mut b: u8 = 0;

        loop {
            b = self.next().unwrap();

            value |= ((b & 0x7F) as i32) << pos;

            if b & 0x80 == 0 {
                break;
            }

            pos += 7;
        }

        Ok(value)
    }
}

impl Iterator for PackerParser {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let mark = self.mark;

        self.mark += 1;

        self.raw_data.get(mark).copied()
    }
}
