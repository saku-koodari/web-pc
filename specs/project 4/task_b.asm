// task B - Write a simple interactive program

// write a simple program that listens the keyboard
// if user press down any key, the program fills the sreen with black
// once the user release the key, the screen should go back to white

// declarations start
    // declare pxsize
    @pxsize
    M=M
        // set value to D register
        // value to be drawn
        @32767
        D=A

        // put D register value to pxsize's RAM address
        @pxsize
        M=D

    // declare index for filler
    @index
    M=M
        @SCREEN // index starts from screen RAM[0]
        D=A 
        @index
        M=D 
// declarations send

@FILL_SCREEN
0;JMP

(FILL_SCREEN)
    @pxsize
    D=M
        // pointer
        @index
        A=M

        // pointer value
        // draws the pixel
        M=D

    // increment
    @index
    M=M+1
    D=M

    // compare
    @KBD
    D=A-D
    @FILL_SCREEN
    D;JGT

// Endless loop
(END)
    @END
    0;JMP