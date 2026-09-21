"""Open the saved UR10 USD, then run this file in Isaac Sim's Script Editor.

Attaches Isaac's UR10 IK helper to the existing robot; no example UI is needed.
Resets the arm, then follows a manually moved cube or runs a timed A/B sequence.
No collision planning, grasping, or scene saving is performed.
"""

import asyncio  # asyncio:- Schedule the experiment without blocking Isac sim. Let our script wait
                # without freezing Isaac Sim's GUI.

import numpy as np
import omni.kit.app     # Communicate with the Issac Sim GUI and its update loop. It talks 
                        # to the GUI and lets Issac Sim complete its physics and rendering updates while our script waits for the next update.
import omni.timeline    # This module gives our script access to the simulation time controls like play, pause, and the current time.
import omni.usd         # To open the usd and use it in the script
from isaacsim.core.api.simulation_context import SimulationContext                     # Manages the simulation setup and connect our code to physics
from isaacsim.core.experimental.prims import XformPrim                                 # Reads and writes the position and orientation of the cube in the scene
from isaacsim.core.simulation_manager import SimulationManager                         # Initializes the physics engine and manages the simulation loop
from isaacsim.robot.manipulators.examples.universal_robots import UR10Experimental     # UR10e robot helper, including its IK. It lets use access the existing UR10 in Issac Sim,
                                                                                       # read its joint positions, end-effector pose. Resets joints and calculate joint targets for a desired end-effector pose using IK solver.


# True: drag the cube yourself. False: run the automatic A/B sequence.
MANUAL_MODE = True

# World-frame positions in metres; used only for the automatic sequence.
TARGETS = ((0.40, 0.20, 0.30), (0.40, 0.10, 0.35))  # A, B, These are cube coordinates in world frame.
HOLD_SECONDS = 1.0  # time between position A end and start of position B
CYCLES = 2
TARGET_PATH = "/World/TargetCube"
ROBOT_PATH = "/World/ur10_robot"
CALLBACK_NAME = "project_two_targets_ik"


async def run_two_targets(previous_task=None):
    # Finish the previous run's cleanup before creating another controller.
    if previous_task is not None:
        previous_task.cancel()
        try:
            await previous_task
        except asyncio.CancelledError:
            pass
        except Exception:
            pass  # The previous run already reported its error.

    context = omni.usd.get_context()
    stage = context.get_stage()
    timeline = omni.timeline.get_timeline_interface()
    app = omni.kit.app.get_app()
    manual_mode = MANUAL_MODE

    if stage is None or not all(
        stage.GetPrimAtPath(path).IsValid()
        for path in (TARGET_PATH, ROBOT_PATH, f"{ROBOT_PATH}/ee_link")
    ):
        print("Open your saved UR10 USD containing /World/ur10_robot and /World/TargetCube.")
        return

    simulation = SimulationContext.instance()
    if simulation is not None and simulation.physics_callback_exists("follow_step"):
        print("Click STOP beside the example's Follow Target before running this script.")
        return

    callback_added = False
    try:
        print("Initializing physics and IK on the open USD (no scene reload).")
        if simulation is None:
            simulation = SimulationContext(
                physics_dt=1.0 / 60.0,
                rendering_dt=1.0 / 60.0,
                stage_units_in_meters=1.0,
            )
        await simulation.initialize_simulation_context_async()
        await simulation.reset_async()
        SimulationManager.initialize_physics()
        await simulation.pause_async()
        if context.get_stage() != stage:
            print("Initialization cancelled: the open stage changed.")
            return

        # Wrap existing prims instead of creating a new robot or stage.
        robot = UR10Experimental(robot_path=ROBOT_PATH, create_robot=False, attach_gripper=False)
        target = XformPrim(TARGET_PATH)
        robot.reset_to_default_pose()
        goal = np.asarray(TARGETS[0], dtype=np.float32)
        orientation = np.array([[0.0, 1.0, 0.0, 0.0]], dtype=np.float32)
        physics_elapsed = 0.0
        control_error = None

        def follow_target(step_size):
            nonlocal physics_elapsed, control_error
            if control_error is not None or context.get_stage() != stage or not timeline.is_playing():
                return
            try:
                if manual_mode:
                    # Read the marker every physics step; do not overwrite mouse edits.
                    positions, _ = target.get_world_poses()
                    current_goal = positions.numpy().reshape(-1, 3)[0]
                else:
                    current_goal = goal
                robot.set_end_effector_pose(
                    position=current_goal,
                    orientation=orientation,
                    ik_method="damped-least-squares",
                )
                physics_elapsed += step_size
            except Exception as error:
                control_error = error

        if not manual_mode:
            target.set_world_poses(positions=[goal])
        simulation.add_physics_callback(CALLBACK_NAME, follow_target)
        callback_added = True
        await simulation.play_async()
        if manual_mode:
            print("Manual IK running: select /World/TargetCube and drag a Move-tool arrow.")
            print("Position following only; tool orientation stays fixed. Pause to end; Run again to restart.")
            previous_time = timeline.get_current_time()
            while context.get_stage() == stage and timeline.is_playing():
                await app.next_update_async()
                if context.get_stage() != stage or not timeline.is_playing():
                    break
                if control_error is not None:
                    raise RuntimeError(f"IK controller failed: {control_error}") from control_error
                current_time = timeline.get_current_time()
                if current_time < previous_time:
                    print("Manual following ended: simulation time was reset or looped.")
                    return
                previous_time = current_time
            print("Manual following ended: stage changed or simulation paused/stopped.")
            return

        print("IK running. Pause the simulation to end the sequence.")
        for cycle in range(CYCLES):
            for label, position in zip(("A", "B"), TARGETS):
                if context.get_stage() != stage or not timeline.is_playing():
                    print("Sequence stopped: stage changed or simulation paused/stopped.")
                    return
                goal = np.asarray(position, dtype=np.float32)
                target.set_world_poses(positions=[position])
                print(f"Cycle {cycle + 1}/{CYCLES}: target {label} = {position} m")
                start_time = physics_elapsed
                previous_time = timeline.get_current_time()
                while physics_elapsed - start_time < HOLD_SECONDS:
                    # Yield to the GUI and physics; never block with time.sleep().
                    await app.next_update_async()
                    if context.get_stage() != stage or not timeline.is_playing():
                        print("Sequence stopped: stage changed or simulation paused/stopped.")
                        return
                    if control_error is not None:
                        raise RuntimeError(f"IK controller failed: {control_error}") from control_error
                    current_time = timeline.get_current_time()
                    if current_time < previous_time:
                        print("Sequence stopped: simulation time was reset or looped.")
                        return
                    previous_time = current_time
        print("Target sequence complete: A → B → A → B. Target left at B.")
        print("This is a timed sequence, not verification that the arm reached each target.")
    except asyncio.CancelledError:
        print("Previous target sequence cancelled.")
        raise
    except Exception as error:
        print(f"Target sequence failed: {error}")
        raise
    finally:
        if callback_added and simulation.physics_callback_exists(CALLBACK_NAME):
            simulation.remove_physics_callback(CALLBACK_NAME)
        if context.get_stage() == stage and timeline.is_playing():
            timeline.pause()


# Re-running in the same Script Editor tab cancels its previous sequence.
previous_task = globals().get("_two_targets_task")
_two_targets_task = asyncio.ensure_future(run_two_targets(previous_task))