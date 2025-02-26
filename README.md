🚧 Work in progress 🚧

# VeloSim
Peloton dynamics simulator.

## Physics
### Power at the crank to forward force
Power at the crank is equal to propulsive force at the rear wheel times drivetrain losses. For now we will assume there are no drivetrain losses, so power at the crank is equal to power at the rear wheel. This means that force in the forward direction is equal to power / velocity.

### Drag
Fd = 1/2 * CdA * density of the medium * v^2
