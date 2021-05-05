/*
 * Copyright (C) 2021 Jordan DALCQ (Keyboard-Slayer) & Contributors
 *
 * This file is part of Onion.
 *
 * Onion is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * Onion is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with Onion.  If not, see <http://www.gnu.org/licenses/>.
 *
 */

mod assert;
mod load_file;
mod print;

use std::collections::HashMap;
use crate::reader::OnionRet;

pub fn get_builtin() -> HashMap<String, OnionRet>
{
    let mut funcs: HashMap<String, OnionRet> = HashMap::new();

    funcs.insert(String::from("load-file"), OnionRet::Fn(load_file::load_file));
    funcs.insert(String::from("assert"), OnionRet::Fn(assert::assert_scheme));
    funcs.insert(String::from("println"), OnionRet::Fn(print::println));

    funcs
}