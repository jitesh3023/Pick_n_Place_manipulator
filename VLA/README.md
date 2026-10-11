# Isaac Sim robotics workflows

Three separate workflows are planned:
1. MoveIt 2 + Isaac Sim
2. cuRobo + Isaac Sim
3. VLA + Isaac Sim

This guide records the setup: a simulated UR10 publishing joint
feedback to ROS 2 and receiving joint-position commands from a small ROS node.

## 1. Files

| File | Purpose |
| --- | --- |
| (scripts/start-isaac.sh) | Start Isaac with the Jazzy ROS bridge |
| (scenes/ur10_follow_target.usd) | Original saved UR10 and target-cube scene |
| (scripts/two_targets.py) | Hard coded to location for the robot end effector to go to in sequence |
| (scenes/ur10_robot_ros_enabled.usd) | Saved ROS-enabled scene |
| (scripts/joint_teleop.py) | Minimal external ROS joint-command node |

## 2. Launch Isaac Sim — terminal 1

From the project root:

```bash
cd "/media/jitesh/Extreme SSD/Projects/Pick_n_Place_manipulator"
bash VLA/scripts/start-isaac.sh
```

The launcher currently uses the machine-specific Isaac installation at
`/media/jitesh/Extreme SSD/isaac/isaacsim`. Change the launcher's installation
directory if Isaac is moved.

This script does the following:
1. Deactivates Conda in the launcher's child shell, if active. The parent terminal
	 can still display `(base)` afterward.
2. Clears inherited CUDA/Python library overrides and ROS workspace prefixes.
3. Selects `ROS_DISTRO=jazzy` and `RMW_IMPLEMENTATION=rmw_fastrtps_cpp`.
4. Sets `LD_LIBRARY_PATH` to Isaac's bundled Jazzy bridge libraries.
5. Launches Isaac with `--enable isaacsim.ros2.bridge`.

Successful startup was verified in the Isaac log with:

```text
Attempting to load internal rclpy for ROS Distro: jazzy
rclpy loaded
Isaac Sim Full App is loaded.
```

## 3. Open the robot scene

In Isaac, use **File → Open** and select the saved
[ROS-enabled scene](scenes/ur10_robot_ros_enabled.usd).
Keep the simulation **stopped** while inspecting or editing the graph.

For normal use, reuse that scene: do not recreate its graph every session.
The setup instructions in section 5 explain how I built it from the
[original scene](scenes/ur10_follow_target.usd).

Verified prim paths in this robot asset:

| Prim | Path |
| --- | --- |
| Robot parent | `/World/ur10_robot` |
| Articulation root | `/World/ur10_robot/root_joint` |
| End-effector link | `/World/ur10_robot/ee_link` |
| Target cube | `/World/TargetCube` |

### Earlier experiment: follow the target cube

Before ROS control, I loaded the **UR10 Follow Target** robotics example,
tested cube following, and saved the original scene. The saved-scene controller
is [two_targets.py](scripts/two_targets.py).

To use that experiment separately:
1. Open the original follow-target scene.
2. Open Isaac's **Script Editor**, load the script, and run it there—not with
	 Ubuntu's system Python.
3. With `MANUAL_MODE = True`, move the target cube using the Move tool.
4. With `MANUAL_MODE = False`, the script commands the A/B sequence.

Current automatic settings are A = `(0.40, 0.20, 0.30)` metres,
B = `(0.40, 0.10, 0.35)` metres, `HOLD_SECONDS = 1.0`, and `CYCLES = 2`.
The duration is time spent commanding each target. Tool orientation remains fixed. Pause/Stop ends the run;
run the script again to restart it.

A USD scene does not save live Python callbacks. The script must be run again
after reopening the original scene. **Do not run this IK controller alongside
the ROS articulation controller**, because they can command the same joints.

## 4. Prepare external ROS — terminal 2

Open another terminal. If Conda is active, deactivate it first:

```bash
conda deactivate
```

Skip that command if Conda is already inactive. Then:

```bash
source /opt/ros/jazzy/setup.bash
```

The distribution should print `jazzy`. Repeat sourcing in each new ROS terminal.

Isaac and the external ROS processes communicate through DDS; they do not need
to share a Python environment. Both must use the same ROS domain. Our graph
used domain **0**.

## 5. How I created the ROS Action Graph

These steps are for reconstructing the graph from the original scene only.

### A. Add joint-state feedback

With simulation stopped, open
**Tools → Robotics → ROS 2 OmniGraphs → Joint States**.

| Field | Value |
| --- | --- |
| Add to an existing graph? | Unchecked |
| Graph Path | `/Graph/ROS_JointStates` |
| Node Namespace | Empty |
| Articulation Root | `/World/ur10_robot/root_joint` |
| Publisher | Checked |
| Publisher Topic | `/joint_states` |
| Subscriber | Unchecked |
| Move Robot? | Unchecked |

