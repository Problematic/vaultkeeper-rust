use bracket_lib::prelude::VirtualKeyCode;
use std::collections::HashMap;

pub type Keymap<T> = HashMap<VirtualKeyCode, T>;
