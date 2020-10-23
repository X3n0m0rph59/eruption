/*
    This file is part of Eruption.

    Eruption is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    Eruption is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with Eruption.  If not, see <http://www.gnu.org/licenses/>.
*/

use async_trait::async_trait;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::{io::AsyncReadExt, io::AsyncWriteExt, net::TcpStream};

type Result<T> = std::result::Result<T, eyre::Error>;

/// Represents an RGBA color value
#[derive(Debug, Copy, Clone)]
pub struct RGBA {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[derive(Debug, thiserror::Error)]
pub enum TransportError {
    #[error("Not connected: {description}")]
    NotConnectedError { description: String },

    #[error("Command failed: {description}")]
    CommandError { description: String },
}

#[async_trait]
pub trait Transport {
    async fn connect(&mut self, address: &String) -> Result<()>;
    async fn reconnect(&mut self) -> Result<()>;

    fn is_connected(&self) -> bool;

    async fn ping(&mut self) -> Result<(bool, String)>;

    async fn send_led_map(&mut self, values: &[RGBA]) -> Result<()>;
}

pub struct NetworkFXTransport {
    is_connected: bool,
    address: String,
    socket: Option<TcpStream>,
}

impl NetworkFXTransport {
    pub fn new() -> Self {
        Self {
            is_connected: false,
            address: String::new(),
            socket: None,
        }
    }
}

#[async_trait]
impl Transport for NetworkFXTransport {
    async fn connect(&mut self, address: &String) -> Result<()> {
        self.address = address.clone();

        let socket = TcpStream::connect(&address).await?;

        self.socket.replace(socket);
        self.is_connected = true;

        Ok(())
    }

    async fn reconnect(&mut self) -> Result<()> {
        let socket = TcpStream::connect(&self.address).await?;

        self.socket.replace(socket);
        self.is_connected = true;

        Ok(())
    }

    fn is_connected(&self) -> bool {
        self.is_connected
    }

    async fn ping(&mut self) -> Result<(bool, String)> {
        if !self.is_connected {
            Err(TransportError::NotConnectedError {
                description: "Transport is not connected".into(),
            }
            .into())
        } else {
            if let Some(socket) = &mut self.socket {
                socket.write_all(&Vec::from("STATUS\n")).await?;

                // receive and print the response
                let mut buf_reader = BufReader::new(socket);

                let mut buffer = String::new();
                buf_reader.read_line(&mut buffer).await?;

                Ok((true, buffer))
            } else {
                Err(TransportError::NotConnectedError {
                    description: "Lost connection to server".into(),
                }
                .into())
            }
        }
    }

    async fn send_led_map(&mut self, values: &[RGBA]) -> Result<()> {
        if !self.is_connected {
            Err(TransportError::NotConnectedError {
                description: "Transport is not connected".into(),
            }
            .into())
        } else {
            if let Some(socket) = &mut self.socket {
                let mut key_index = 1;
                let mut commands = String::new();

                for v in values {
                    commands += &format!("{}:{}:{}:{}:{}\n", key_index, v.r, v.g, v.b, v.a);
                    key_index += 1;
                }

                socket.write_all(&Vec::from(commands)).await?;

                // receive and print the response
                let mut buffer = Vec::new();
                socket.read(&mut buffer).await?;

                let reply = String::from_utf8_lossy(&buffer).to_string();

                if reply.starts_with("BYE") || reply.starts_with("ERROR:") {
                    Err(TransportError::CommandError {
                        description: reply.into(),
                    }
                    .into())
                } else {
                    Ok(())
                }
            } else {
                Err(TransportError::NotConnectedError {
                    description: "Lost connection to server".into(),
                }
                .into())
            }
        }
    }
}

// pub struct DbusTransport {}

// impl DbusTransport {}

// impl Transport for DbusTransport {}
