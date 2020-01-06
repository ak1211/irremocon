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
use irremocon::lib::InfraredCodes;
use rppal::gpio::{Gpio, Level};
use std::error::Error;
use std::thread;
use std::time::Duration;
use timerfd::{SetTimeFlags, TimerFd, TimerState};

// 赤外線受信モジュール電源ピン(GPIO 24 / Physical 18)
const GPIO_IRM_POWER_PIN: u8 = 24;

// 赤外線受信モジュール入力ピン(GPIO 25 / Physical 22)
const GPIO_IRM_INPUT_PIN: u8 = 25;

// 赤外線受信モジュールは負論理信号
const ASSERT_IR: Level = Level::Low;
const NEGATE_IR: Level = Level::High;

// キャリア周波数[kHz}
// 38kHz
const CARRIER_FREQ_KHZ: u16 = 38;

// キャリア周期[us]
// 1/38,000 * 1,000,000 = 26us
const CARRIER_PERIOD_MICROS: u16 = 1000 / CARRIER_FREQ_KHZ;

//
// ラズパイゼロではキャリア周期に同期できず
// タイミング違反が起きるので分周する
//

// プリスケーラの倍率
// 2分周
const N_OF_PRESCALER: u16 = 2;

// 分周後でのクロックカウンタ増加量
const COUNT_PACE: u16 = N_OF_PRESCALER;

// タイマー周期[us]
// キャリア周期 * プリスケーラの倍率
const TIMER_INTERVAL_MICROS: u16 = CARRIER_PERIOD_MICROS * N_OF_PRESCALER;

// この時間信号が変化しないと, 赤外線リモコン信号を読み取るのを終了する
// 34ms
const TIMEOUT_COUNTS: u16 = 34 * 1000 / TIMER_INTERVAL_MICROS;

///
/// 赤外線リモコン信号を読み取る
///
fn receive_ir_codes(ircodes_buffer: &mut InfraredCodes) -> Result<(), Box<dyn Error>> {
    // 赤外線受信モジュール入力ピンはラズパイ内蔵プルダウンを利用する
    let pin = Gpio::new()?.get(GPIO_IRM_INPUT_PIN)?.into_input_pulldown();
    //
    let state = TimerState::Periodic {
        current: Duration::from_micros(1),
        interval: Duration::from_micros(TIMER_INTERVAL_MICROS as u64),
    };
    let mut timerfd = TimerFd::new()?;

    timerfd.set_state(state, SetTimeFlags::Default);

    // リモコン信号入力待ち
    while pin.read() == NEGATE_IR {
        timerfd.read(); // タイマー待ち
    }

    // リモコン信号を検出したのでカウント開始
    let mut previous: Level = ASSERT_IR;
    let mut count: u16 = 0;
    while count < TIMEOUT_COUNTS {
        if previous == pin.read() {
            // 信号が変化しないならカウンタを増やす
            count += COUNT_PACE;
        } else {
            // 信号が変化したら
            // カウント値をバッファに入れて
            // カウンタを初期化
            ircodes_buffer.push(count);
            previous = !previous;
            count = 0;
        }
        timerfd.read(); // タイマー待ち
    }
    // 最後のカウント値をバッファに入れる
    ircodes_buffer.push(count);
    Ok(())
}

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

    // リモコン信号スレッドを立てる
    let _handle = thread::spawn(|| {
        let mut ircodes = InfraredCodes::new();
        loop {
            ircodes.clear();
            receive_ir_codes(&mut ircodes).unwrap();
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
