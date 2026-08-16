# Step 01 - Python Environment Setup

## Objective
Create a clean Python environment for the grasp generator.

## Tasks
1. Open terminal in project root.
2. Create and activate a Python environment using one of these options:
   - `venv` option:
     - `python3 -m venv .venv`
     - `source .venv/bin/activate`
   - `conda` option:
     - `conda create -n graspgen python=3.11 -y`
     - `conda activate graspgen`
3. Install dependencies:
   - `pip install -r phase2/requirements.txt`

## Validation
- `python --version` works from the active environment.
- `python -c "import trimesh, numpy; print('ok')"` prints `ok`.
