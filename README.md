# xorhelper

A simple Rust tool to obfuscate or deobfuscate data with a repeating XOR key.

Usage:

```sh
$ echo 'hello, world' | xorhelper 'my super secret password' > obfuscated.txt
```

The data to be translated is read from STDIN. The translated data is returned to STDOUT.
