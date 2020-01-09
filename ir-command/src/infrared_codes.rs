/*
 irremocon <https://github.com/ak1211/irremocon>
 Copyright 2019 Akihiro Yamamoto

 Licensed under the Apache License, Version 2.0 (the "License");
 you may not use this file except in compliance with the License.
 You may obtain a copy of the License at

     http://www.apache.org/licenses/LICENSE-2.0

 Unless required by applicable law or agreed to in writing, software
 distributed under the License is distributed on an "AS IS" BASIS,
 WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 See the License for the specific language governing permissions and
 limitations under the License.
*/

///
///
///
use std::error;

pub struct InfraredCodes(Vec<u16>);

impl InfraredCodes {
    ///
    pub fn new() -> InfraredCodes {
        InfraredCodes {
            0: Vec::with_capacity(512), // 数字に深い意味は無い
        }
    }
    ///
    pub fn from_hexdump(hexdump: &str) -> Result<InfraredCodes, Box<dyn error::Error>> {
        /// 4文字から16ビットuintへ
        fn to_u16(cs: &[u8]) -> Result<u16, Box<dyn error::Error>> {
            if cs.len() == 4 {
                let s = String::from_utf8(vec![cs[2], cs[3], cs[0], cs[1]])?;
                Ok(u16::from_str_radix(&s, 16)?)
            } else {
                Err(From::from("Must be lsb-first 32-bit hexadecimal number."))
            }
        }
        //
        hexdump
            .as_bytes()
            .chunks(4)
            .map(|a| to_u16(&a))
            .collect::<Result<Vec<u16>, Box<dyn error::Error>>>()
            .map(|a| InfraredCodes { 0: a })
    }
    ///
    pub fn to_hexdump(&self) -> String {
        /// 16ビットuintから文字列へ
        fn to_str(n: u16) -> String {
            let lo = n & 0x00ff;
            let hi = n >> 8 & 0x00ff;
            format!("{:02X}{:02X}", lo, hi) // 下位バイト先頭(little endian)
        }
        //
        self.0.iter().map(|a| to_str(*a)).collect::<String>()
    }
    ///
    pub fn to_counts(&self) -> &[u16] {
        &self.0
    }
    ///
    pub fn to_micro_seconds(&self) -> Vec<u32> {
        self.0.iter().map(|n| (*n as u32) * 26).collect()
    }
    ///
    pub fn clear(&mut self) {
        self.0.clear();
    }
    ///
    pub fn push(&mut self, v: u16) {
        self.0.push(v);
    }
}

