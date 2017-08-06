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
    flag: 24b

## Instruction Format

    000 000 000 000 000 000 000 000

    opcode|operand

### r-class

    000 000|000 000

### i-class

    000 000|000 000 000 000 000 000
