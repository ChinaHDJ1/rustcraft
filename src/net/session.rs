use std::{io, net::SocketAddr};

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
        let reader = PacketReader::new(&buf, self.compressed);
    }
}

pub struct PacketReader<'a> {
    mark: usize,
    data: &'a [u8],
    packet_len: usize,
    data_len: usize,
    compressed: bool,
}

impl PacketReader<'_> {
    fn new(data: &[u8], compressed: bool) -> PacketReader {
        let mut reader = PacketReader {
            mark: 0,
            data,
            compressed,
            packet_len: 0,
            data_len: 0,
        };

        reader.read_varint();

        reader
    }

    fn read_packet_len() {}

    fn read_varint(&mut self) -> Result<i32, ()> {
        const SEGMENT_BITS: u8 = 0x7F;
        const CONTINUE_BIT: u8 = 0x80;

        let mut value: i32 = 0;
        let mut position = 0;
        loop {
            let byte = self.read_byte()?;
            value |= ((byte & SEGMENT_BITS) as i32) << position;

            if (byte & CONTINUE_BIT) == 0 {
                break;
            }

            position += 7;

            if position >= 32 {
                //panic!("VarInt is too big")
                return Err(());
            };
        }

        Ok(value)
    }

    fn read_byte(&mut self) -> Result<u8, ()> {
        self.mark += 1;
        Ok(self.data[self.mark])
    }

    fn add_mark_offest(size: i32) {}
}
