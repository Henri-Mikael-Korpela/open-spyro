# Open Spyro

![Spyro the Dragon screenshot from Town Square](http://henrijahanna.fi/projects/open_spyro/spyro_town_square.bmp)

## Introduction

Welcome to the Spyro the Dragon PC port project called OpenSpyro! OpenSpyro is an open-source initiative driven by a passionate programmer and fan of the original Spyro the Dragon trilogy on Playstation. The project is focused on reverse-engineering and adapting the game to run natively on Linux and Windows. I aim to provide an authentic experience that stays true to the original while leveraging the capabilities of modern hardware.

This project will not contain any game data in its original form, you must have an original ROM of the game available in order to play this port.

## Goals

* No emulation required to run the game on Linux and Windows
* Maintaining the original experience while leveraging the capabilities of modern hardware

## Current state

The project is currently in its early stages. Port is currently unplayable. Some of the MIPS assembly code has been disassembled with comments and labels added. Disassembly is into a completely custom assembly, this repository even has its own assembler and disassembler! This custom assembler and disassembler ought to have more documentation, but it is still in the works.

The project is currently focused on reverse-engineering the MIPS assembly and understanding how the data is layed out on the ROM. Some tools have been built along the way, like a working ROM file replacer, which is used for saving Playstation executable files back into the ROM file. Some work has been started on reading WAD files, but it is in its early stages.

ROM currently supported is an NTSC version. I have not worked on any other versions yet.

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