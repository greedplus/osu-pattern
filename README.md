# osu-pattern

An osu pattern detector for osu!standard and osu!taiko. Meant to be used by other rust projects.

## TODO

1. [ ] Get map from file
2. [ ] Break map into patterns
    - [ ] Intervals
    - [ ] Hit type
    - [ ] Pattern list
3. [ ] Analyze patterns
    - [ ] osu!standard
        - [ ] Jumps (1/2 or above, includes slider)
        - [ ] Bursts (1-7 notes 1/4)
        - [ ] Streams
        - [ ] Slider + Burst
        - [ ] Slider + Stream
        - [ ] Complex (Anything not listed above)
    - [ ] osu!taiko
        - [ ] Non-Patterns (1/2 or above)
        - [ ] Patterns (1-15 notes 1/3 or 1/4)
        - [ ] Streams
        - [ ] Sixths
        - [ ] Eighths
        - [ ] Sixths + Stream
        - [ ] Eighths + Stream
        - [ ] Sixths + Eighths + Stream
        - [ ] Complex (Anything not listed above)