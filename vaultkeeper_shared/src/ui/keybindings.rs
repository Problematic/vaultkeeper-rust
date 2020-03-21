use super::Input;
use bracket_lib::prelude::VirtualKeyCode;
use std::collections::HashMap;

pub type Keybindings = HashMap<VirtualKeyCode, Input>;
