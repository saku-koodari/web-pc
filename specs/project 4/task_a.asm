// VAL 1
@6  // line 0: 
D=A // line 1:
@0  // line 2:
M=D // line 3:

// VAL 2
@7  // line 4:
D=A // line 5:
@1  // line 6:
M=D // line 7:

// RES
@0  // line 08:
D=A // line 09:
@2  // line 10:
M=D // line 11:

// INDEX
@0  // line 12:
D=A // line 13:
@R3 // line 14:
M=D // line 15:


// Start of loop
(LOOP)
    // Check if the index variable i is less than the number of iterations
    @R1   // line 16: // loads the value stored in RAM[1] into the D register.
    D=M   // line 17: 
    @R2   // line 18: // loads the value stored in RAM[2] into the A register, 
                      // and then loads that value into the D register.
    
    // toimii väärin. 
    // lataa A-rekisteriin @2, koska R2
    D=D-M // line 19  //  subtracts the value stored in RAM[2] from the value stored in RAM[1].
    @END  // line 20
    D;JEQ // line 21

    // Body of the loop goes here
    // Add 1 to RAM[1] and store the result back in RAM[1]
    @1  // line 22
    D=M     // line 23 
    @1      // line 24
    M=D+1   // line 25

    // Increment the index variable i
    @R1     // line 26
    M=M+1   // line 27


    // Perform loop body here
    @0
    D=M
    @2
    D=D+M
    @2
    M=D


    // Jump back to the start of the loop
    @LOOP
    0;JMP

// Endless loop
(END)
    @END
    0;JMP