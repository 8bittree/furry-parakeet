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
 - `-` = reserved
 - `0` = `0`
 - `c` = opcode
 - `o` = destination operand
 - `p` = second operand

        c0c ccc ooo ppp

### i-class

 - 24b
 - `-` = reserved
 - `0` = `0`
 - `1` = `1`
 - `c` = opcode
 - `o` = destination operand
 - `i` = immediate value

        -1c ccc ooo iii iii iii iii iii

## Opcode Listing

Hob\Lob |  000 | 001 |  010 |  011 | 100 | 101 | 110 | 111
   :-:  |  :-: | :-: |  :-: |  :-: | :-: | :-: | :-: | :-:
   00   | noop | swp | push |  pop | add | sub | mul | div
   01   | jmpr | jzr | jnzr | jlzr | not | and | or  | xor
   10   |      |     |      |      | add | sub | mul | div
   11   | jmpi | jzi | jnzi | jlzi |     |     |     | 

    noop

    jmpi    jmpr
    jzi     jzr
    jnzi    jnzr
    jlzi    jlzr

    add     sub     mul     div

    and     or      not     xor

    push     pop

    swp

    splt    merg

    sto     lod

    imm

    int

_splt: splits `accc` into `acca` (high bits) and `accb` (low bits)_

_merg: inverse of splt_
