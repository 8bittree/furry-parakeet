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

    000 000 000 000 000 000 000 000

    opcode|operand

### r-class

    000 000|000 000

### i-class

    000 000|000 000 000 000 000 000

## Opcode Listing

HOB\LOB |  000 | 001 | 010 | 011 | 100 | 101 | 110 | 111
   :-:  |  :-: | :-: | :-: | :-: | :-: | :-: | :-: | :-:
   000  | noop |     |     |     |     |     |     | 
   001  |      |     |     |     |     |     |     | 
   010  |      |     |     |     |     |     |     | 
   011  |      |     |     |     |     |     |     | 
   100  |      |     |     |     |     |     |     | 
   101  |      |     |     |     |     |     |     | 
   110  |      |     |     |     |     |     |     | 
   111  |      |     |     |     |     |     |     | 

    noop

    jump
    jz
    jnz
    jlz

    add    sub    mul    div

    and    or     not    xor

    psh    pop

    swp

    splt    merg

    sto    lod

    imm

    int

_splt: splits `accc` into `acca` (high bits) and `accb` (low bits)_

_merg: inverse of splt_
