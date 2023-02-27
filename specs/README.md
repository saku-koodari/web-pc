# Spepcs - ASM

I started this project as RUST WASM, in order to learn both, computer architecture and Web Assembly with Rust. However, Assembly parts are more efficient to learn using nand2tetris.org course material. Otherwise I would have to spend lot's of time building the emulating environment, writing asm compiler, etc, before moving on with the computer. Once this assembly part is done, then I will plan to go back to writing Rust code. Now it's time for writing some asm.


## Hack Asm - Buil-in symbols

+--------+-------+---------------------+
| symbol | value | meaning             |
+--------+-------+---------------------+
|   R0   |   0   | Virtual register 0  |
|   R1   |   1   | Virtual register 1  |
|   R2   |   2   | Virtual register 2  |
|  ...   |  ...  |                     |
|  R15   |  15   | Virtual register 15 |
| SCREEN | 16384 | base addresse of... |
|  KBD   | 24576 | ...I/O memory maps. |
|   SP   |   0   |  TBA...             |
|  LCL   |   1   |  TBA...             |
|  ARG   |   2   |  TBA...             |
|  THIS  |   3   |  TBA...             |
|  THAT  |   4   |  TBA...             |
+--------+-------+---------------------+

