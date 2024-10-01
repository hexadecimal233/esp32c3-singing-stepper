use std::collections::HashMap;
use std::time::Duration;

use midly::{num::u7, MidiMessage, Smf};
use nodi::midly;
use nodi::{midly::Format, timers::Ticker, Connection, MidiEvent, Player, Sheet};
use serialport::SerialPort;

fn main() {
    let file = r"D:\Aoharu.mid"; // MIDI文件
    let port = "COM5"; // 串口名称
    let motor_count = 1;

    let midi_data = std::fs::read(file).unwrap();
    let Smf { header, tracks } = &Smf::parse(&midi_data).unwrap();

    let timer = Ticker::try_from(header.timing).unwrap();

    let con = MyConnection::new(port, motor_count);
    let sheet = match header.format {
        Format::SingleTrack | Format::Sequential => Sheet::sequential(&tracks),
        Format::Parallel => Sheet::parallel(&tracks),
    };

    let mut player = Player::new(timer, con);
    player.play(&sheet);
}

// 音高转频率
fn freq_from_midi_key(key: i32) -> f32 {
    440.0 * 2.0f32.powf((key - 69) as f32 / 12.0)
}

struct MyConnection {
    port: Box<dyn SerialPort>, // 串口
    pressed: HashMap<i32, u7>, // <channel, key>，用来防止电机已经在转的情况下有别的note打断
    motor_count: i32,          // 步进电机数目
}

impl Connection for MyConnection {
    fn play(&mut self, msg: MidiEvent) -> bool {
        let channel = u8::from(msg.channel) as i32;
        match msg.message {
            MidiMessage::NoteOn { key, vel } => {
                if vel == 0 {
                    self.stop_note(channel, key);
                } else {
                    self.play_note(channel, key, true);
                }
            }
            MidiMessage::NoteOff { key, vel } => {
                self.stop_note(channel, key);
            }
            _ => {}
        }

        true
    }
}

impl MyConnection {
    fn new(port_name: &str, motor_count: i32) -> MyConnection {
        MyConnection {
            port: serialport::new(port_name, 115_200)
                .timeout(Duration::from_millis(10))
                .open()
                .expect("Failed to open port"),
            pressed: HashMap::new(),
            motor_count,
        }
    }

    // 播放音符，参数分别是：电机编号，音高，是否让电机反转
    fn play_note(&mut self, channel: i32, key: u7, reverse: bool) {
        if self.pressed.contains_key(&channel) {
            return;
        }

        let keyy = u8::from(key) as i32;
        let play_data = format!(
            "{} {} {} {}\n",
            0,
            channel % self.motor_count, // 电机编号
            freq_from_midi_key(keyy),
            if reverse { "1" } else { "0" }
        );
        self.port
            .write(play_data.as_bytes())
            .expect("Error writing");
        self.pressed.insert(channel, key);
    }

    // 停止音符
    fn stop_note(&mut self, channel: i32, key: u7) {
        if let Some(that_key) = self.pressed.get(&channel) {
            if that_key == &key {
                let play_data = format!("{} {}\n", 1, channel % self.motor_count);
                self.port
                    .write(play_data.as_bytes())
                    .expect("Error writing");
                self.pressed.remove(&channel);
            }
        }
    }
}
