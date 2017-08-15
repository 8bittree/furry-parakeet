# furry-parakeet
A simple-ish ISA and an implementation.

## Registers

Register | Size | Operand ID
:-   | :-: | :-:
acca | 24b | 001
accb | 24b | 010
accc | 48b | 011
baka | 24b | 101
bakb | 24b | 110
bakc | 48b | 111
ip   | 24b |  -
sp   | 24b |  -
bp   | 24b |  -
flag | 24b |  -

## Instruction Format

### r-class

 - 12b
 - `0` = `0`
 - `c` = opcode
 - `o` = destination operand
 - `p` = second operand

        c0c ccc ooo ppp

### i-class

 - 24b
 - `0` = `0`
 - `1` = `1`
 - `c` = opcode
 - `o` = destination operand
 - `i` = immediate value

        01c ccc ooo iii iii iii iii iii

## Opcode Listing

 Hob\Lob |  000 | 001  | 010  | 011  | 100 | 101  | 110 | 111
   :-:   |  :-: | :-:  | :-:  | :-:  | :-: | :-:  | :-: | :-:
   000   | noop | swp  | push | pop  | add | sub  | mul | div
   001   | jmp  | jz   | jnz  | jlz  | not | and  | or  | xor
   010   | merg | splt |  -   |  -   | add | sub  | mul | div
   011   | jmp  | jz   | jnz  | jlz  | imm | immh |  -  |  -
   100   | int  |  -   | sto  | load |  -  |  -   |  -  |  -
   101   |  -   |  -   |  -   |  -   |  -  |  -   |  -  |  -
   110   |  -   |  -   |  -   |  -   |  -  |  -   |  -  |  -
   111   |  -   |  -   |  -   |  -   |  -  |  -   |  -  |  -

### `noop`

 - Format: r-class
 - Opcode: 000 000
 - Operands: Ignored
 - Description: Does nothing for a cycle

### `swp`

 - Format: r-class
 - Opcode: 000 001
 - Operands:
      - `ooo`: The register pair to swap, according to the `acc` register's ID
      - `ppp`: Ignored
 - Description: Exchanges the values in an `acc` and `bak` register pair.

### `push`

 - Format: r-class
 - Opcode: 000 010
 - Operands:
      - `ooo`: The register to push
      - `ppp`: Ignored
 - Description: Decrements `sp`, then copies value of `ooo` to `[sp]`.

pop

jmp     jz      jnz     jlz

add     sub     mul     div

and     or      not     xor

splt    merg

sto     load

imm     immh

int

_splt: splits `accc` into `acca` (high bits) and `accb` (low bits)_

_merg: inverse of splt_
