#![allow(unused)]

use std::time::Duration;

mod midi;

fn main() {
    let mut port = serialport::new("COM5", 9_600)
        .timeout(Duration::from_millis(10))
        .open()
        .expect("Failed to open port");

    print!("成功打开串口");
    
    let packet_id_play: u8 = 0;
    let freq: f32 = 888.0;
    let channel_index_play: u8 = 0;

    let play_data = format!("{} {} {}\n", packet_id_play, freq, channel_index_play);
    port.write(play_data.as_bytes()).expect("写入数据失败");
}
