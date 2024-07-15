use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::{
    fs::File,
    io::{BufReader, BufWriter, Write},
};
use vizia::prelude::*;

#[derive(Lens, Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Spell {}
