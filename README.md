# RUST-FlappyBird
1st level of the 20 games challenge

flappy bird in RUST with Bevy engine 0.9

- Event System for scoring and gameover logic
- States System with system_set condition (Menu/Playing/Died)
- Audio system (flap/hit/point)
- Parallax (customized reusable system for background and ground)
- simple aabb colision
- simple input system

main difficulties:
- Text GUI
- mut query collision (solved with paramset or explicit without condition)
