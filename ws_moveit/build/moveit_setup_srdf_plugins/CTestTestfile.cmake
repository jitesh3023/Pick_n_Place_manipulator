# CMake generated Testfile for 
# Source directory: /mnt/moveit/ws_moveit/src/moveit2/moveit_setup_assistant/moveit_setup_srdf_plugins
# Build directory: /mnt/moveit/ws_moveit/build/moveit_setup_srdf_plugins
# 
# This file includes the relevant testing commands required for 
# testing this directory and lists subdirectories to be tested as well.
add_test([=[test_srdf]=] "/usr/bin/python3" "-u" "/opt/ros/jazzy/share/ament_cmake_test/cmake/run_test.py" "/mnt/moveit/ws_moveit/build/moveit_setup_srdf_plugins/test_results/moveit_setup_srdf_plugins/test_srdf.gtest.xml" "--package-name" "moveit_setup_srdf_plugins" "--output-file" "/mnt/moveit/ws_moveit/build/moveit_setup_srdf_plugins/ament_cmake_gtest/test_srdf.txt" "--command" "/mnt/moveit/ws_moveit/build/moveit_setup_srdf_plugins/test_srdf" "--gtest_output=xml:/mnt/moveit/ws_moveit/build/moveit_setup_srdf_plugins/test_results/moveit_setup_srdf_plugins/test_srdf.gtest.xml")
set_tests_properties([=[test_srdf]=] PROPERTIES  LABELS "gtest" REQUIRED_FILES "/mnt/moveit/ws_moveit/build/moveit_setup_srdf_plugins/test_srdf" TIMEOUT "60" WORKING_DIRECTORY "/mnt/moveit/ws_moveit/build/moveit_setup_srdf_plugins" _BACKTRACE_TRIPLES "/opt/ros/jazzy/share/ament_cmake_test/cmake/ament_add_test.cmake;125;add_test;/opt/ros/jazzy/share/ament_cmake_gtest/cmake/ament_add_gtest_test.cmake;95;ament_add_test;/opt/ros/jazzy/share/ament_cmake_gtest/cmake/ament_add_gtest.cmake;93;ament_add_gtest_test;/mnt/moveit/ws_moveit/src/moveit2/moveit_setup_assistant/moveit_setup_srdf_plugins/CMakeLists.txt;71;ament_add_gtest;/mnt/moveit/ws_moveit/src/moveit2/moveit_setup_assistant/moveit_setup_srdf_plugins/CMakeLists.txt;0;")
subdirs("gtest")
