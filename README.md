# Rust NUCC Binary Library
This library provides an abstract interface for CyberConnect2's custom data structures from their NUCC engine.

The following structures are currently supported (this library enforces PascalCase):
- PlayerColorParam

## Usage
This library is currently in its infancy, so no usage documentation will be created just yet, nor has it been uploaded to crates.io.

For further enquiries, come join JoJo's Bizarre Modding Community on Discord and ask us there: https://discord.jojomodding.com

## Contributing
There are many, many, MANY different data structures across CyberConnect2's videogame catalogue, and the differences of these between games has yet to have been sufficiently documented! Therefore, any additions and modifications are very much welcome, although please be consistent with the format and API of existing structures (not that it's easy to deviate in Rust anyway).

It would also be great to have developers contributing to the open-source NUCC ecosystem! Using this library in projects of your own (e.g. an NUCC to JSON converter) helps grow the available tooling for others to then use in *their* projects, and so on.

Also, although this library is programmed in Rust, I invite anyone to code something similar in other languages too to spread accessibility. Personally, I think a C ABI would be great, and a C wrapper could potentially be made for this library if that works better than writing a C library from scratch.
