#![feature(plugin)]
#![plugin(clippy)]

extern crate time;
extern crate rand;
extern crate crypto;
extern crate sxd_document;
extern crate sxd_xpath;
extern crate rustc_serialize;
extern crate byteorder;
extern crate hyper;
#[cfg(test)] #[macro_use]
extern crate yup_hyper_mock as hyper_mock;

mod xmlutil;
mod cryptography;
pub mod messages;
mod events;
mod parser;
mod errors;
pub mod replies;

pub use self::messages::MessageParser;
pub use self::messages::Message;
pub use self::parser::parse_message;
pub use self::errors::WeChatError;
pub use self::cryptography::WeChatCrypto;
pub use self::replies::ReplyRenderer;
pub use self::replies::Reply;
