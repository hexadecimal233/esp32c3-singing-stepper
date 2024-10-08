use std::collections::HashMap;
use std::io::Write;
use std::time::Duration;

use midly::{num::u7, MidiMessage, Smf};
use nodi::midly;
use nodi::{midly::Format, timers::Ticker, Connection, MidiEvent, Player, Sheet};

fn main() {
    let mut buf = String::new();

    // MIDI文件
    print!("MIDI文件路径: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut buf).unwrap();
    let file = buf.trim().to_string();
    buf.clear();

    // 串口设备（本人这里是COM5）
    print!("串口设备名: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut buf).unwrap();
    let port = buf.trim().to_string();
    buf.clear();

    // 电机数量
    // TODO: 发包让esp32返回步进电机数目
    print!("步进电机数量: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut buf).unwrap();
    let motor_count = buf.trim().to_string().parse::<i32>().unwrap();
    buf.clear();

    let midi_data = std::fs::read(&file).unwrap();
    let Smf { header, tracks } = &Smf::parse(&midi_data).unwrap();

    let timer = Ticker::try_from(header.timing).unwrap();

    let con = MyConnection::new(&port, motor_count);
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
    port: Box<dyn serialport::SerialPort>, // 串口
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
                    self.play_note(channel, key);
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

    // 播放音符，参数分别是：电机编号，音高
    fn play_note(&mut self, channel: i32, key: u7) {
        if self.pressed.contains_key(&channel) {
            return;
        }

        let reverse = if channel >= self.motor_count {
            "1"
        } else {
            "0"
        };

        let play_data = format!(
            "{} {} {} {}\n",
            0,
            channel % self.motor_count, // 电机编号
            freq_from_midi_key(u8::from(key) as i32),
            reverse
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
