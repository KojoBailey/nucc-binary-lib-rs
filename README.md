# Rust NUCC Binary Library
This library provides an abstract interface for [CyberConnect2](https://jojomodding.miraheze.org/wiki/CyberConnect2)'s custom data structures from their [NUCC](https://jojomodding.miraheze.org/wiki/NUCC) engine.

It is abstract in which it stores the data in the most logical manner, as opposed to mirroring how the structures are layed out as binary-encoded game data (which is inconsistent anyway). This way, it aims to set the foundation for which other libraries can build off of, adding support for serialisation and the like.

> [!NOTE]
> This library enforces PascalCase for struct names, so familiar titles like "messageInfo" are changed to "MessageInfo". Mind you, the casing of the original names weren't consistent to begin with anyway (e.g. `PlayerColorParam` in ASB and `playerColorParam` in EoH).

The following structures are currently supported (this library enforces PascalCase):
- PlayerColorParam
- MessageInfo

> [!WARNING]
> So far, these are just drafts, and have only been created with the JJBA CC2 games in mind. The other games may have differences that require additional changes.

## Usage
This library is currently in its infancy, so no usage documentation will be created just yet, nor has it been uploaded to crates.io.

For further enquiries, come join JoJo's Bizarre Modding Community on Discord and ask us there: https://discord.jojomodding.com

## Contributing
There are many, many, MANY different data structures across CyberConnect2's videogame catalogue, and the differences of these between games has yet to have been sufficiently documented! Therefore, any additions and modifications are very much welcome, although please be consistent with the format and API of existing structures (not that it's easy to deviate in Rust anyway).

It would also be great to have developers contributing to the open-source NUCC ecosystem! Using this library in projects of your own (e.g. an NUCC to JSON converter) helps grow the available tooling for others to then use in *their* projects, and so on.

Also, although this library is programmed in Rust, I invite anyone to code something similar in other languages too to spread accessibility. Personally, I think a C ABI would be great, and a C wrapper could potentially be made for this library if that works better than writing a C library from scratch.
