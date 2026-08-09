# Step 02 - TCP and Waypoints

## Objective
Define a correct tool center point at the vacuum tip and teach reliable waypoints.

## URSim TCP Definition (PolyScope steps)

### What is a TCP?
The Tool Center Point is a virtual point you declare at the tip of your end-effector.
All robot moves are calculated relative to this point.
For a vacuum gripper, set TCP at the center of the suction cup face.

### Steps in PolyScope
1. Go to **Installation → TCP Configuration**.
2. Add a new TCP named `vacuum_tip`.
3. Set offsets — for a simple vacuum cup ~80 mm below flange:
   - X: 0, Y: 0, Z: 0.08 m, Rx: 0, Ry: 0, Rz: 0
4. Set `vacuum_tip` as the **Active TCP**.
5. Verify by jogging the robot — the TCP marker should appear at the suction cup center.

### Waypoint Teaching (PolyScope)
1. Open a **new Program** (Program tab).
2. Use the **MoveJ** block to jog the robot manually with **Freedom Drive** or arrow keys.
3. Save each joint configuration as a named waypoint:
   - `home` — safe all-joints-zero position
   - `pick_approach` — 100 mm above the box center
   - `pick_contact` — TCP touching box top
   - `pick_lift` — 100 mm above box again after pick
   - `placeA_approach` — 100 mm above Bin A
   - `placeA_drop` — inside Bin A
   - `placeB_approach` — 100 mm above Bin B
   - `placeB_drop` — inside Bin B

## Validation
- TCP marker in 3D view sits at gripper tip when you move the robot.
- Moving to `pick_contact` aligns TCP to the imaginary box position.
- All 8 waypoints saved and visible in program tree.

## Learning Notes
Capture why each waypoint exists (approach vs contact vs retreat).
Approach waypoints exist to avoid striking objects on the way down — always approach from above.
