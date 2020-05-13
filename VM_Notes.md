The furry-parakeet Virtual Machine
==================================

Or at least this particular implementation, which is likely to be the only
implementation.

Specifications
--------------

### Memory

- Default: 4,096 (2^12) 24-bit words
- Maximum: 16,777,216 (2^24) 24-bit words

Startup
-------

System loads ROM into Memory. Default ROM is hardcoded into VM. Alternative ROMs
can be specified on the command line. `IP` is set to beginning of ROM in Memory.