Click **OK**. This creates four nodes: On Playback Tick, ROS2 Context,
Isaac Read Simulation Time, and ROS2 Publish Joint State.

### B. Add incoming joint commands

Open the same Joint States shortcut again:

| Field | Value |
| --- | --- |
| Add to an existing graph? | Checked |
| Graph Path | `/Graph/ROS_JointStates` |
| Node Namespace | Empty |
| Articulation Root | `/World/ur10_robot/root_joint` |
| Publisher | Unchecked—do not add a duplicate |
| Subscriber | Checked |
| Subscriber Topic | `/joint_command` |
| Move Robot? | Checked |

Click **OK**. The existing publisher remains, and two nodes are added:
**ROS2 Subscribe Joint State** and **Articulation Controller**.

The controller's **Target Prim** should be `/World/ur10_robot/root_joint`.
Leave **Robot Path** empty when using Target Prim.

### C. View the six-node graph

In the UI used during this setup, open
**Window → Graph Editors → Action Graph**, then **Edit** and choose
`/Graph/ROS_JointStates`. Some layouts label the menu **Visual Scripting**.
Do not choose New Graph to inspect an existing graph.

| Connection | Role |
| --- | --- |
| Playback Tick → publisher Exec In | Trigger feedback publication |
| Playback Tick → subscriber Exec In | Process incoming commands |
| Playback Tick → controller Exec In | Apply joint commands |
| ROS2 Context → publisher and subscriber Context | ROS communication context |
| Simulation Time → publisher Timestamp | Timestamp the measured state |
| Subscriber Joint Names → controller Joint Names | Select commanded joints |
| Subscriber Position/Velocity/Effort Command → matching controller inputs | Pass received targets |

Target Prim and topic names are configured values, so they do not need wires.
The minimal teleop node supplies only positions; velocity and effort arrays
are left empty.

After testing, stop simulation and use **File → Save As** to save the
ROS-enabled scene separately from the original follow-target scene. Save later
graph changes too.

## 6. Verify Isaac → ROS feedback

Press **Play** in Isaac. In the sourced ROS terminal:

```bash
ros2 topic list
ros2 topic echo /joint_states
```

We successfully received messages containing all six joint names, positions,
velocities, efforts, and a simulation timestamp. Seeing a topic in the list
alone is not proof that messages are arriving; the echo verifies data transfer.

If echo waits without output, stop it with Ctrl+C and inspect:

```bash
ros2 topic info /joint_states --verbose
```

Check Play state, publisher Target Prim, graph connections, and matching ROS
domains. The material-panel `Expected UsdShade prim` warnings seen during setup
did not block the verified messages. Warnings about missing graph attributes
should not be assumed harmless if publication fails.

## 7. Run the minimal joint teleop node

Keep Isaac playing and the target-following script inactive. In the sourced
ROS terminal, from the project root:

```bash
/usr/bin/python3 VLA/scripts/joint_teleop.py
```

This script runs **outside Isaac**, not in Isaac's Script Editor. It publishes
`sensor_msgs/msg/JointState` messages to `/joint_command`.

Enter **joint number followed by an absolute target angle in degrees**, then
press Enter:

```text
6 10
6 0
q
```

The first input commands wrist 3 to 10 degrees; the second commands it back to
zero; `q` exits. Test small movements near the current pose.

| Number | Joint |
| --- | --- |
| 1 | `shoulder_pan_joint` |
| 2 | `shoulder_lift_joint` |
| 3 | `elbow_joint` |
| 4 | `wrist_1_joint` |
| 5 | `wrist_2_joint` |
| 6 | `wrist_3_joint` |

The node converts degrees to radians, publishes only the selected joint, and
prints the target. It checks basic input validity and whether a subscriber is
discovered. A discovered subscriber does **not** guarantee simulation is playing.
There is no feedback subscription or arrival check in this minimal version.

In another sourced ROS terminal, check the resulting measured position:

```bash
ros2 topic echo /joint_states --once
```

## 8. Command versus feedback

| Topic | Direction | Meaning |
| --- | --- | --- |
| `/joint_command` | Teleop node → Isaac | Desired position; one message per accepted input |
| `/joint_states` | Isaac → ROS | Measured state of all six joints; published while playing |

Both use `sensor_msgs/msg/JointState`, but their contents need not match.
For example, a command requests wrist 3 at 10 degrees while feedback reports
its changing angle during motion. After settling, the measured angle should
approximately match the requested angle. ROS messages use radians.

## 9. Current milestone and next step

Completed:
- Isaac startup with bundled Jazzy bridge verified.
- Native Jazzy talker/listener communication verified.
- UR10 joint feedback received externally.
- External ROS joint commands successfully moved the simulated UR10.
- The six-node graph was inspected visually and saved in a separate scene.

Next: MoveIt 2 with a matching UR10 description/configuration and a trajectory
execution interface. A `/joint_command` topic alone is not a MoveIt
`FollowJointTrajectory` action server.