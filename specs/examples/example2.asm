// Note: This is a bundle of example, not necessary working  per-se

// Example 1: Store number 10 in register D
// D=10

@10 // (line 0) Sets A register to 10
D=A // (line 1) Sets D register to A (addressing)

// Example 2: incrementing value of register D
// D++
D=D+1 // (line 2)
// This works, because `D+1` operation exists in the opcodes.
// You can see the op specs at ` ./specs/README.md `file


// D=RAM[17] 
@17 // (line 3) Sets A register to 17
D=M // (line 4) Sets D register to M (memory)

// RAM[17]=D
@17 // (line 5) Sets A register to 17
M=D // (line 6) Sets M (memory) to D register (reverse)

 // Set RAM[17] to 10
 @10 // (line 7) Sets A register to 10
 D=A // (line 8) Sets D register to A


 // REMEMBER to end your app with an infinite loop
 // in order to prevent appliction to use non-allocated memory
 (END) // ( ) - is a label
   @END // with labels, you don't need to know the line number.
   0;JMP // (line 10) Jump to line 9, causes infinite loop
 
 