pub struct Track {
  pub channels: Vec<Channel>,
}

pub struct Channel {
  
}

// MIDI音符转音高频率
pub fn freq_from_midi_key(key: i32) -> f32 {
  440.0 * 2.0f32.powf((key - 69) as f32 / 12.0)
}