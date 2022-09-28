#![no_std]
#![feature(type_alias_impl_trait)]

mod base;
mod attrs;
mod msg;
mod raw;
mod typed;

use base::*;
use attrs::*;
use msg::*;

pub mod rfc3489;
pub mod rfc5389;