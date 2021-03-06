# furry-parakeet
A ~~simple-ish~~ small-ish, overly complicated, and poorly thought-out ISA and
maybe an implementation.

## Registers

Register | Size | Operand ID | Octal
:-   | :-: | :-: | :-:
acca | 24b | 001 | 1
accb | 24b | 010 | 2
accc | 48b | 011 | 3
baka | 24b | 101 | 5
bakb | 24b | 110 | 6
bakc | 48b | 111 | 7
ip   | 24b |  -  | -
sp   | 24b |  -  | -
bp   | 24b |  -  | -
flag | 24b |  -  | -

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

*r-class*

**i-class**

 Hob\Lob |    000   |   001    |   010    |   011    |   100   |   101    |   110   |   111
   :-:   |    :-:   |   :-:    |   :-:    |   :-:    |   :-:   |   :-:    |   :-:   |   :-:
   000   |  *noop*  |  *swp*   |  *push*  |  *pop*   |  *add*  |  *sub*   |  *mul*  |  *div*
   001   |  *jmp*   |  *jz*    |  *jnz*   |  *jlz*   |  *not*  |  *and*   |  *or*   |  *xor*
   010   | **merg** | **splt** |    -     |    -     | **add** | **sub**  | **mul** | **div**
   011   | **jmp**  | **jz**   | **jnz**  | **jlz**  | **imm** | **immh** |    -    |    -
   100   |  *int*   |  *halt*  |  *sto*   |  *load*  |    -    |    -     |    -    |    -
   101   |     -    |    -     |  *in*    |  *out*   |    -    |    -     |    -    |    -
   110   |     -    |    -     |    -     |    -     |    -    |    -     |    -    |    -
   111   |     -    |    -     |    -     |    -     |    -    |    -     |    -    |    -

Note: `[ooo]` means the memory address pointed to by `ooo`.

### `add`

 - Format: r-class
 - Opcode: 000 100
 - Octal: 04
 - Operands:
      - `ooo`: Destination register to add to.
      - `ppp`: Second register to add.
 - Description: Adds value in register `ppp` to register `ooo`.

---

 - Format: i-class
 - Opcode: 010 100
 - Octal: 24
 - Operands:
      - `ooo`: Destination register to add to.
      - `i`: Value to add.
 - Description: Adds immediate value to register `ooo`.

### `and`

 - Format: r-class
 - Opcode: 001 101
 - Octal: 15
 - Operands:
      - `ooo`: First input and destination.
      - `ppp`: Second input.
 - Description: Performs a logical and of `ooo` and `ppp`, storing the result
   into `ooo`.

### `div`

 - Format: r-class
 - Opcode: 000 111
 - Octal: 07
 - Operands:
      - `ooo`: Dividend and destionation.
      - `ppp`: Divisor.
 - Description: Performs integer division of `ooo` by `ppp`, storing the
   quotient in `ooo` and the remainder in `ooo`'s alternative.

---

 - Format: i-class
 - Opcode: 010 111
 - Octal: 27
 - Operands:
      - `ooo`: Dividend and destination.
      - `i`: Divisor.
 - Description: Performs integer division of `ooo` by `i`, storing the quotient
   in `ooo` and the remainder in `ooo`'s alternative.

### `halt`

 - Format: r-class
 - Opcode: 100 001
 - Octal: 41
 - Operands:
	  - `ooo`: Ignored.
	  - `ppp`: Ignored.
 - Description: Halts and shuts down the machine.

### `imm`

 - Format: i-class
 - Opcode: 011 100
 - Octal: 34
 - Operands:
      - `ooo`: Destination register.
      - `i`: Value.
 - Description: Stores `i` in `ooo`, using sign extension to fill the upper
   bits.

### `immh`

 - Format: i-class
 - Opcode: 011 101
 - Octal: 35
 - Operands:
      - `ooo`: Destination register.
      - `i`: Value.
 - Description: Stores the lower 12 bits of `i` in the upper 12 bits of `ooo`.
   The lower 12 bits of `ooo` are unmodified.

### `in`

 - Format: r-class
 - Opcode: 101 010
 - Octal: 52
 - Operands:
      - `ooo`: Destination register.
      - `ppp`: Sign flag, bit count and port number.
 - Description: Reads from port specified in lower 12 bits of `ppp` into `ooo`.
   The format of `ppp` is:

       srr rrr rcc ccc ppp ppp ppp ppp

   where `s` = sign flag, `r` = reserved, `c` = bit count, and `p` = port
   number. If `s = 1`, the value read in will be sign extended into `ooo`,
   otherwise it will be `0` extended. Behavior is undefined for `c > 24`.

### `int`

 - Format: r-class
 - Opcode: 100 000
 - Octal: 40
 - Operands:
      - `ooo`: Interrupt ID.
      - `ppp`: Argument.
 - Description: Triggers a software interrupt.

### `jlz`

 - Format: r-class
 - Opcode: 001 011
 - Octal: 13
 - Operands:
      - `ooo`: Value to compare.
      - `ppp`: Location to jump to.
 - Description: Jumps to `ppp` if `ooo` is less than zero.

---

 - Format: i-class
 - Opcode: 011 011
 - Octal: 33
 - Operands:
      - `ooo`: Value to compare.
      - `i`: Location relative to `ip` to jump to.
 - Description: Jumps to `ip + i` if `ooo` is less than zero.

### `jmp`

 - Format: r-class
 - Opcode: 001 000
 - Octal: 10
 - Operands:
      - `ooo`: Ignored.
      - `ppp`: Location to jump to.
 - Description: Unconditionally jumps to `ppp`.

