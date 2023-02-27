# Specs - ASM

I started this project as RUST WASM, in order to learn both, computer architecture and Web Assembly with Rust. However, Assembly parts are more efficient to learn using nand2tetris.org course material. Otherwise I would have to spend lot's of time building the emulating environment, writing asm compiler, etc, before moving on with the computer. Once this assembly part is done, then I will plan to go back to writing Rust code. Now it's time for writing some asm.

## Hack Asm - Buil-in symbols




## Set

Set the A register to value:

```
@value
```

value is less tahn 32767

## instructions

symbolic: dest = comp; jump

binary: 1 1 1 a c1 c2 c3 c4 c5 c6 d1 d2 d3 j1 j2 j3

### Comp - compare

| comp |
| a | b | c1 | c2 | c3 | c4 | c5 | c6 |
|-----|-----|----|----|----|----|----|----|
| 0 | | 1 | 0 | 1 | 0 | 1 | 0 |
| 1 | | 1 | 1 | 1 | 1 | 1 | 1 |
| -1 | | 1 | 1 | 1 | 0 | 1 | 0 |
| D | | 0 | 0 | 1 | 1 | 0 | 0 |
| A | M | 1 | 1 | 0 | 0 | 0 | 0 |
| !D | | 0 | 0 | 1 | 1 | 0 | 1 |
| !A | !M | 1 | 1 | 0 | 0 | 0 | 1 |
| -D | | 0 | 0 | 1 | 1 | 1 | 1 |
| -A | -M | 1 | 1 | 0 | 0 | 1 | 1 |
| D+1 | | 0 | 1 | 1 | 1 | 1 | 1 |
| A+1 | M+1 | 1 | 1 | 0 | 1 | 1 | 1 |
| D-1 | | 0 | 0 | 1 | 1 | 1 | 0 |
| A-1 | M-1 | 1 | 1 | 0 | 0 | 1 | 0 |
| D+A | D+M | 0 | 0 | 0 | 0 | 1 | 0 |
| D-A | D-M | 0 | 1 | 0 | 0 | 1 | 1 |
| A-D | M-D | 0 | 0 | 0 | 1 | 1 | 1 |
| D&A | D&M | 0 | 0 | 0 | 0 | 0 | 0 |
| D|A | D|M | 0 | 1 | 0 | 1 | 0 | 1 |

### dest = destination

| dest | d1  | d2  | d3  | effect                             |
| ---- | --- | --- | --- | ---------------------------------- |
| null | 0   | 0   | 0   | The value is not stored            |
| M    | 0   | 0   | 1   | RAM[A]                             |
| D    | 0   | 1   | 0   | D register                         |
| MD   | 0   | 1   | 1   | RAM[A] and D register              |
| A    | 1   | 0   | 0   | A register                         |
| AM   | 1   | 0   | 1   | A register and RAM[D]              |
| AD   | 1   | 1   | 0   | A register and R register          |
| AMD  | 1   | 1   | 1   | A register, RAM[A], and D register |

### jump = jump

| jump | j1  | j2  | j3  | effect             |
| ---- | --- | --- | --- | ------------------ |
| null | 0   | 0   | 0   | no jump            |
| JGT  | 0   | 0   | 1   | if out > 0 jump    |
| JEQ  | 0   | 1   | 0   | if out = 0 jump    |
| JGE  | 0   | 1   | 1   | if out >= 0 jump   |
| JLT  | 1   | 0   | 0   | if out < 0 jump    |
| JNE  | 1   | 0   | 1   | if out != 0 jump   |
| JLE  | 1   | 1   | 0   | if out <= 0 jump   |
| JMP  | 1   | 1   | 1   | unconditional jump |

For example, `D + 1`. Check the previous table, find the row, then map the binaries with the letters

`D+1 = 011111`
=> `1 1 1 a 0 1 1 1 1 1 d1 d2 d3 j1 j2 j3`

#####

| symbol | value | meaning                           |
| ------ | ----- | --------------------------------- |
| R0     | 0     | Virtual register 0                |
| R1     | 1     | Virtual register 1                |
| R2     | 2     | Virtual register 2                |
| ...    | ...   |                                   |
| R15    | 15    | Virtual register 15               |
| SCREEN | 16384 | base addresse of I/O memory maps. |
| KBD    | 24576 | base addresse of I/O memory maps. |
| SP     | 0     | TBA...                            |
| LCL    | 1     | TBA...                            |
| ARG    | 2     | TBA...                            |
| THIS   | 3     | TBA...                            |
| THAT   | 4     | TBA...                            |
