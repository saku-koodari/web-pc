// VAL 1
@6  // line 0:  // Sets value 6 A register
D=A // line 1:  // Sets D-register value to ALU D input.
                // Sets A-register value to ALU M/A input.
                // calculate operation, A
                // outputs result to ALU out
                // sets ALU out to D-register

@0  // line 2:  // Sets value 0 to A register
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
    @R3 // line 16: get index
    D=M // line 17

    @R1   // line 18: R1 has max value
    D=D-M // line 19:
    @END  // line 20:
    D;JEQ // line 21:
    
    @R0
    D=M

    // Body of the loop goes here
    // Add 1 to RAM[1] and store the result back in RAM[1]
    // point from R2 to M/A input
    @R2     // line xx // Gets the value of RAM[2], indx should be at M/A
    D=D+M   // line xx // calculates D+M (7 + res)
    M=D     // line xx // updates the value to res

    // Increment the index variable i
    @R3     // line xx
    M=M+1 

    // Jump back to the start of the loop
    @LOOP
    0;JMP

// Endless loop
(END)
    @R2
    D=M

    @END
    0;JMP