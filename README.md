# Slime Mold Simulator

Rust + WASM implementation of a slime mold simulation (based on [Characteristics of Pattern Formation and Evolution in Approximations of Physarum Transport Networks](https://uwe-repository.worktribe.com/output/980579/characteristics-of-pattern-formation-and-evolution-in-approximations-of-physarum-transport-networks))

## TODO


[ ] - (big perf issue) Make sure that `remaining` (in `motor()` step) is always allocated, memcpy is taking up lots of time
[ ] - - Use a HashMap to store the cells, (key = coordinate, no need to re-arrange vector and all that stuff, + random sampling from rand crate)
[x] - Improve rng performance (function is blocking, either spawn threads or do async? or generate a large sample pool?)  
[x] - Fix the bug of cells randomly dying  
[ ] - Implement filter intensity (trail map)
[ ] - Implement multilayer
[ ] - - Linked unidirectional
[ ] - - Linked bidirectional (with wrap-around?)
[ ] - - Dynamic linked unidirectional (push/pop layers/cells based on chemoattractant values at the bottom of the stack)
[ ] - Link `SimulationConfig` to HTML form for easy parameter tweaking  
[ ] - Switch `Point` implementation for a library that has vectors/coords  