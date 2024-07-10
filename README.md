# GTASA Downloader

## Introduction

This downloader aims to make it effortless for anyone to have a portable copy of the game, along with *SA-MP*, an *ASI loader* (for your varied scripts) and *SAMPCAC* (SA-MP Client Anti-Cheat, which is discontinued, but still works fine and is better than nothing...)

## Why

1. There's not many places to download the game anymore. And when there is, you never know what you're gonna get.
2. The archive it downloads is really small (it's ~500MB), compared to the ~4.7GB, when it's decompressed. So I don't mind hosting it.
3. It's hosted on one of my VPS's, that will never go away (costs ~50USD per year). It's sole purpose is to serve files. It's limited to 1Mb/s downloads and 1 concurrent connection.
4. Piecing together all the right files/installing the correct software is always a pain.
5. The *SAMPCAC* ASI script gets flagged as being malware (it's not), so an exception needs to be added to *Windows Defender*, for it to not be removed from the game directory. (Why it needs to be ran as *Administrator*.)
6. I deemed this a simple enough exercise to get started with the Rust language.

## ASI Loader

The loader provided is [my fork](https://github.com/viruxe/sa-asi-loader) of [Carlos Menezes' *ASI Loader*](https://github.com/carlos-menezes/sa-asi-loader).

## SAMPCAC

All client-side anti-cheats were all bypassed at some point and this one wasn't the exception. But whatever, it's included, as some servers still use it, as not everyone can bypass it.

### Anti-Cheat Issues

It might not work alongside other ASI scripts. (It didn't work for me when trying to run a Window Mode ASI.)
Either remove it if you don't need it or rename the extension if you think you'll need it for later.

## Known Issues

- Since I'm using *WinGet* to install *7Zip* (to decompress the game archive) and *DirectX* (Yes, fresh *Windows* installs don't come with *DirectX* pre-installed.) you need to accept *msstore* (*Microsoft* Store - that's the source for this WinGet package) terms before downloading anything.
So if it doesn't work the first time, open the command prompt and run `winget install --id=7zip.7zip -e`, to install accept those terms and install *7Zip*.

## How it works

1. Downloads a 7Zip archive ("gtasa.7z") to the same directory as the executable
2. Asks you for a directory name (press enter for the default to be used - "GTA San Andreas") to where it will decompress the game archive, equally, to the same directory as the executable.
3. Decompresses the archive onto the specified directory.
4. Adds an exception/exclusion to *Windows Defender*, in order for *SAMPCAC*'s ASI script not to be removed.
5. Downloads "sampcac_client.asi"

The game archive will remain so it can be reused the next time you want a fresh installation or if you want to store it for yourself.
