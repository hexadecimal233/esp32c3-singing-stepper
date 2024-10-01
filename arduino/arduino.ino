// 这里定义你的输出针脚，注意刷写模式要设置成DIO哦（不然不让跑）
#define DIR 18  // GPIO 18
#define PUL 19  // GPIO 19

#define PULSE_PER_REV 200  // 决定多少个脉冲转360度 (预留)

#define CHANNEL_NUM 1 //电机数

struct Channel {
  bool playing; // 是否正在播放
  float freq; // 音高频率
  int dir_pin; // 方向脚
  int pul_pin; // 脉冲脚
  
  // 初始化针脚
  void init() {
    playing = false;
    freq = 440;
    
    if (dir_pin != 0)  {
      // 设定电机旋转方向
      pinMode(dir_pin, OUTPUT);
      digitalWrite(dir_pin, LOW);
    }
    pinMode(pul_pin, OUTPUT);
  }
};

Channel* channels = new Channel[CHANNEL_NUM];

void setup() {
  channels[0] = {
    .dir_pin = DIR,
    .pul_pin = PUL
  };

  // 打开通信串口 波特率9600
  Serial.begin(9600);

  // 初始化电机
  for (int i = 0; i < CHANNEL_NUM; i++) {
    channels[i].init();
  }

  Serial.println(F("Singing Motor by hexadecimal233!"));
}

// 串行控制电机
void update_status() {
  while (Serial.available() > 0) {
    int packet_id = Serial.parseInt();
    switch (packet_id) {
      case 0: { // 播放音符
        float freq = Serial.parseFloat();
        int index = Serial.parseInt();
        Channel& channel = channels[index];
        channel.freq = freq;
        channel.playing = true;
        
        Serial.print(F("Received play: channel="));
        Serial.print(index);
        Serial.print(F("; frequency="));
        Serial.println(freq);
        break;
      }
      case 1: { // 停止音符
        int index = Serial.parseInt();
        Channel& channel = channels[index];
        channel.playing = false;

        Serial.print(F("Received stop: channel="));
        Serial.println(index);
        break;
      }
    }
  }
}

// 播放音符
void play_note(Channel &channel) {
  if (channel.freq <= 0 || !channel.playing) return;
  int pulse_delay = 1000 * 1000 / (channel.freq * 2);  // microseconds between pulses (half-period)
  digitalWrite(channel.pul_pin, HIGH);
  delayMicroseconds(pulse_delay);
  digitalWrite(channel.pul_pin, LOW);
  delayMicroseconds(pulse_delay);
  update_status();
}

void loop() {
  update_status();
  for (int i = 0; i < CHANNEL_NUM; i++) {
    play_note(channels[i]);
  }
}