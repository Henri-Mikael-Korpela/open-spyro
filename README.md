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

The project is currently focused on reverse-engineering the MIPS assembly and understanding how the data is layed out on the ROM. Some tools have been built along the way, like a working ROM file replacer, which is used for saving Playstation executable files back into the ROM file. Some work has been started on reading WAD files, but it is in its early stages.

ROM currently worked on is an NTSC version. I have not worked on any other versions yet.

## Commands

Here's a list CLI commands currently supported:

* `generate-doc`
* `mips-assemble`
* `mips-disassemble`
* `ps1exe-assemble`
* `ps1exe-disassemble`
* `rom-check`
* `rom-extract`
* `rom-replace`
* `wad-read`

## Disclaimer

OpenSpyro is an independent project and is not affiliated with the original creators or owners of Spyro the Dragon. It is a fan-driven initiative for educational and entertainment purposes. I do not claim ownership of the original game's assets or intellectual property.

I am still learning about reverse engineering, how the Playstation works and how porting with all its intricacies when its comes to like audio and graphics should be handled. I am not an expert. Suggestions and help is welcome and appreciated.