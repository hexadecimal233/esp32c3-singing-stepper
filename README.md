# esp32c3-singng-stepper

使用 ESP32-C3 开发板与 42 步进电机驱动的 MIDI 音乐播放器！

## 如何创建支持本程序的 MIDI 文件

下方教程使用`FL Studio`作为示例，其他支持 MIDI 导出和编辑的软件需要按需调整

1. 创建 FL 工程，然后添加 **和你电机数目一样的** MIDI Out，每个通道对应一个电机
   > （若需预览结果可 MIDI Out 连到其他的合成器上）
2. 将自己的 MIDI 导入进来然后对每个音轨进行适配（最好选用 Lead (& Lead Vocal)，Bass，Arp，Chrod 等，效果最好）
3. 导出中选择 MIDI 文件，正常导出就得到可以由本程序播放的 MIDI 文件

P.S. 如果要控制电机反转，将 MIDI Out 的端口号设为`电机数 + 1`即可，触发的音符将使电机反转

## 食用方法

1. 运行程序，输入 MIDI 路径和开发板串口设备名（Windows 是 COM+一个数字，Linux 是/dev/ttyUSB+一个数字）
2. 开玩！

> 注意！C5 以下的音符可能会出现类似嘶哑的声音

## 开发者文档

数据包格式：
`<packet_id> <data>`

- packet_id = 0: 播放音符
  data = `<index: 电机编号，从0开始> <freq: 音符频率> <reverse: 是否改变电机旋转方向>`

- packet_id = 1: 停止播放音符
  data = `<index: 电机编号，从0开始>`

- packet_id = 2: 获取电机数目
  data 无需任何参数，返回电机数目

> 注意！串口接收数据有较大延迟，获取电机数目可能需要等 1 秒左右，有时候还会超时（）

