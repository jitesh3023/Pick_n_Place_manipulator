# Phase 2 - Synthetic Grasp Generator

In this phase, you will write a script that:
1. Loads a mesh object.
2. Samples candidate approach angles around the object.
3. Rejects grasps where simple finger geometry collides with the mesh.
4. Exports valid grasp poses.

## Deliverables
- Script that evaluates at least 100 angle samples
- Output file with valid grasp candidates
- Visual sanity check using trimesh scene

## Quick Start
1. Create and activate a virtual environment.
2. Install dependencies from `requirements.txt`.
3. Run:
   - `python src/grasp_sampler.py --primitive cylinder --samples 100`

## Progress Checklist
- [ ] Python environment ready
- [ ] Dependencies installed
- [ ] Script runs on primitive meshes
- [ ] Script runs on at least one external mesh
- [ ] Valid grasps saved to `phase2/output/`
