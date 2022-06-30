++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
+
+					R E A D M E  |  M O R P H  I T !!!
+
++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++

++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
+++ ORDNER STRUKTUR

|-- Game
	|-- morph_it.exe					// Start der Anwendung
	|-- game.ini						// Konfigurationsdatei für Physik, ...
	|-- editor.html						// Level-Editor
	|-- README.txt						// Generelle Informationen zur Entwicklung
	|-- assets/							// Resourcen
		|-- builder/					// Resourcen für Level-Editor
		|	|-- ...
		|	
		|-- game/						// Grundlegende Spieldateien/Grafiken
		|	|-- font/					// Resourcen für Schrift
		|	|	|-- font.ttf			// Font-Datei
		|	|
		|	|-- gui/					// Resourcen für Menu
		|	|	|-- [textures]			// Einzelne Menugrafiken
		|	|	|-- ...
		|	|	
		|	|-- core/					// Unveränderte Resourcen für Level
		|		|-- [textures]			// Einzelne Spielgrafiken (morph, target, ...)
		|		|-- ...
		|
		|-- level/						// Dynamische Package Dateien
			|-- level.ini				// Konfigurationsdatei für verfügbare Packages
			|-- [package]/				// Order für ein Package
			|	|-- info.json			// PackageInfo mit Infos über Level, Texturen, (mit Editor gebaut)
			|	|-- [textures]			// Package texturen
			|
			|-- ...						// Order für weitere Packages
		
++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
+++ FUNKTIONSWEISE

- Beim Spielstart werden die "game.ini" und die "level.ini" Dateien ausgelesen (Änderungen erst nach Neustart)
- Ist ein Package in der level.ini aufgeführt, wird automatisch nach der info.json im gleichnamigen Order gesucht.
- Die info.json wird im Editor gebaut und beinhaltet alle nötigen Package informationen:
	- Alle Level, Morph Position, Morph Verwandlungen, Ziel, Objekte, Kollisionen, später Animationen uvw.
	- Alle Texturen die das Package verwendet
	- Texturen können nicht für andere Packages benutzt werden, sondern müssen extra in deren Ordner gelegt werden
	- Texturen die in allen Packages genutzt werden, sollen später hardcodiert in den game Ordner verlegt werden


++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
+++ BEKANNTE BUGS

- aktuell nichts bekannt ;)	
