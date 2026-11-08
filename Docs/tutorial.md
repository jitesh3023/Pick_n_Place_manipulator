# For launching the moveit2_tutorial
Open a terminal
1) source /opt/ros/jazzy/setup.bash
2) source /mnt/moveit/ws_moveit/install/setup.bash
3) AMENT_PREFIX_PATH="/mnt/moveit/ws_moveit/compat/kinova_rviz${AMENT_PREFIX_PATH:+:$AMENT_PREFIX_PATH}" \
  ros2 launch moveit2_tutorials demo.launch.py
  ros2 launch moveit2_tutorials demo.launch.py rviz_config:=kinova_hello_moveit.rviz

# For running the custom node
1) open another terminal
2) source /opt/ros/jazzy/setup.bash
3) source /mnt/moveit/ws_moveit/install/setup.bash
4) ros2 run hello_moveit hello_moveit
5) After every change to the node we have to build the package and source it again-?
    a) colcon build --packages-select hello_moveit --mixin debug
    b) source /mnt/moveit/ws_moveit/install/setup.bash