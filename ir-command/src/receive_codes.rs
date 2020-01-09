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

use crate::infrared_codes::InfraredCodes;
use rppal::gpio::{InputPin, Level};
use std::error::Error;
use std::time::Duration;
use timerfd::{SetTimeFlags, TimerFd, TimerState};

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
pub fn receive_ir_codes(
    pin: &InputPin,
    ircodes_buffer: &mut InfraredCodes,
) -> Result<(), Box<dyn Error>> {
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
