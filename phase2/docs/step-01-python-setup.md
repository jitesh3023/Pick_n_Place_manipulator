# Step 01 - Python Environment Setup

## Objective
Create a clean Python environment for the grasp generator.

## Tasks
1. Open terminal in project root.
2. Create virtual environment:
   - `python3 -m venv .venv`
3. Activate environment:
   - Linux/macOS: `source .venv/bin/activate`
4. Install dependencies:
   - `pip install -r phase2/requirements.txt`

## Validation
- `python --version` works from active venv.
- `python -c "import trimesh, numpy; print('ok')"` prints `ok`.
