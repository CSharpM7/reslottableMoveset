# reslottableMoveset

This is probably pretty messy as I only quickly made this for 2 mods. Decided to release this source code upon my hiatus. Anyways, brace yourselves for when the users who wanted a moveset to be reslottable complain that they can't reslot it.
## Usage

Rename the smashline_sonicoc file to whatever, and also update the `IDENTIFIER` const in `data.rs` to be the same name as that file. Place that file in the root of your mod's folder (ie `SonicOC/smashline_sonicoc`). If a user decides to remove that file, this plugin will send them a message and boot them out of smash.
Change the file `/fighter/sonic/motion/body` filepath in `singleslot.rs` to whatever would be relevant for your mod. The plugin will automatically assign the modded slots to whatever is in the motion folder.