---

 - Format: i-class
 - Opcode: 011 000
 - Octal: 30
 - Operands:
      - `ooo`: Ignored.
      - `i`: Location relative to `ip` to jump to.
 - Description: Unconditionally jumps to `ip + i`.

### `jnz`

 - Format: r-class
 - Opcode: 001 010
 - Octal: 12
 - Operands:
      - `ooo`: Value to compare.
      - `ppp`: Location to jump to.
 - Description: Jumps to `ppp` if `ooo` is not equal to zero.

---

 - Format: i-class
 - Opcode: 011 010
 - Octal: 32
 - Operands:
      - `ooo`: Value to compare.
      - `i`: Location relative to `ip` to jump to.
 - Description: Jumps to `ip + i` if `ooo` is not equal to zero.

### `jz`

 - Format: r-class
 - Opcode: 001 001
 - Octal: 11
 - Operands:
      - `ooo`: Value to compare.
      - `ppp`: Location to jump to.
 - Description: Jumps to `ppp` if `ooo` is equal to zero.

---

 - Format: i-class
 - Opcode: 011 001
 - Octal: 31
 - Operands:
      - `ooo`: Value to compare.
      - `i`: Location relative to `ip` to jump to.
 - Description: Jumps to `ip + i` if `ooo` is equal to zero.

### `load`

 - Format: r-class
 - Opcode: 100 011
 - Octal: 43
 - Operands:
      - `ooo`: The register to load into.
      - `ppp`: The register holding the address from which to load.
 - Description: Loads value at `[ppp]` into `ooo`.

### `merg`

 - Format: i-class
 - Opcode: 010 000
 - Octal: 20
 - Operands:
      - `ooo`: Ignored
      - `i`: Ignored
 - Description: Copies `acca` into the higher half of `accc` and `accb` into the
   lower half of `accc`.

### `mul`

 - Format: r-class
 - Opcode: 000 110
 - Octal: 06
 - Operands:
      - `ooo`: Multiplicand and destination.
      - `ppp`: Multiplier.
 - Description: Multiplies `ooo` by `ppp`, storing the result in `ooo`.

---

 - Format: i-class
 - Opcode: 010 110
 - Octal: 26
 - Operands:
      - `ooo`: Multiplicand and destination.
      - `i`: Multiplier.
 - Description: Multiplies `ooo` by `i`, storing the result in `ooo`.

### `noop`

 - Format: r-class
 - Opcode: 000 000
 - Octal: 00
 - Operands: Ignored
 - Description: Does nothing for a cycle

### `not`

 - Format: r-class
 - Opcode: 001 100
 - Octal: 14
 - Operands:
      - `ooo`: The register to not.
      - `ppp`: Ignored
 - Description: Logically nots `ooo`.

### `or`

 - Format: r-class
 - Opcode: 001 110
 - Octal: 16
 - Operands:
      - `ooo`: First input and destination register.
      - `ppp`: Second input register.
 - Description: Performs a logical or of `ooo` and `ppp`, storing the result in
   `ooo`.

### `out`

 - Format: r-class
 - Opcode: 101 011
 - Octal: 53
 - Operands:
      - `ooo`: Source register.
      - `ppp`: Sign flag, bit count, and port number.
 - Description: Writes `ooo` into port specified in lower 12 bits of `ppp`.
   The format of `ppp` is:

       rrr rrr rcc ccc ppp ppp ppp ppp

   where `r` = reserved, `c` = bit count, and `p` = port number. Behavior is
   undefined for `c > 24`.

### `pop`

 - Format: r-class
 - Opcade: 000 011
 - Octal: 03
 - Operands:
      - `ooo`: The register to pop into
      - `ppp`: Ignored
 - Description: Copies the value of `[sp]` into `ooo`, then increments `sp`.

### `push`

 - Format: r-class
 - Opcode: 000 010
 - Octal: 02
 - Operands:
      - `ooo`: The register to push
      - `ppp`: Ignored
 - Description: Decrements `sp`, then copies value of `ooo` to `[sp]`.

### `splt`

 - Format: i-class
 - Opcode: 010 001
 - Octal: 21
 - Operands:
      - `ooo`: Ignored
      - `i`: Ignored
 - Description: Copies the higher half of `accc` into `acca` and the lower half
   of `accc` into `accb`.

### `sto`

 - Format: r-class
 - Opcode: 100 010
 - Octal: 42
 - Operands:
      - `ooo`: The register to store
      - `ppp`: The register holding the address at which to store.
 - Description: Stores value of `ooo` at `[ppp]`.

### `sub`

 - Format: r-class
 - Opcode: 000 101
 - Octal: 05
 - Operands:
      - `ooo`: Destination register to subtract from.
      - `ppp`: Second register to subtract.
 - Description: Subtracts `ppp` from `ooo`, stores result in `ooo`.

---

 - Format: i-class
 - Opcode: 010 101
 - Octal: 25
 - Operands:
      - `ooo`: Destination register to subtract from.
      - `i`: Value to subtract.
 - Description: Subtracts `i` from `ooo`, stores result in `ooo`.

### `swp`

 - Format: r-class
 - Opcode: 000 001
 - Octal: 01
 - Operands:
      - `ooo`: The register pair to swap, according to the `acc` register's ID
      - `ppp`: Ignored
 - Description: Exchanges the values in an `acc` and `bak` register pair.

### `xor`

 - Format: r-class
 - Opcode: 001 111
 - Octal: 17
 - Operands:
      - `ooo`: First input and destination register
      - `ppp`: Second input register
 - Description: Performs a logical exclusive or of `ooo` and `ppp`, storing the
   result in `ooo`.
