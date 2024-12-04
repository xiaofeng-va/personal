## Experimental examples
### Blinky
Connect the MicroUSB with the NUCLEO board, and run the command below:
```
cargo run --package ferox --example blinky
```

You should see the RED led flash and output on your screen.

### UART demo
Wire `GND` / `TX` / `RX` between the board and the TTY-USB board correctly. (similar to [this diagram](https://microcontrollerslab.com/wp-content/uploads/2021/12/STM32-with-FTDI-programmer-connection-diagram.jpg))

Plug both of the USBs (NUCLEO & TTY-USB) into your host machine. If your TTY-USB is use `/dev/ttyUSB0`, you can use command below to communicate with the serial port:
```
picocom /dev/ttyUSB0 -b 115200 --omap crcrlf --imap lfcrlf
```

Then run command below to flash the code into your NUCLEO:
```
cargo run --package ferox --example usart_split
```

You should be able to connect the two devices.
