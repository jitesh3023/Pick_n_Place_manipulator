# Step 03 - Evaluate External Meshes

## Objective
Test the sampler on real object meshes (mug, drill, bottle, etc.).

## Tasks
1. Create folder `phase2/assets/meshes/`.
2. Add at least one mesh file (`.stl`, `.obj`, or `.ply`).
3. Run:
   - `python phase2/src/grasp_sampler.py --mesh phase2/assets/meshes/your_mesh.stl --samples 100`
4. Compare valid grasp count across 2-3 mesh types.
# conda run -n graspgen python phase2/src/grasp_sampler.py --mesh phase2/assets/meshes/your_mesh.stl --samples 100
## Validation
- Script loads mesh without type errors.
- Output JSON is generated and parseable.
- At least one candidate is valid or you can explain why none passed (jaw limit too small, object too large).

## Stretch Goal
Add a score field that prefers smaller jaw openings and more centered contacts.
