## Run openOCD

````
openocd -f interface/stlink-v2-1.cfg -f target/stm32f3x.cfg
````

run

````
cargo run --release
````

If needed change gdb init sequence in ``openocd.gdb``.


[See Rust embedded book](https://docs.rust-embedded.org/discovery/f3discovery/)
