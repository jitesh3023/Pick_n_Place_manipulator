# generated from rosidl_cmake/cmake/rosidl_cmake_aggregate_target-extras.cmake.in

# Create a convenience aggregate target moveit_msgs::moveit_msgs
# that links all generated interface targets, so downstream packages can use
# a single modern CMake target name instead of ${moveit_msgs_TARGETS}.
if(moveit_msgs_TARGETS AND NOT TARGET moveit_msgs::moveit_msgs)
  add_library(moveit_msgs::moveit_msgs INTERFACE IMPORTED)
  set_target_properties(moveit_msgs::moveit_msgs PROPERTIES
    INTERFACE_LINK_LIBRARIES "${moveit_msgs_TARGETS}")
endif()
