# computer-emulator
Emulator written in Rust for my own 4-bit architecture. My hope is to one day build this architecture, or a simpler, stripped-down version, in real life or some sort of logic simulator.

## Project Structure
The folder structure looks roughly as follows: \
. \
├── `assembler` - contains the assembler that can be used to generate binaries to load into the emulator \
├── `common` - contains common pieces between the assembler and emulator like architectural information and utility structs. \
├── `emulator` - contains the emulator itself and some external devices, which can be simulated along with the emulator. \
└── `programs` - contains programs that can be assembled and run.

## Architecture
The design was inspired somewhat by the TMS1000 series. There are four general-purpose registers, including the accumulator. Using two of them registers, up to 256 nibbles of RAM can be addressed. The architecture also supports up to 1024 bytes (1KB) of ROM (64 bytes within a page, for a total of 1024 across 16 pages). Port-mapped GPIO is also possible with 4, 4-bit ports and 4 single-bit pins. Finally, though the architecture doesn't have a stack, it supports calling one subroutine at a time.

### Registers

| Registers             | Description                                                   | Code                                  |
| --------------------- | ------------------------------------------------------------- | ------------------------------------- |
| A(ccumulator)         | Stores results from the ALU                                   | 00                                    |
| X                     | Working register, also used to address high end memory        | 01                                    |
| Y                     | Working register, also used to address low end memory         | 10                                    |
| Z                     | Working register, also used for I/O                           | 11                                    |
| P(rogram) C(ounter)   | Used to address program memory within a page. 6-bit register. | N/A (controlled by jump instructions) |
| P(age) A(ddress)      | Used to address pages. 4-bit register.                        | N/A (controlled by jump instructions) |
| P(age) B(uffer)       | Used to change PA on jump. 4-bit register.                    | N/A (controlled by LPB only)          |
| S(ubroutine) B(uffer) | Used for returning from subroutines. 6-bit register.          | N/A (controlled by jump instructions) |

### Instructions and OpCodes

| Instruction | Binary   | Description                                                                                                                                                                                             |
| ----------- | -------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |                                                                                                                                                                      
| STR         | 001000rr | Stores the result of register rr in the memory location pointed to by XY                                                                                                                                |
| LOD         | 001001rr | Loads register rr from the memory location pointed to by XY                                                                                                                                             |
| LDI         | 11rrxxxx | Loads an immediate value into register rr                                                                                                                                                               |
| INC         | 001010rr | Increments register rr                                                                                                                                                                                  |
| DEC         | 001011rr | Decrements register rr                                                                                                                                                                                  |
| MOV         | 0100rrkk | Moves the value in register rr to register kk                                                                                                                                                           |                                                                                                                                                                      
| INP         | 011100pp | Reads the value of port pp into Z                                                                                                                                                                       |
| OUT         | 011101pp | Writes the value of Z to port pp                                                                                                                                                                        |
| SEP         | 011111qq | Sets pin qq                                                                                                                                                                                             |
| RSP         | 011110qq | Resets pin qq                                                                                                                                                                                           |
| ADD         | 010100rr | Adds rr to A. Sets the status flag if the result overflows                                                                                                                                              |
| SUB         | 010101rr | Subtracts rr from A. Sets the status flag if the result underflows                                                                                                                                      |
| BOR         | 010110rr | Bitwise ORs rr with A                                                                                                                                                                                   |
| AND         | 010111rr | Bitwise ANDs rr with the A                                                                                                                                                                              |
| NOT         | 00000100 | Not of A                                                                                                                                                                                                |
| SHR         | 00000101 | Logical shift right of A                                                                                                                                                                                |
| SHL         | 00000110 | Logical shift left of A                                                                                                                                                                                 |
| GRT         | 011000rr | Compares rr with A. Sets the status flag if A is greater                                                                                                                                                |
| LES         | 011001rr | Compares rr with A. Sets the status flag if A is lesser                                                                                                                                                 |
| CMP         | 011010rr | Compares rr with A. Sets the status flag if they are equal                                                                                                                                              |
| BRN         | 10xxxxxx | Jumps to the immediate memory address in the page stored in PB if the status flag is set. If the subroutine jump flag is set, PB is ignored and a subroutine jump within the current page is performed. |
| LPB         | 0001xxxx | Loads the immediate into the page buffer.                                                                                                                                                               |
| SSJ         | 00000001 | Sets the subroutine jump flag                                                                                                                                                                           |
| RSJ         | 00000111 | Resets the subroutine jump flag                                                                                                                                                                         |
| RET         | 00001000 | Returns from subroutine, continuing execution at the location in SB.                                                                                                                                    |                                                                                                                                                                      
| SSF         | 00000011 | Sets the status flag                                                                                                                                                                                    |
| RSF         | 00000010 | Resets the status flag (sets it to 0)                                                                                                                                                                   |
| NOP         | 00000000 | Literally does nothing                                                                                                                                                                                  |
