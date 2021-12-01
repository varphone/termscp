//! ## Form
//!
//! auth activity components for file transfer params form

/**
 * MIT License
 *
 * termscp - Copyright (c) 2021 Christian Visintin
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */
use super::{FileTransferProtocol, Msg};

use tui_realm_stdlib::{Input, Radio};
use tuirealm::command::{Cmd, CmdResult, Direction, Position};
use tuirealm::event::{Key, KeyEvent, KeyModifiers};
use tuirealm::props::{Alignment, BorderType, Borders, Color, InputType, Style};
use tuirealm::{Component, Event, MockComponent, NoUserEvent, State, StateValue};

// -- protocol

#[derive(MockComponent)]
pub struct ProtocolRadio {
    component: Radio,
}

impl ProtocolRadio {
    pub fn new(default_protocol: FileTransferProtocol, color: Color) -> Self {
        Self {
            component: Radio::default()
                .borders(
                    Borders::default()
                        .color(color)
                        .modifiers(BorderType::Rounded),
                )
                .choices(&["SFTP", "SCP", "FTP", "FTPS", "AWS S3"])
                .foreground(color)
                .rewind(true)
                .title("Protocol", Alignment::Left)
                .value(Self::protocol_enum_to_opt(default_protocol)),
        }
    }

    /// ### protocol_opt_to_enum
    ///
    /// Convert radio index for protocol into a `FileTransferProtocol`
    fn protocol_opt_to_enum(protocol: usize) -> FileTransferProtocol {
        match protocol {
            1 => FileTransferProtocol::Scp,
            2 => FileTransferProtocol::Ftp(false),
            3 => FileTransferProtocol::Ftp(true),
            4 => FileTransferProtocol::AwsS3,
            _ => FileTransferProtocol::Sftp,
        }
    }

    /// ### protocol_enum_to_opt
    ///
    /// Convert `FileTransferProtocol` enum into radio group index
    fn protocol_enum_to_opt(protocol: FileTransferProtocol) -> usize {
        match protocol {
            FileTransferProtocol::Sftp => 0,
            FileTransferProtocol::Scp => 1,
            FileTransferProtocol::Ftp(false) => 2,
            FileTransferProtocol::Ftp(true) => 3,
            FileTransferProtocol::AwsS3 => 4,
        }
    }
}

impl Component<Msg, NoUserEvent> for ProtocolRadio {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        let result = match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Left, ..
            }) => self.perform(Cmd::Move(Direction::Left)),
            Event::Keyboard(KeyEvent {
                code: Key::Right, ..
            }) => self.perform(Cmd::Move(Direction::Right)),
            Event::Keyboard(KeyEvent {
                code: Key::Enter, ..
            }) => return Some(Msg::Connect),
            Event::Keyboard(KeyEvent {
                code: Key::Down, ..
            }) => return Some(Msg::ProtocolBlurDown),
            Event::Keyboard(KeyEvent { code: Key::Up, .. }) => return Some(Msg::ProtocolBlurUp),
            Event::Keyboard(KeyEvent { code: Key::Tab, .. }) => return Some(Msg::ParamsFormBlur),
            _ => return None,
        };
        match result {
            CmdResult::Changed(State::One(StateValue::Usize(choice))) => {
                Some(Msg::ProtocolChanged(Self::protocol_opt_to_enum(choice)))
            }
            _ => Some(Msg::None),
        }
    }
}

// -- address

#[derive(MockComponent)]
pub struct InputAddress {
    component: Input,
}

impl InputAddress {
    pub fn new(host: &str, color: Color) -> Self {
        Self {
            component: Input::default()
                .borders(
                    Borders::default()
                        .color(color)
                        .modifiers(BorderType::Rounded),
                )
                .foreground(color)
                .placeholder("127.0.0.1", Style::default().fg(Color::Rgb(128, 128, 128)))
                .title("Remote host", Alignment::Left)
                .input_type(InputType::Text)
                .value(host),
        }
    }
}

