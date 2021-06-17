# lightsaber

A Unix-like hobby operating system written in almost pure Rust, alongside with some inline Assembly.

## Project Name

This operating system project is currently codenamed "Lightsaber". The name may be changed in the future, but the crate names
shall not be changed anyway.

## Project Structure

This project is organized by separating code in their corresponding crates. There are currently 4 crates, as listed below:

| <div align="center">Crate</div> | <div align="center">Usage</div>                                                           |
| ------------------------------- | ----------------------------------------------------------------------------------------- |
| [`lightsaber_bootloader`](https://github.com/HT-Studios/project-lightsaber)         | The UEFI bootloader for the operating system.                                             |
| [`lightsaber_buildsystem`]        | The build system utility for easy building of the operating system.                       |
| [`lightsaber_graphics`]           | Provides various graphical APIs for the operating system.                                 |
| [`lightsaber_kernel`]             | The actual kernel of the operating system.                                                |
| [`lightsaber_util`]               | Some useful utilities for use in the entire ecosystem of crates for the operating system. |

## Building 

You can first compile the `lightsaber_buildsystem` executable. After that, you should copy the executable to somewhere else
(preferably the same working directory as the project, as the build system will first remove previously built artifacts
before performing a build, that the executable itself will also be deleted). After that, run the executable and follow the
on-screen instructions.
