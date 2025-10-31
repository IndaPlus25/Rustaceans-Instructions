# isaklar-assembly

Malte Blomqvist (maltebl) and I have collaborated on creating the FOR ME 👉👈 assembly language (see specification.md).

My implementation does not support macro declarations or comments. Nor does it have any kind of error-handling so if there is a slightest error in the code, either the compiler or the emulator will completely crash and not tell you why.

To complie your code, just run the rust-compiler crate with the .forme file as an argument. Then run the outputted .emojiexe file as an argument in the rust-emulator crate.

