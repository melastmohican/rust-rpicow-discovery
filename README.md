# rust-rpicow-discovery
Embedded Rust with Raspberry Pi Pico(w)

## Pico Pinout
[Pico Pinout](https://pico.pinout.xyz/)

## Examples
This project uses probe-run to run the examples. Once set up, it should be as simple as `cargo run --example <example name> --release`.

### DHT11 sensor

![DHT11](images/pico_dht11_bb.png)

### HD44780 / 1602A LCD

HD44780 | RPi Pico | Function
--------|-------------|---------
1 (Vss) | Ground (0V) | Powers the HD44780's processor
2 (Vdd) | 5V | Powers the HD44780's processor
3 (V0) | Ground (0V) via a 1K resistor | LCD contrast control: high for nothing, low for aggressive white squares that hate you
4 (RS) | Pi Pico GP15 (pin 20) | Register Select. High to send data, low to send instructions (like 'clear screen').
5 (R/W) | Ground (0V) | Read/write toggle. I've kept it permanently tied low since we only ever output to the display.
6 (E) | Pi Pico GP14 (pin 19) | Enable signal.
11, 12, 13, 14 (DB4-DB7) | GP18, GP19, GP20, GP21 (pins 24-27) | Data pins.
15 (BLA) | 5V | Backlight LED anode.
16 (BLK) | Ground (0V) via 220 ohm resistor| Backlight LED cathode.

![HD44780](images/pico_hd44780_bb.png)

### SSD1306 I2C

![SSD1306](images/pico_ssd1306_bb.png)


### WS2812

15-WS2813 | RPi Pico | Function
--------|-------------|---------
1 GND | Ground (0V) pin 40 | Ground
2 VCC | 5V pin 38 | Power
3 NC | - | Not connected
4 SIG | Pi Pico GP6 (pin 9) |  1-wire serial data connection

![Grove - RGB LED Stick (15-WS2813 Mini)](images/pico_neopixel_bb.png)

### ST7733S

Using [Waveshare 160x80, General 0.96inch LCD display Module](https://www.waveshare.com/0.96inch-lcd-module.htm) with [Rust ST7735 driver](https://github.com/sajattack/st7735-lcd-rs)

![ST7733S](images/pico_st7733s_bb.png)