#define CHANNEL_NUM 4 // 电机数，ESP32C3只支持四路PWM

struct Channel {
  int dir_pin; // 方向脚
  int pul_pin; // 脉冲脚
  int pwm_channel; // PWM控制线程
  bool direction; // 电机转动方向
  
  // 初始化针脚
  void init(int pwm_ch) {
    pwm_channel = pwm_ch;
    
    pinMode(dir_pin, OUTPUT);
    pinMode(pul_pin, OUTPUT);
    
    set_direction(false);
    
    ledcSetup(pwm_channel, 2000, 8);
    ledcAttachPin(pul_pin, pwm_channel);
  }


  // 设定电机旋转方向
  void set_direction(bool dir) {
    direction = dir;
    digitalWrite(dir_pin, dir ? HIGH : LOW);
  }
  
  void reverse() {
    set_direction(!direction);
  }
};

Channel* channels = new Channel[CHANNEL_NUM];

void setup() {
  // 这里定义你的输出针脚（GPIO口的编号就是PIN号），注意刷写模式要设置成DIO哦（不然不让跑）
  channels[0] = {
    .dir_pin = 18, 
    .pul_pin = 19
  };
  channels[1] = {
    .dir_pin = 4,
    .pul_pin = 5
  };
  channels[2] = {
    .dir_pin = 2,
    .pul_pin = 3
  };
  channels[3] = {
    .dir_pin = 0,
    .pul_pin = 1
  };
  
  // 打开通信串口 波特率 115200
  Serial.begin(115200);

  // 初始化电机
  for (int i = 0; i < CHANNEL_NUM; i++) {
    channels[i].init(i);
  }

  Serial.print(F("Singing Motor by hexadecimal233!"));
}

// 串行控制电机
void update_status() {
  while (Serial.available() > 0) {
    int packet_id = Serial.parseInt();
    switch (packet_id) {
      case 0: { // 播放音符
        int index = Serial.parseInt();
        float freq = Serial.parseFloat();
        int reverse = Serial.parseInt();
        Channel& channel = channels[index];

        if (freq <= 0) break;

        ledcWriteTone(channel.pwm_channel, freq);
        if (reverse) channel.reverse();
        
        Serial.print(F("Received play: channel="));
        Serial.print(index);
        Serial.print(F("; frequency="));
        Serial.println(freq);
        break;
      }
      case 1: { // 停止音符
        int index = Serial.parseInt();
        Channel& channel = channels[index];
        
        ledcWrite(channel.pwm_channel, 0);

        Serial.print(F("Received stop: channel="));
        Serial.println(index);
        break;
      }
      case 2: { // 获取电机数目
        Serial.println(CHANNEL_NUM);
      }
    }
  }
}

void loop() {
  update_status();
}