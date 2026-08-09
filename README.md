# Pick_n_Place_manipulator (2-Phase Project)

This project is a practical path to learn industrial robot programming and grasp planning.

## Phase 1: Digital Twin Warehouse (Industrial Setup)
Goal: Build a virtual pick-and-place robotic cell that sorts boxes from a conveyor.

What I learnt:
- Teach pendant basics (jogging axes, frames, speed)
- TCP (Tool Center Point) definition for a vacuum gripper
- Waypoints and hardcoded motion loops

Primary tools:
- URSim

## Phase 2: Synthetic Grasp Generator (Grasp Factory Replica)
Goal: Write a Python pipeline that samples grasp angles around 3D objects and filters collision-prone grasps.

What I learnt:
- 3D mesh handling
- Collision checks for gripper fingers
- Basic forward/inverse kinematics concepts in application

Primary tools:
- Python 3.10+
- trimesh, numpy

## Project Structure
- `phase1/`: Digital twin simulation assets and runbook
- `phase2/`: Python grasp-generation code

## How We Will Work Step-by-Step
1. Complete `phase1/docs/step-01-setup.md` and verify simulator setup.
2. Build the first hardcoded pick-and-place cycle in `phase1`.
3. Set up Python environment and run `phase2/src/grasp_sampler.py`.
4. Add better collision filtering and scoring in Phase 2.
5. Integrate both phases conceptually (digital twin + grasp logic).

## Success Criteria
- Phase 1: Robot repeatedly sorts boxes into at least 2 bins in simulation.
- Phase 2: Script outputs valid grasp candidates for at least 2 object meshes.
