# CPU Emulator
Simple CPU emulator written in Rust

The instruction set implemented in this program is from [this page](http://teaching.idallen.com/dat2343/09f/notes/13lmc_opcodes.htm) (only difference is that STO is STA).

## Instruction list

| Name     | Code         | Opcode | Definition                                                                   |
|----------|--------------|--------|------------------------------------------------------------------------------|
| Load     | `LDA XY`     | 1XY    | Load the memory address of XY into the accumulator                           |
| Store    | `STA XY`     | 2XY    | Store accumulator value in memory address XY                                 |
| Add      | `ADD XY`     | 3XY    | Add value of memory address XY to accumulator                                |
| Subtract | `SUB XY`     | 4XY    | Subtract the value of memory address XY from accumulator                     |
| Input    | `IN`         | 500    | Take user input and store in accumulator                                     |
| Output   | `OUT`        | 600    | Write value of accumulator to shell                                          |
| Halt     | `HLT`        | 700    | Halt execution of the program                                                |
| Set      | `SET XY`     | 8XY    | Set the loaded memory address (loaded via LDA) to the memory address of XY   |
| Jump     | `JMP XY`     | 9XY    | Change the program counter to memory address XY                              |
| Call     | `CALL LABEL` | N/A    | This instruction internally changes into a `JMP XY` call to jump to a label. |

## Example programs

### Add two numbers

```
MAIN:
	CALL GETINPUTS
    CALL ADDNUMS
    CALL PRINTOUT

GETINPUTS:
	IN
	STA 50
	IN
	STA 51

ADDNUMS:
	LDA 50
	ADD 51
	STA 52

PRINTOUT:
	LDA 52
	OUT
	HLT
```

### Subtract one number from another

```
MAIN:
	CALL GETINPUTS
    CALL SUBTRACT
    CALL PRINTOUT

GETINPUTS:
	IN
	STA 50
	IN
	STA 51

SUBTRACT:
	LDA 50
	SUB 51
	STA 52

PRINTOUT:
	LDA 52
	OUT
	HLT
```

### Infinite get input & print loop

```
MAIN:
	CALL LOOP

LOOP:
	IN
	OUT
	CALL LOOP
```

### Pre-defining data

```
MAIN:
	LDA 50
	ADD 51
	OUT
	HLT

50 DAT 10
51 DAT 11
```

The above code means that at address 50, the number 10 will be stored, and at address 51, the number 11 will be stored. Upon execution 50 will be loaded into the accumulator, 51 will then be added to the accumulator and the accumulator will be outputted. Followed by a halt instruction.

## Execution

1) Build the binaries
2) `./path/to/binary --compile source.file target.file`
3) `./path/to/binary --run target.file`
