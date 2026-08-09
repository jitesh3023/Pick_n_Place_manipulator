# Step 02 - Run First Grasp Sampling

## Objective
Run the starter sampler on primitive objects and inspect outputs.

## Tasks
1. From project root, run:
   - `python phase2/src/grasp_sampler.py --primitive cylinder --samples 100`
2. Open output JSON:
   - `phase2/output/grasps.json`
3. Repeat with:
   - `--primitive box`
   - `--primitive capsule`

## Validation
- Script reports `Candidates checked: 100`.
- Output contains non-empty list of valid grasps for at least one primitive.
- `jaw_width` values are less than or equal to `max-jaw`.
