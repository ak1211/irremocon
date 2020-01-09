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

use getch::Getch;
use irremocon::infrared_codes::InfraredCodes;
use irremocon::receive_codes::receive_ir_codes;
use rppal::gpio::Gpio;
use std::error::Error;
use std::thread;

// 赤外線受信モジュール電源ピン(GPIO 24 / Physical 18)
const GPIO_IRM_POWER_PIN: u8 = 24;

// 赤外線受信モジュール入力ピン(GPIO 25 / Physical 22)
const GPIO_IRM_INPUT_PIN: u8 = 25;

///
///
///
fn main() -> Result<(), Box<dyn Error>> {
    eprintln!("irrec v{}", env!("CARGO_PKG_VERSION"));
    eprintln!("This program is display infrared codes.");
    //
    eprintln!("");
    eprintln!("<<< Press any key to exit. >>>");
    eprintln!("");
    eprintln!("Please press buttons on your remote control.");
    eprintln!("");

    let mut pin = Gpio::new()?.get(GPIO_IRM_POWER_PIN)?.into_output();
    // 赤外線受信モジュールに電源を供給する。
    pin.set_high();
    // 赤外線受信モジュール入力ピンはラズパイ内蔵プルダウンを利用する
    let input_pin = Gpio::new()?.get(GPIO_IRM_INPUT_PIN)?.into_input_pulldown();

    // リモコン信号スレッドを立てる
    let _handle = thread::spawn(move || {
        let mut ircodes = InfraredCodes::new();
        loop {
            ircodes.clear();
            receive_ir_codes(&input_pin, &mut ircodes).unwrap();
            println!("{}\n", ircodes.to_hexdump())
        }
    });

    // キーボード入力待ち
    let g = Getch::new();
    g.getch()?;

    // 赤外線受信モジュールの電源を切る
    pin.set_low();

    eprintln!("bye.");
    Ok(())
}
