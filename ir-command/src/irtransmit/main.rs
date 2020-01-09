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

use irremocon::infrared_codes::InfraredCodes;
use irremocon::transmit_codes::transmit_ir_codes;
use std::error::Error;
use std::thread;
use std::time::Duration;

///
///
///
fn main() -> Result<(), Box<dyn Error>> {
    eprintln!("irtransmit v{}", env!("CARGO_PKG_VERSION"));

    let mut buffer = String::new();
    // EndOfFileまで繰り返す
    while std::io::stdin().read_line(&mut buffer)? > 0 {
        let line = buffer.trim();
        if !line.is_empty() {
            let ircodes = InfraredCodes::from_hexdump(line)?;
            transmit_ir_codes(ircodes)?;
        }
        buffer.clear();
        // 送信間隔をあける
        thread::sleep(Duration::from_millis(1));
    }

    Ok(())
}
