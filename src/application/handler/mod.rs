use std::collections::HashMap;

use super::Application;
use crate::errors::*;
mod app;
mod buffer;
mod cursor;
mod monitor;
mod workspace;

pub fn handle_map() -> HashMap<&'static str, fn(&mut Application) -> Result<()>> {
    include!(concat!(env!("OUT_DIR"), "/handle_map"))
}