impl Component<Msg, NoUserEvent> for InputAddress {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Left, ..
            }) => {
                self.perform(Cmd::Move(Direction::Left));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Right, ..
            }) => {
                self.perform(Cmd::Move(Direction::Right));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Home, ..
            }) => {
                self.perform(Cmd::GoTo(Position::Begin));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent { code: Key::End, .. }) => {
                self.perform(Cmd::GoTo(Position::End));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Delete, ..
            }) => {
                self.perform(Cmd::Cancel);
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Backspace,
                ..
            }) => {
                self.perform(Cmd::Delete);
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Char(ch),
                modifiers: KeyModifiers::NONE,
            }) => {
                self.perform(Cmd::Type(ch));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Enter, ..
            }) => Some(Msg::Connect),
            Event::Keyboard(KeyEvent {
                code: Key::Down, ..
            }) => Some(Msg::AddressBlurDown),
            Event::Keyboard(KeyEvent { code: Key::Up, .. }) => Some(Msg::AddressBlurUp),
            Event::Keyboard(KeyEvent { code: Key::Tab, .. }) => Some(Msg::ParamsFormBlur),
            _ => None,
        }
    }
}

// -- port number

#[derive(MockComponent)]
pub struct InputPort {
    component: Input,
}

impl InputPort {
    pub fn new(port: u16, color: Color) -> Self {
        Self {
            component: Input::default()
                .borders(
                    Borders::default()
                        .color(color)
                        .modifiers(BorderType::Rounded),
                )
                .foreground(color)
                .placeholder("22", Style::default().fg(Color::Rgb(128, 128, 128)))
                .input_type(InputType::UnsignedInteger)
                .input_len(5)
                .title("Port number", Alignment::Left)
                .value(port.to_string()),
        }
    }
}

impl Component<Msg, NoUserEvent> for InputPort {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Left, ..
            }) => {
                self.perform(Cmd::Move(Direction::Left));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Right, ..
            }) => {
                self.perform(Cmd::Move(Direction::Right));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Home, ..
            }) => {
                self.perform(Cmd::GoTo(Position::Begin));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent { code: Key::End, .. }) => {
                self.perform(Cmd::GoTo(Position::End));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Delete, ..
            }) => {
                self.perform(Cmd::Cancel);
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Backspace,
                ..
            }) => {
                self.perform(Cmd::Delete);
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Char(ch),
                modifiers: KeyModifiers::NONE,
            }) => {
                self.perform(Cmd::Type(ch));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Enter, ..
            }) => Some(Msg::Connect),
            Event::Keyboard(KeyEvent {
                code: Key::Down, ..
            }) => Some(Msg::PortBlurDown),
            Event::Keyboard(KeyEvent { code: Key::Up, .. }) => Some(Msg::PortBlurUp),
            Event::Keyboard(KeyEvent { code: Key::Tab, .. }) => Some(Msg::ParamsFormBlur),
            _ => None,
        }
    }
}

// -- username

#[derive(MockComponent)]
pub struct InputUsername {
    component: Input,
}

impl InputUsername {
    pub fn new(username: &str, color: Color) -> Self {
        Self {
            component: Input::default()
                .borders(
                    Borders::default()
                        .color(color)
                        .modifiers(BorderType::Rounded),
                )
                .foreground(color)
                .placeholder("root", Style::default().fg(Color::Rgb(128, 128, 128)))
                .title("Username", Alignment::Left)
                .input_type(InputType::Text)
                .value(username),
        }
    }
}

