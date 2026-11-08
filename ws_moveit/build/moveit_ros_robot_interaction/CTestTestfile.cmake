# CMake generated Testfile for 
# Source directory: /mnt/moveit/ws_moveit/src/moveit2/moveit_ros/robot_interaction
# Build directory: /mnt/moveit/ws_moveit/build/moveit_ros_robot_interaction
# 
# This file includes the relevant testing commands required for 
# testing this directory and lists subdirectories to be tested as well.
add_test([=[locked_robot_state_test]=] "/usr/bin/python3" "-u" "/opt/ros/jazzy/share/ament_cmake_test/cmake/run_test.py" "/mnt/moveit/ws_moveit/build/moveit_ros_robot_interaction/test_results/moveit_ros_robot_interaction/locked_robot_state_test.gtest.xml" "--package-name" "moveit_ros_robot_interaction" "--output-file" "/mnt/moveit/ws_moveit/build/moveit_ros_robot_interaction/ament_cmake_gtest/locked_robot_state_test.txt" "--command" "/mnt/moveit/ws_moveit/build/moveit_ros_robot_interaction/locked_robot_state_test" "--gtest_output=xml:/mnt/moveit/ws_moveit/build/moveit_ros_robot_interaction/test_results/moveit_ros_robot_interaction/locked_robot_state_test.gtest.xml")
set_tests_properties([=[locked_robot_state_test]=] PROPERTIES  LABELS "gtest" REQUIRED_FILES "/mnt/moveit/ws_moveit/build/moveit_ros_robot_interaction/locked_robot_state_test" TIMEOUT "60" WORKING_DIRECTORY "/mnt/moveit/ws_moveit/build/moveit_ros_robot_interaction" _BACKTRACE_TRIPLES "/opt/ros/jazzy/share/ament_cmake_test/cmake/ament_add_test.cmake;125;add_test;/opt/ros/jazzy/share/ament_cmake_gtest/cmake/ament_add_gtest_test.cmake;95;ament_add_test;/opt/ros/jazzy/share/ament_cmake_gtest/cmake/ament_add_gtest.cmake;93;ament_add_gtest_test;/mnt/moveit/ws_moveit/src/moveit2/moveit_ros/robot_interaction/CMakeLists.txt;53;ament_add_gtest;/mnt/moveit/ws_moveit/src/moveit2/moveit_ros/robot_interaction/CMakeLists.txt;0;")
subdirs("gtest")
