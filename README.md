This is a WIP asteroids clone. I made it to learn Rust (which btw is fucking great).
The code sucks, and is a pain to work with.
I kind of just started writing without ever having written a real game before.

Notes for myself for a next possible game:
* Make a default struct for all game objects. This could implement a trait to draw itself on the screen etc, and also have enums that define certain behavior like screen wrapping.

* Draw all meshes with their origin in the middle, saves offset problems and can make hitboxes easier to implement.

* Make an hit detection library. It shouldn't take much math skills to make a good hitbox detection system that works with just boxes and circles. Simple take the angle between the center points of the two meshes, calculate the length between the center and the edge of each mesh and see if they when added together are bigger than the total length between the two points.

* Use ECS and stuff because borrowing yourself as a mutable twice is a bitch

* Jubliee: avoid self-referential structs, avoid limitless recursion (including of self-reference), do exploit https://doc.rust-lang.org/nomicon/borrow-splitting.html where plausible.

