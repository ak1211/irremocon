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

use irremocon::infrared_codes::{InfraredCodes};
use rppal::pwm::{Channel, Polarity, Pwm};
use std::error::Error;
use std::thread;
use std::time::Duration;
use timerfd::{SetTimeFlags, TimerFd, TimerState};

// キャリア周波数[kHz}
// 38kHz
const CARRIER_FREQ_KHZ: u16 = 38;

// キャリア周期[us]
// 1/38,000 * 1,000,000 = 26us
const CARRIER_PERIOD_MICROS: u16 = 1000 / CARRIER_FREQ_KHZ;

// 1/3 duty cycle
const ON_DUTY_MICROS: u16 = CARRIER_PERIOD_MICROS / 3;

///
/// リモコン信号送信
///
fn transmit_ir_codes(ircodes: InfraredCodes) -> Result<(), Box<dyn Error>> {
    // 赤外線LED出力は正論理信号
    // 赤外線LED出力ピン (GPIO 18 / Physical 12)
    let pwm = Pwm::with_period(
        Channel::Pwm0,
        Duration::from_micros(CARRIER_PERIOD_MICROS as u64),
        Duration::from_micros(ON_DUTY_MICROS as u64),
        Polarity::Normal,
        false,
    )?;

    let mut timerfd = TimerFd::new()?;
    // リモコン信号出力
    for onoff in ircodes.to_micro_seconds().chunks(2) {
        let t_on = onoff[0];
        let t_off = onoff[1];
        timerfd.set_state(
            TimerState::Oneshot(Duration::from_micros(t_on as u64)),
            SetTimeFlags::Default,
        );
        pwm.enable()?;
        timerfd.read(); // タイマー待ち
                        //
        timerfd.set_state(
            TimerState::Oneshot(Duration::from_micros(t_off as u64)),
            SetTimeFlags::Default,
        );
        pwm.disable()?;
        timerfd.read(); // タイマー待ち
    }

    Ok(())
}

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
