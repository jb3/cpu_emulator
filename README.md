# CPU Emulator
Simple CPU emulator written in Rust

The instruction set implemented in this program is from [this page](http://teaching.idallen.com/dat2343/09f/notes/13lmc_opcodes.htm) (only difference is that STO is STA).

# Example programs

## Add two numbers

```
IN
STA 40
IN
STA 41

LDA 40
ADD 41

OUT
HLT
```

## Subtract one number from another

```
IN
STA 40
IN
STA 41

LDA 40
SUB 41

OUT
HLT
```

## Execution

1) Build the binaries
2) `./path/to/binary --compile source.file`
3) `./path/to/binary --run`