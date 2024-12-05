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

### CTL200

#### Wiring (STM32 <=> CTL200)
PF6 (RX) <=> CTL200 TX
PF7 (TX) <=> CTL200 RX
GND <=> CTL200 GND

Based on the wiring, you can run command below to connect CTL200 and STM32:
```shell
cargo run --package ferox --example ctl200
```

and below is the expected result:
```
INFO  CTL200 Test Starting!
└─ ctl200::____embassy_main_task::{async_fn#0} @ ferox/examples/ctl200.rs:97  
TRACE USART: presc=1, div=0x0000022c (mantissa = 34, fraction = 12)
└─ embassy_stm32::usart::configure @ /home/xguo/.cargo/registry/src/index.crates.io-6f17d22bba15001f/embassy-stm32-0.1.0/src/fmt.rs:117 
TRACE Using 16 bit oversampling, desired baudrate: 115200, actual baudrate: 115107
└─ embassy_stm32::usart::configure @ /home/xguo/.cargo/registry/src/index.crates.io-6f17d22bba15001f/embassy-stm32-0.1.0/src/fmt.rs:117 
INFO  char_processor started
└─ ctl200::__char_processor_task::{async_fn#0} @ ferox/examples/ctl200.rs:58  
INFO  Sent CRLF
└─ ctl200::__char_processor_task::{async_fn#0}::wait_for_prompt::{async_fn#0} @ ferox/examples/ctl200.rs:78  
INFO  uart_reader started
└─ ctl200::__uart_reader_task::{async_fn#0} @ ferox/examples/ctl200.rs:27  
INFO  uart_reader(): Read buffer: [d, a, 3e, 3e]
└─ ctl200::__uart_reader_task::{async_fn#0} @ ferox/examples/ctl200.rs:46  
INFO  Buffer content after wait_for_prompt: [d, a, 3e, 3e]
└─ ctl200::__char_processor_task::{async_fn#0}::wait_for_prompt::{async_fn#0} @ ferox/examples/ctl200.rs:81  
INFO  Sent 'version' command after initial loop
└─ ctl200::__char_processor_task::{async_fn#0}::wait_for_prompt::{async_fn#0} @ ferox/examples/ctl200.rs:86  
INFO  uart_reader(): Read buffer: [76]
└─ ctl200::__uart_reader_task::{async_fn#0} @ ferox/examples/ctl200.rs:46  
INFO  uart_reader(): Read buffer: [65, 72, 73, 69, 6f, 6e, d, a, 56, 30, 2e, 31, 37, d, a, 3e, 3e]
└─ ctl200::__uart_reader_task::{async_fn#0} @ ferox/examples/ctl200.rs:46  
INFO  PASS
```

Test needs to catch the "PASS" to make sure the case and its corresponding code passes.