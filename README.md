# Morph (Rust Game Development)
I started a game developed completely in Rust with OpenGL. It's a clone of SuperMorph, a game from SNES times. 
But work, family and other stuff makes it hard to get things done. I want to share this to everyone who is interested.

# Feel free
Feel free to add stuff or make commits.

# Installation
It's based on my own OpenGl/Winit/Glutin fix, that works on android, so this game also works on android, producing working APKs if you follow installation as described here:
https://github.com/Kaiser1989/game-gl

# Editor
I've added an Level editor (check the html). Sry for not having time to make a better documentation. But if you have any questions, feel free to ask. I will answer whenever i have time to do.

# Basic Functioning
* When starting the game, `assets/game.ini` and `assets/level/level.ini` are read (Changes after restart)
* If there is a package in `level.ini`, it will automatically search for a `info.json` within same named folder
* This `info.json` is built with given editor and contains all needed package information:
** All levels, morph positions, morph counts, targets, objects, collisions, animations (later)
** All textures used in this package
** Texture cannot be used for other packes (you need to duplicate)
