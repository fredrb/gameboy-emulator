# GameBoy Emulator 

> This is a work in progress and it's nowhere near completion.

**(!)** Games and bootrom are not included in this repository

## Run 

```
cargo run <path to rom>
```

## Debugger

A lot of time in this project was spent on a CPU debugger. Enable the debugger property in `Settings.toml`: `debug_enabled = true` to run
the emulator in Debug mode. This allows you to step through the CPU instructions and print registers/memory data. 

```
Debugger started
 - Add breakpoints (b)
 - Add start (s)
> s
Application stopped at breakpoint: 0x0000
> n
Application stopped at breakpoint: 0x0003
> n
Application stopped at breakpoint: 0x0004
> n
Application stopped at breakpoint: 0x0007
> n
Application stopped at breakpoint: 0x0008
> reg
AF: 0x00 0x40 (01000000)
BC: 0x00 0x00
DE: 0xff 0x56
HL: 0x9f 0xfe
SP: 0xfffe
PC: 0x0008
>
```


