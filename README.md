# furry-parakeet
A simple-ish ISA and an implementation.

## Registers

    acca: 24b
    accb: 24b
    accc: 48b
    baka: 24b
    bakb: 24b
    bakc: 48b
    ip  : 24b
    sp  : 24b
    bp  : 24b
    flag: 24b

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

    adda    suba    mula    diva
    addb    subb    mulb    divb
    addc    subc    mulc    divc

    anda    ora     nota    xora
    andb    orb     notb    xorb

    psha    popa
    pshb    popb
    pshc    popc

    swpa
    swpb
    swpc

    splt    merg

    stoa    loda
    stob    lodb
    stoc    lodc

    imm

    int

_splt: splits `accc` into `acca` (high bits) and `accb` (low bits)_
_merg: inverse of splt_
