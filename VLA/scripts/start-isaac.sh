#!/bin/bash

# Initialize Conda in this script, then deactivate any active environments.
if [[ "${CONDA_SHLVL:-0}" -gt 0 ]]; then
	if [[ ! -x "${CONDA_EXE:-}" ]]; then
		echo "Cannot locate Conda. Run conda deactivate in your terminal first." >&2
		exit 1
	fi
	conda_setup="$("$CONDA_EXE" shell.bash hook)" || exit 1
	eval "$conda_setup"
	while [[ "${CONDA_SHLVL:-0}" -gt 0 ]]; do
		conda deactivate || exit 1
	done
fi

# Use Isaac Sim's bundled libraries, not the terminal's system CUDA/Python paths.
unset LD_LIBRARY_PATH LD_PRELOAD CUDA_HOME CUDA_PATH PYTHONPATH PYTHONHOME

cd "/media/jitesh/Extreme SSD/isaac/isaacsim" || exit 1
./isaac-sim.sh --no-ros-env