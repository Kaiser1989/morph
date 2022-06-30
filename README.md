# Morph (Rust Game Development)
I started a game developed completely in Rust with OpenGL. It's a clone of SuperMorph, a game from SNES times. 
But work, family and other stuff makes it hard to get things done. I want to share this to everyone who is interested.

# Feel free
Feel free to add stuff or make commits.

# Installation
If you are on Windows, you can instantly start playing:
`cargo run --release`

This project is also working on android, after fixing bugs in winit & glutin. Check my game engine for more information:
https://github.com/Kaiser1989/game-gl

I tried to push my fixes on both projects, but i failed, as both changes are bare workarounds, breaking with existing conventions,
and need to be added to both projects at the same time. I can live with this fix, as i do not want to publish any crates.

Currently their seems to be a bug, that my clicks are not handled in android emulator. Shouldn't be a big thing.

# Editor
I've added an Level editor (check the html). Sry for not having time to make a better documentation. But if you have any questions, feel free to ask. I will answer whenever i have time to do.

# Basic Functioning
I've never had the time to design some good levels. You will find a test package "jungle" with 6 test levels, where some basic game objects are shown.
It took same time to design some levels, follow this instructions:

* When starting the game, `assets/game.ini` and `assets/level/level.ini` are read (Changes after restart)
* If there is a package in `level.ini`, it will automatically search for a `info.json` within same named folder
* This `info.json` is built with given editor and contains all needed package information:
  * All levels, morph positions, morph counts, targets, objects, collisions, animations (later)
  * All textures used in this package
  * Texture cannot be used for other packes (you need to duplicate)
