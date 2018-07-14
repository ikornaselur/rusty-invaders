# Rusty Invaders

A test project to learn more about writing emulators and to learn Rust at the same time. This project is the result of following [emulator101](http://emulator101.com/).


## Integration test
The integration test runs a cpudiag binary as described in the [Full 8080 emulation](http://www.emulator101.com/full-8080-emulation.html) from emulator101. The binary checked into this repo has been modified in the following way in a hex editor:
* Prepends the file with `C3 00 01`, followed by 253 `00` bytes to start the binary at `0x0100` instead of `0x0000`. The first three bytes immediately jump to `0x0100`.
* Patch out the DAA test by replacing three bytes starting at offset 0x059C with `C3 C2 05`

### Resources
* [Instruction Set](http://pastraiser.com/cpu/i8080/i8080_opcodes.html)
* [emulator101](http://emulator101.com/)
