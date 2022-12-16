# Dipole Simulation
Started with [this project](https://github.com/chinedufn/webgl-water-tutorial) as a foundation.

This project was developed for the simulation of dipole moments in a way which is accessible on the web. I opted to use web assembly with Rust for this project as the performance benifit for the numerical calculations necessary for the simulation would be noticable. I used raw WebGL for the graphical display aspect of the project, as there was no complex graphics necessary and it gave fine grained control with high performance. For the simulation I wrote a 4th order Runge Kutta algorithm for linear and rotational mechanics. The input forces for the Runge Kutta were calculated using Coulomb's law across discrete charges. The dipoles were modeled as a physical dipole with two opposite charges seperated by an offset.

As this simulation does not account for any replusive or normal forces between objects the dipoles eventually converge which results in the simulation failing. This can be tracked via the momentum and energy readouts, which report the current state of the simulation. In cases were more accuracy could correct the divergence, it would be possible to address the issue by using different methods of solving the differential equations such as Verlet integration which is energy conserving (as far as the mean is concerned), [csRKN](https://arxiv.org/pdf/1808.08451.pdf), or simply adaptive Runga Kutta for more accuracy in edge cases, but the reality is that at some point all methods will fail as the forces become infinite without some model of normal or repulsive forces.

The momentum and energy read outs are absolute, the timescale and offse sliders work while the simulation is running, and the reset button only resets the dipole position, orientation, and related velocities.
![image](https://user-images.githubusercontent.com/35360746/208027997-0c61bb40-ca53-4157-b41f-013a268d8534.png)
# Instructions for compiling and running
```sh
# You can use any static file server that properly sets the
# `application/wasm` mime type
cargo install https

git clone https://github.com/chinedufn/dipole-simulation
cd dipole-simulation

# A version of Rust that can compile wasm-bindgen-cli version 0.2.29
cargo install -f wasm-bindgen-cli --version 0.2.29 # Or download a release binary

# Build
./build.sh

## Opens your browser to http://localhost:8000  where the demo will be running
http -m wasm:application/wasm
```
