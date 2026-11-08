# CMake generated Testfile for 
# Source directory: /mnt/moveit/ws_moveit/src/moveit2/moveit_kinematics/kdl_kinematics_plugin
# Build directory: /mnt/moveit/ws_moveit/build/moveit_kinematics/kdl_kinematics_plugin
# 
# This file includes the relevant testing commands required for 
# testing this directory and lists subdirectories to be tested as well.
add_test([=[test_kdl_pseudoinverse]=] "/usr/bin/python3" "-u" "/opt/ros/jazzy/share/ament_cmake_test/cmake/run_test.py" "/mnt/moveit/ws_moveit/build/moveit_kinematics/test_results/moveit_kinematics/test_kdl_pseudoinverse.gtest.xml" "--package-name" "moveit_kinematics" "--output-file" "/mnt/moveit/ws_moveit/build/moveit_kinematics/ament_cmake_gtest/test_kdl_pseudoinverse.txt" "--command" "/mnt/moveit/ws_moveit/build/moveit_kinematics/kdl_kinematics_plugin/test_kdl_pseudoinverse" "--gtest_output=xml:/mnt/moveit/ws_moveit/build/moveit_kinematics/test_results/moveit_kinematics/test_kdl_pseudoinverse.gtest.xml")
set_tests_properties([=[test_kdl_pseudoinverse]=] PROPERTIES  LABELS "gtest" REQUIRED_FILES "/mnt/moveit/ws_moveit/build/moveit_kinematics/kdl_kinematics_plugin/test_kdl_pseudoinverse" TIMEOUT "60" WORKING_DIRECTORY "/mnt/moveit/ws_moveit/build/moveit_kinematics/kdl_kinematics_plugin" _BACKTRACE_TRIPLES "/opt/ros/jazzy/share/ament_cmake_test/cmake/ament_add_test.cmake;125;add_test;/opt/ros/jazzy/share/ament_cmake_gtest/cmake/ament_add_gtest_test.cmake;95;ament_add_test;/opt/ros/jazzy/share/ament_cmake_gtest/cmake/ament_add_gtest.cmake;93;ament_add_gtest_test;/mnt/moveit/ws_moveit/src/moveit2/moveit_kinematics/kdl_kinematics_plugin/CMakeLists.txt;34;ament_add_gtest;/mnt/moveit/ws_moveit/src/moveit2/moveit_kinematics/kdl_kinematics_plugin/CMakeLists.txt;0;")
subdirs("../gtest")
