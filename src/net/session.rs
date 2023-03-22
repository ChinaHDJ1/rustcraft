use std::{
    io::{self, Read},
    net::SocketAddr,
};

use tokio::{io::AsyncReadExt, net::TcpStream};

pub struct LogicError;

pub enum State {
    Handshaking,
    Status,
    Login,
    Play,
}

pub struct Session {
    current_state: State,
    addr: SocketAddr,
    stream: TcpStream,
    compressed: bool,
}

impl Session {
    pub fn new_session(addr: SocketAddr, stream: TcpStream) -> Self {
        let session = Self {
            current_state: State::Handshaking,
            compressed: false,
            addr,
            stream,
        };

        session
    }

    pub fn next_state(&self, state: State) -> Result<(), io::Empty> {
        match state {
            State::Handshaking => {
                //非法的参数
                return Err(io::empty());
            }
            State::Status => {}

            State::Login => {}

            State::Play => {}
        }

        Ok(())
    }

    pub async fn epoll_read_packet(&mut self) -> io::Result<()> {
        let mut buf = [0u8; 1024];
        loop {
            let _ = &self.stream.readable().await?;

            let size = self.stream.read(&mut buf).await?;

            self.read_packet(&buf[0..size]).await
        }
    }

    pub async fn read_packet(&self, buf: &[u8]) {
        let data = Vec::from(buf);
    }
}

pub struct PacketReader {
    mark: usize,
    packet_len: usize,
    data_len: usize,
    packet_id: i32,
    data: Vec<u8>,
}

impl PacketReader {
    fn from(data: &[u8], compressed: bool) -> Result<Self, ()> {
        let mut r = PacketReader {
            mark: 0,
            data_len: 0,
            packet_id: 0,
            packet_len: 0,
            data: Vec::from(data),
        };

        //下面的步骤是检验包

        r.packet_len = r.read_varint()? as usize;

        //非压缩的数据包
        if !compressed {
            let packet_id = r.read_varint()?;
        } else {
            let data_len = r.read_varint()? as usize;

            let mut da = &r.data[r.mark..data_len + r.mark];
            let mut decompressed = Vec::new();
            flate2::read::ZlibDecoder::new(da)
                .read_to_end(&mut decompressed)
                .unwrap();
        }

        //r.data = &r.data[r.mark..r.packet_len];
        Err(())
    }

    fn unvisited_data(&self) -> &[u8] {
        &self.data[self.mark..self.packet_len]
    }

    fn read_varint(&mut self) -> Result<i32, ()> {
        let mut int = 0;
        let mut pos = 0;
        let mut current_byte = 0;
        let SEGMENT_BITS = 0x7F;
        let CONTINUE_BIT = 0x80;

        loop {
            current_byte = self.read_byte().unwrap();

            int |= ((current_byte & SEGMENT_BITS) as i32) << pos;

            if ((current_byte as u8) & CONTINUE_BIT) == 0 {
                break;
            }

            pos += 7;
            if pos >= 32 {
                //"VarInt is too big"
                return Err(());
            }
        }

        Ok(int)
    }

    fn read_byte(&mut self) -> Option<i8> {
        let a: Vec<&u8> = self.data.iter().skip(1).collect();

        let mark = self.mark.clone();

        if mark >= self.data.len() {
            return None;
        }

        self.mark += 1;

        Some(self.data[mark] as i8)
    }

    fn read_bytes(&mut self, count: usize) -> Option<&[u8]> {
        let mark = self.mark.clone();

        let mut counter = 0;
        let result: Vec<u8> = self
            .data
            .iter()
            .filter_map(|b| -> Option<u8> {
                counter += 1;

                Some(*b)
            })
            .collect();

        Some(&self.data[mark..(mark + count)])
    }
}
