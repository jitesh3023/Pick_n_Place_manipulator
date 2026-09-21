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
# Do not carry a sourced system ROS installation or workspace overlay into Isaac.
unset AMENT_PREFIX_PATH CMAKE_PREFIX_PATH COLCON_PREFIX_PATH ROS_PACKAGE_PATH
unset ROS_VERSION ROS_PYTHON_VERSION

cd "/media/jitesh/Extreme SSD/isaac/isaacsim" || exit 1

# Use the Jazzy libraries built for Isaac's embedded Python, not system Python.
# Set these explicitly: the installed launcher uses nounset in its ROS setup.
export ROS_DISTRO=jazzy
export RMW_IMPLEMENTATION=rmw_fastrtps_cpp
export LD_LIBRARY_PATH="$PWD/exts/isaacsim.ros2.bridge/jazzy/lib"
if [[ ! -f "$LD_LIBRARY_PATH/librmw_fastrtps_cpp.so" ]]; then
	echo "Isaac Sim's bundled Jazzy libraries are missing: $LD_LIBRARY_PATH" >&2
	exit 1
fi

exec ./isaac-sim.sh --enable isaacsim.ros2.bridge "$@"