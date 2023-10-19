# Open Spyro

![Spyro the Dragon screenshot from Town Square](http://henrijahanna.fi/projects/open_spyro/spyro_town_square.bmp)

## Introduction

Welcome to the Spyro the Dragon Linux and Windows port project called OpenSpyro! OpenSpyro is an open-source initiative driven by a passionate programmer and fan of the original Spyro the Dragon trilogy on Playstation. The project is focused on reverse-engineering and adapting the game to run natively on Linux and Windows (I don't have a Mac, sorry). I aim to provide an authentic experience that stays true to the original while leveraging the capabilities of modern hardware.

This project will not contain any game data in its original form. You must have an original ROM of the game available in order to play this port.

## Goals

* Make the game run natively, no emulation needed to run the game on Linux or Windows.
* Maintaining the original experience as is while leveraging the capabilities of modern hardware. The focus is on rewriting only the necessary logic to get the game running on Linux and Windows, although some quality of life improvements may be made.
* Support for mods in some way.

## Current state

The project is currently in its early stages. Port is currently unplayable. Some of the MIPS assembly code has been disassembled with comments and labels added. Disassembly is in a completely custom assembly, the repository even has its own assembler and disassembler! This custom assembler and disassembler ought to have more documentation, but it is still in the works.

The project is currently focused on reverse-engineering the MIPS assembly and understanding how the data is layed out on the ROM. I am still learning MIPS assembly works, how assembly works in general (now having trouble understanding how to decode/encode j instructions and how stack works for storing local variables). Some tools have been built along the way, like a working ROM file replacer, which is used for saving Playstation executable files back into the ROM file. Some work has been started on reading WAD files, but it is in its early stages.

While learning MIPS assembly and thinking about decompilation, I decided to start working on a custom programming language for aiding reverse engineering seriously. I could use C, but I was having trouble linking stuff and I didn't want to bother with linker scripts and stuff. I have already built tokenization, basic parsing for functions, variable initialization and support for some built-in return types and values. Assembly to MIPS and compilation to MIPS processor compatible machine code still WIP. This custom compiler will be added to this repository as a separate crate once there's enough significant progress made. `cmips` crate is a separate attempt, it will be replaced eventually.

ROM currently worked on is an NTSC version. I have not worked on any other versions yet.

## Commands

Here's a list CLI commands currently supported:

* `generate-doc` Generates README.md file describing the project at project root.
* `mips-assemble` Converts MIPS assembly instruction into machine code (as hexadecimal) and into LE bytes also.
* `mips-disassemble` Converts machine code into an MIPS assembly instruction string.
* `ps1exe-assemble` Assembles MIPS assembly code from a given text file into a Playstation executable.
* `ps1exe-disassemble` Disassembles a section of MIPS assembly code from a given Playstation executable binary.
* `rom-check` Checks the given ROM file structure for correctness.
* `rom-extract` Extracts a file from a ROM to a given extract path.
* `rom-replace` Replaces a file in a given ROM with a given input file.
* `wad-read` Reads information about WAD file. Heavily WIP.

## Disclaimer

OpenSpyro is an independent project and is not affiliated with the original creators or owners of Spyro the Dragon. It is a fan-driven initiative for educational and entertainment purposes. I do not claim ownership of the original game's assets or intellectual property.

I am still learning about reverse engineering, how the Playstation works and how porting with all its intricacies when its comes to like audio and graphics should be handled. I am not an expert. Suggestions and help is welcome and appreciated.