impl Component<Msg, NoUserEvent> for InputUsername {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Left, ..
            }) => {
                self.perform(Cmd::Move(Direction::Left));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Right, ..
            }) => {
                self.perform(Cmd::Move(Direction::Right));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Home, ..
            }) => {
                self.perform(Cmd::GoTo(Position::Begin));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent { code: Key::End, .. }) => {
                self.perform(Cmd::GoTo(Position::End));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Delete, ..
            }) => {
                self.perform(Cmd::Cancel);
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Backspace,
                ..
            }) => {
                self.perform(Cmd::Delete);
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Char(ch),
                modifiers: KeyModifiers::NONE,
            }) => {
                self.perform(Cmd::Type(ch));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Enter, ..
            }) => Some(Msg::Connect),
            Event::Keyboard(KeyEvent {
                code: Key::Down, ..
            }) => Some(Msg::UsernameBlurDown),
            Event::Keyboard(KeyEvent { code: Key::Up, .. }) => Some(Msg::UsernameBlurUp),
            Event::Keyboard(KeyEvent { code: Key::Tab, .. }) => Some(Msg::ParamsFormBlur),
            _ => None,
        }
    }
}

// -- password

#[derive(MockComponent)]
pub struct InputPassword {
    component: Input,
}

impl InputPassword {
    pub fn new(password: &str, color: Color) -> Self {
        Self {
            component: Input::default()
                .borders(
                    Borders::default()
                        .color(color)
                        .modifiers(BorderType::Rounded),
                )
                .foreground(color)
                .title("Password", Alignment::Left)
                .input_type(InputType::Password('*'))
                .value(password),
        }
    }
}

impl Component<Msg, NoUserEvent> for InputPassword {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Left, ..
            }) => {
                self.perform(Cmd::Move(Direction::Left));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Right, ..
            }) => {
                self.perform(Cmd::Move(Direction::Right));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Home, ..
            }) => {
                self.perform(Cmd::GoTo(Position::Begin));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent { code: Key::End, .. }) => {
                self.perform(Cmd::GoTo(Position::End));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Delete, ..
            }) => {
                self.perform(Cmd::Cancel);
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Backspace,
                ..
            }) => {
                self.perform(Cmd::Delete);
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Char(ch),
                modifiers: KeyModifiers::NONE,
            }) => {
                self.perform(Cmd::Type(ch));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Enter, ..
            }) => Some(Msg::Connect),
            Event::Keyboard(KeyEvent {
                code: Key::Down, ..
            }) => Some(Msg::PasswordBlurDown),
            Event::Keyboard(KeyEvent { code: Key::Up, .. }) => Some(Msg::PasswordBlurUp),
            Event::Keyboard(KeyEvent { code: Key::Tab, .. }) => Some(Msg::ParamsFormBlur),
            _ => None,
        }
    }
}

// -- s3 bucket

#[derive(MockComponent)]
pub struct InputS3Bucket {
    component: Input,
}

impl InputS3Bucket {
    pub fn new(bucket: &str, color: Color) -> Self {
        Self {
            component: Input::default()
                .borders(
                    Borders::default()
                        .color(color)
                        .modifiers(BorderType::Rounded),
                )
                .foreground(color)
                .placeholder("my-bucket", Style::default().fg(Color::Rgb(128, 128, 128)))
                .title("Bucket name", Alignment::Left)
                .input_type(InputType::Text)
                .value(bucket),
        }
    }
}

impl Component<Msg, NoUserEvent> for InputS3Bucket {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Left, ..
            }) => {
                self.perform(Cmd::Move(Direction::Left));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Right, ..
            }) => {
                self.perform(Cmd::Move(Direction::Right));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Home, ..
            }) => {
                self.perform(Cmd::GoTo(Position::Begin));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent { code: Key::End, .. }) => {
                self.perform(Cmd::GoTo(Position::End));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Delete, ..
            }) => {
                self.perform(Cmd::Cancel);
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Backspace,
                ..
            }) => {
                self.perform(Cmd::Delete);
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Char(ch),
                modifiers: KeyModifiers::NONE,
            }) => {
                self.perform(Cmd::Type(ch));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Enter, ..
            }) => Some(Msg::Connect),
            Event::Keyboard(KeyEvent {
                code: Key::Down, ..
            }) => Some(Msg::S3BucketBlurDown),
            Event::Keyboard(KeyEvent { code: Key::Up, .. }) => Some(Msg::S3BucketBlurUp),
            Event::Keyboard(KeyEvent { code: Key::Tab, .. }) => Some(Msg::ParamsFormBlur),
            _ => None,
        }
    }
}

