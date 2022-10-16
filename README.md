# write-driver-template

write a embedded driver template and examples by esp32-c3

## 创建一个 lib 项目

```
cargo new --lib driver-template
```

## 修改 `cargo.toml`

1. 添加依赖 `embedded-hal`

```
[dependencies]
embedded-hal = "0.2.7"

[dev-dependencies]
esp32c3-hal = "0.2.0"
riscv-rt = { version = "0.9" }
esp-backtrace = { version = "0.3.0", features = [
  "esp32c3",
  "panic-handler",
  "exception-handler",
  "print-uart",
] }
```

2. 新建一个 `example`
   `mkdir examples && vim example.rs`

```
#![no_std]
#![no_main]

use esp32c3_hal::{clock::ClockControl, pac::Peripherals, prelude::*, timer::TimerGroup, Rtc};
use esp_backtrace as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    // Disable the RTC and TIMG watchdog timers
    let mut rtc = Rtc::new(peripherals.RTC_CNTL);
    let timer_group0 = TimerGroup::new(peripherals.TIMG0, &clocks);
    let mut wdt0 = timer_group0.wdt;
    let timer_group1 = TimerGroup::new(peripherals.TIMG1, &clocks);
    let mut wdt1 = timer_group1.wdt;

    rtc.swd.disable();
    rtc.rwdt.disable();
    wdt0.disable();
    wdt1.disable();

    loop {}
}
```

3. 添加编译的平台
   `mkdir .cargo && vim config.toml`

```
[target.riscv32imac-unknown-none-elf]
runner = "espflash --monitor"

[build]
rustflags = [
  # Required to obtain backtraces (e.g. when using the "esp-backtrace" crate.)
  # NOTE: May negatively impact performance of produced code
  "-C", "force-frame-pointers",

  "-C", "link-arg=-Tlinkall.x",
]
target = "riscv32imac-unknown-none-elf"

[unstable]
build-std = ["core"]
```

## 测试

### 开始之前准备

1. 在电脑中下载启动 wokwi 的命令 [wokwi-server](https://github.com/MabezDev/wokwi-serv

### 开始测试

```
make run
```
