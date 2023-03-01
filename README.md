## Run openOCD

````
openocd -f interface/stlink-v2-1.cfg -f target/stm32f3x.cfg
````

run

````
cargo run --target thumbv7em-none-eabihf
````

Then
 
````
target remote :3333
load
continue
````

[See Rust embedded book](https://docs.rust-embedded.org/discovery/f3discovery/)
