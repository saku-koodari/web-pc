// Example 3. 3 poinsters with array

// for (i=0; i<n; i++) {
//       arr[i] = -1
// }
// // fills -1 to the array

    // Suppose that arr=100 and n=10

    // arr = 100
    @100 // variable araay is set at RAM[ 16]
    D=A  // and it has value of 100 
    @arr // value 16, because it was compiled by the asm compiler
    M=D

    // n=10
    @10 // next variable is set at RAM[17], 
    D=A // since according to asm compiler, 
    @n  //it was the next free memory register
    M=D

    // initialize i = 0
    @i  // i is set at RAM[18]
    M=0

// DISCLAIMER: This is example
// I haven't tested this
(LOOP)     
    // if (i==n) goto END
    @i
    D=M
    @n
    D=M-D
    @END
    D;JEQ

    // RAM[arr+i] = -1
    @arr
    D=M
    @i
    A=D+M
    M=-1

    // i++
    @i
    M=M+1

    @LOOP
    0;JMP

(END)
    @END
    0;JMP
