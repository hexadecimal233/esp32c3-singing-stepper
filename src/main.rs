use std::collections::HashMap;
use std::time::Duration;

use midly::{num::u7, MidiMessage, Smf};
use nodi::midly;
use nodi::{midly::Format, timers::Ticker, Connection, MidiEvent, Player, Sheet};
use serialport::SerialPort;

fn main() {
    let file = "here.mid"; // MIDI文件
    let port = "COM5"; // 串口名称

    let midi_data = std::fs::read(file).unwrap();
    let Smf { header, tracks } = &Smf::parse(&midi_data).unwrap();

    let timer = Ticker::try_from(header.timing).unwrap();

    let con = MyConnection::new(port);
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
    port: Box<dyn SerialPort>,  // 串口
    pressed: HashMap<i32, i32>, // <channel, key>，用来防止电机已经在转的情况下有别的note打断
}

impl Connection for MyConnection {
    fn play(&mut self, msg: MidiEvent) -> bool {
        match msg.message {
            MidiMessage::NoteOn { key, vel } => {
                if vel == 0 {
                    self.stop_note(0);
                } else {
                    self.play_note(0, key, false);
                }
            }
            MidiMessage::NoteOff { key, vel } => {
                self.stop_note(0);
            }
            _ => {}
        }

        true
    }
}

impl MyConnection {
    fn new(port_name: &str) -> MyConnection {
        MyConnection {
            port: serialport::new(port_name, 115_200)
                .timeout(Duration::from_millis(10))
                .open()
                .expect("Failed to open port"),
            pressed: HashMap::new(),
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
            channel,
            freq_from_midi_key(keyy),
            if reverse { "1" } else { "0" }
        );
        self.port
            .write(play_data.as_bytes())
            .expect("Error writing");
        self.pressed.insert(channel, keyy);
    }

    // 停止音符
    fn stop_note(&mut self, channel: i32) {
        let play_data = format!("{} {}\n", 1, channel);
        self.port
            .write(play_data.as_bytes())
            .expect("Error writing");
        self.pressed.remove(&channel);
    }
}