// -- s3 bucket

#[derive(MockComponent)]
pub struct InputS3Region {
    component: Input,
}

impl InputS3Region {
    pub fn new(region: &str, color: Color) -> Self {
        Self {
            component: Input::default()
                .borders(
                    Borders::default()
                        .color(color)
                        .modifiers(BorderType::Rounded),
                )
                .foreground(color)
                .placeholder("eu-west-1", Style::default().fg(Color::Rgb(128, 128, 128)))
                .title("Region", Alignment::Left)
                .input_type(InputType::Text)
                .value(region),
        }
    }
}

impl Component<Msg, NoUserEvent> for InputS3Region {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Left, ..
            }) => {
                self.perform(Cmd::Move(Direction::Left));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Right, ..
            }) => {
                self.perform(Cmd::Move(Direction::Right));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Home, ..
            }) => {
                self.perform(Cmd::GoTo(Position::Begin));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent { code: Key::End, .. }) => {
                self.perform(Cmd::GoTo(Position::End));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Delete, ..
            }) => {
                self.perform(Cmd::Cancel);
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Backspace,
                ..
            }) => {
                self.perform(Cmd::Delete);
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Char(ch),
                modifiers: KeyModifiers::NONE,
            }) => {
                self.perform(Cmd::Type(ch));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Enter, ..
            }) => Some(Msg::Connect),
            Event::Keyboard(KeyEvent {
                code: Key::Down, ..
            }) => Some(Msg::S3RegionBlurDown),
            Event::Keyboard(KeyEvent { code: Key::Up, .. }) => Some(Msg::S3RegionBlurUp),
            Event::Keyboard(KeyEvent { code: Key::Tab, .. }) => Some(Msg::ParamsFormBlur),
            _ => None,
        }
    }
}

// -- s3 bucket

#[derive(MockComponent)]
pub struct InputS3Profile {
    component: Input,
}

impl InputS3Profile {
    pub fn new(profile: &str, color: Color) -> Self {
        Self {
            component: Input::default()
                .borders(
                    Borders::default()
                        .color(color)
                        .modifiers(BorderType::Rounded),
                )
                .foreground(color)
                .placeholder("default", Style::default().fg(Color::Rgb(128, 128, 128)))
                .title("Profile", Alignment::Left)
                .input_type(InputType::Text)
                .value(profile),
        }
    }
}

impl Component<Msg, NoUserEvent> for InputS3Profile {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Left, ..
            }) => {
                self.perform(Cmd::Move(Direction::Left));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Right, ..
            }) => {
                self.perform(Cmd::Move(Direction::Right));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Home, ..
            }) => {
                self.perform(Cmd::GoTo(Position::Begin));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent { code: Key::End, .. }) => {
                self.perform(Cmd::GoTo(Position::End));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Delete, ..
            }) => {
                self.perform(Cmd::Cancel);
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Backspace,
                ..
            }) => {
                self.perform(Cmd::Delete);
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Char(ch),
                modifiers: KeyModifiers::NONE,
            }) => {
                self.perform(Cmd::Type(ch));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Enter, ..
            }) => Some(Msg::Connect),
            Event::Keyboard(KeyEvent {
                code: Key::Down, ..
            }) => Some(Msg::S3ProfileBlurDown),
            Event::Keyboard(KeyEvent { code: Key::Up, .. }) => Some(Msg::S3ProfileBlurUp),
            Event::Keyboard(KeyEvent { code: Key::Tab, .. }) => Some(Msg::ParamsFormBlur),
            _ => None,
        }
    }
}