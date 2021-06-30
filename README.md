# Slime Mold Simulator

Rust + WASM implementation of a slime mold simulation (based on [Characteristics of Pattern Formation and Evolution in Approximations of Physarum Transport Networks](https://uwe-repository.worktribe.com/output/980579/characteristics-of-pattern-formation-and-evolution-in-approximations-of-physarum-transport-networks))

## TODO

[ ] - Switch `Point` implementation for a library that has vectors/coords  
[ ] - Improve rng performance (function is blocking, either spawn threads or do async? or generate a large sample pool?)  
[x] - Fix the bug of cells randomly dying  
[ ] - Implement filter intensity (trail map)  
[ ] - Link `SimulationConfig` to HTML form for easy parameter tweaking  
