# CPU Emulator
Simple CPU emulator written in Rust

The instruction set implemented in this program is from [this page](http://teaching.idallen.com/dat2343/09f/notes/13lmc_opcodes.htm) (only difference is that STO is STA).

## Instruction list

| Name     | Code     | Opcode | Usage                                                    |
|----------|----------|--------|----------------------------------------------------------|
| Load     | `LDA XY` | 1XY    | Load the memory address of XY into the accumulator       |
| Store    | `STA XY` | 2XY    | Store accumulator value in memory address XY             |
| Add      | `ADD XY` | 3XY    | Add value of memory address XY to accumulator            |
| Subtract | `SUB XY` | 4XY    | Subtract the value of memory address XY from accumulator |
| Input    | `IN`     | 500    | Take user input and store in accumulator                 |
| Output   | `OUT`    | 600    | Write value of accumulator to shell                      |
| Halt     | `HLT`    | 700    | Halt execution of the program                            |
| Set      | `SET XY` | 8XY    | Set the accumulator to memory address XY                 |
| Jump     | `JMP XY` | 9XY    | Change the program counter to memory address XY          |

## Example programs

### Add two numbers

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

### Subtract one number from another

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