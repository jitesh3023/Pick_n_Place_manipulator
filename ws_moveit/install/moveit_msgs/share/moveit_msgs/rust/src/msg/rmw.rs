#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__msg__AllowedCollisionEntry() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__msg__AllowedCollisionEntry__init(msg: *mut AllowedCollisionEntry) -> bool;
    fn moveit_msgs__msg__AllowedCollisionEntry__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<AllowedCollisionEntry>, size: usize) -> bool;
    fn moveit_msgs__msg__AllowedCollisionEntry__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<AllowedCollisionEntry>);
    fn moveit_msgs__msg__AllowedCollisionEntry__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<AllowedCollisionEntry>, out_seq: *mut rosidl_runtime_rs::Sequence<AllowedCollisionEntry>) -> bool;
}

// Corresponds to moveit_msgs__msg__AllowedCollisionEntry
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// whether or not collision checking is enabled

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AllowedCollisionEntry {

    // This member is not documented.
    #[allow(missing_docs)]
    pub enabled: rosidl_runtime_rs::Sequence<bool>,

}



impl Default for AllowedCollisionEntry {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__msg__AllowedCollisionEntry__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__msg__AllowedCollisionEntry__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for AllowedCollisionEntry {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__AllowedCollisionEntry__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__AllowedCollisionEntry__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__AllowedCollisionEntry__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for AllowedCollisionEntry {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for AllowedCollisionEntry where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/msg/AllowedCollisionEntry";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__msg__AllowedCollisionEntry() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__msg__AllowedCollisionMatrix() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__msg__AllowedCollisionMatrix__init(msg: *mut AllowedCollisionMatrix) -> bool;
    fn moveit_msgs__msg__AllowedCollisionMatrix__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<AllowedCollisionMatrix>, size: usize) -> bool;
    fn moveit_msgs__msg__AllowedCollisionMatrix__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<AllowedCollisionMatrix>);
    fn moveit_msgs__msg__AllowedCollisionMatrix__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<AllowedCollisionMatrix>, out_seq: *mut rosidl_runtime_rs::Sequence<AllowedCollisionMatrix>) -> bool;
}

// Corresponds to moveit_msgs__msg__AllowedCollisionMatrix
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// The list of entry names in the matrix

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AllowedCollisionMatrix {

    // This member is not documented.
    #[allow(missing_docs)]
    pub entry_names: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,

    /// The individual entries in the allowed collision matrix
    /// square, symmetric, with same order as entry_names
    pub entry_values: rosidl_runtime_rs::Sequence<super::super::msg::rmw::AllowedCollisionEntry>,

    /// In addition to the collision matrix itself, we also have
    /// the default entry value for each entry name.
    /// If the allowed collision flag is queried for a pair of names (n1, n2)
    /// that is not found in the collision matrix itself, the value of
    /// the collision flag is considered to be that of the entry (n1 or n2)
    /// specified in the list below. If both n1 and n2 are found in the list
    /// of defaults, the result is computed with an AND operation
    pub default_entry_names: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub default_entry_values: rosidl_runtime_rs::Sequence<bool>,

}



impl Default for AllowedCollisionMatrix {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__msg__AllowedCollisionMatrix__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__msg__AllowedCollisionMatrix__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for AllowedCollisionMatrix {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__AllowedCollisionMatrix__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__AllowedCollisionMatrix__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__AllowedCollisionMatrix__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for AllowedCollisionMatrix {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for AllowedCollisionMatrix where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/msg/AllowedCollisionMatrix";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__msg__AllowedCollisionMatrix() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__msg__AttachedCollisionObject() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__msg__AttachedCollisionObject__init(msg: *mut AttachedCollisionObject) -> bool;
    fn moveit_msgs__msg__AttachedCollisionObject__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<AttachedCollisionObject>, size: usize) -> bool;
    fn moveit_msgs__msg__AttachedCollisionObject__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<AttachedCollisionObject>);
    fn moveit_msgs__msg__AttachedCollisionObject__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<AttachedCollisionObject>, out_seq: *mut rosidl_runtime_rs::Sequence<AttachedCollisionObject>) -> bool;
}

// Corresponds to moveit_msgs__msg__AttachedCollisionObject
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// The CollisionObject will be attached with a fixed joint to this link

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AttachedCollisionObject {

    // This member is not documented.
    #[allow(missing_docs)]
    pub link_name: rosidl_runtime_rs::String,

    /// This contains the actual shapes and poses for the CollisionObject
    /// to be attached to the link
    /// If action is remove and no object.id is set, all objects
    /// attached to the link indicated by link_name will be removed
    pub object: super::super::msg::rmw::CollisionObject,

    /// The set of links that the attached objects are allowed to touch
    /// by default - the link_name is already considered by default
    pub touch_links: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,

    /// If certain links were placed in a particular posture for this object to remain attached
    /// (e.g., an end effector closing around an object), the posture necessary for releasing
    /// the object is stored here
    pub detach_posture: trajectory_msgs::msg::rmw::JointTrajectory,

    /// The weight of the attached object, if known
    pub weight: f64,

}



impl Default for AttachedCollisionObject {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__msg__AttachedCollisionObject__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__msg__AttachedCollisionObject__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for AttachedCollisionObject {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__AttachedCollisionObject__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__AttachedCollisionObject__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__AttachedCollisionObject__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for AttachedCollisionObject {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for AttachedCollisionObject where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/msg/AttachedCollisionObject";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__msg__AttachedCollisionObject() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__msg__BoundingVolume() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__msg__BoundingVolume__init(msg: *mut BoundingVolume) -> bool;
    fn moveit_msgs__msg__BoundingVolume__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<BoundingVolume>, size: usize) -> bool;
    fn moveit_msgs__msg__BoundingVolume__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<BoundingVolume>);
    fn moveit_msgs__msg__BoundingVolume__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<BoundingVolume>, out_seq: *mut rosidl_runtime_rs::Sequence<BoundingVolume>) -> bool;
}

// Corresponds to moveit_msgs__msg__BoundingVolume
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Define a volume in 3D

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BoundingVolume {
    /// A set of solid geometric primitives that make up the volume to define (as a union)
    pub primitives: rosidl_runtime_rs::Sequence<shape_msgs::msg::rmw::SolidPrimitive>,

    /// The poses at which the primitives are located
    pub primitive_poses: rosidl_runtime_rs::Sequence<geometry_msgs::msg::rmw::Pose>,

    /// In addition to primitives, meshes can be specified to add to the bounding volume (again, as union)
    pub meshes: rosidl_runtime_rs::Sequence<shape_msgs::msg::rmw::Mesh>,

    /// The poses at which the meshes are located
    pub mesh_poses: rosidl_runtime_rs::Sequence<geometry_msgs::msg::rmw::Pose>,

}



impl Default for BoundingVolume {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__msg__BoundingVolume__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__msg__BoundingVolume__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for BoundingVolume {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__BoundingVolume__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__BoundingVolume__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__BoundingVolume__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for BoundingVolume {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for BoundingVolume where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/msg/BoundingVolume";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__msg__BoundingVolume() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__msg__CartesianPoint() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__msg__CartesianPoint__init(msg: *mut CartesianPoint) -> bool;
    fn moveit_msgs__msg__CartesianPoint__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<CartesianPoint>, size: usize) -> bool;
    fn moveit_msgs__msg__CartesianPoint__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<CartesianPoint>);
    fn moveit_msgs__msg__CartesianPoint__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<CartesianPoint>, out_seq: *mut rosidl_runtime_rs::Sequence<CartesianPoint>) -> bool;
}

// Corresponds to moveit_msgs__msg__CartesianPoint
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// This message defines a point in a cartesian trajectory

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CartesianPoint {

    // This member is not documented.
    #[allow(missing_docs)]
    pub pose: geometry_msgs::msg::rmw::Pose,


    // This member is not documented.
    #[allow(missing_docs)]
    pub velocity: geometry_msgs::msg::rmw::Twist,


    // This member is not documented.
    #[allow(missing_docs)]
    pub acceleration: geometry_msgs::msg::rmw::Accel,

}



impl Default for CartesianPoint {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__msg__CartesianPoint__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__msg__CartesianPoint__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for CartesianPoint {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__CartesianPoint__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__CartesianPoint__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__CartesianPoint__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for CartesianPoint {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for CartesianPoint where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/msg/CartesianPoint";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__msg__CartesianPoint() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__msg__CartesianTrajectory() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__msg__CartesianTrajectory__init(msg: *mut CartesianTrajectory) -> bool;
    fn moveit_msgs__msg__CartesianTrajectory__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<CartesianTrajectory>, size: usize) -> bool;
    fn moveit_msgs__msg__CartesianTrajectory__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<CartesianTrajectory>);
    fn moveit_msgs__msg__CartesianTrajectory__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<CartesianTrajectory>, out_seq: *mut rosidl_runtime_rs::Sequence<CartesianTrajectory>) -> bool;
}

// Corresponds to moveit_msgs__msg__CartesianTrajectory
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// This message describes the trajectory of a tracked frame in task-space

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CartesianTrajectory {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,

    /// The name of the Cartesian frame being tracked with respect to the base frame provided in header.frame_id
    pub tracked_frame: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub points: rosidl_runtime_rs::Sequence<super::super::msg::rmw::CartesianTrajectoryPoint>,

}



impl Default for CartesianTrajectory {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__msg__CartesianTrajectory__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__msg__CartesianTrajectory__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for CartesianTrajectory {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__CartesianTrajectory__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__CartesianTrajectory__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__CartesianTrajectory__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for CartesianTrajectory {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for CartesianTrajectory where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/msg/CartesianTrajectory";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__msg__CartesianTrajectory() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__msg__CartesianTrajectoryPoint() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__msg__CartesianTrajectoryPoint__init(msg: *mut CartesianTrajectoryPoint) -> bool;
    fn moveit_msgs__msg__CartesianTrajectoryPoint__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<CartesianTrajectoryPoint>, size: usize) -> bool;
    fn moveit_msgs__msg__CartesianTrajectoryPoint__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<CartesianTrajectoryPoint>);
    fn moveit_msgs__msg__CartesianTrajectoryPoint__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<CartesianTrajectoryPoint>, out_seq: *mut rosidl_runtime_rs::Sequence<CartesianTrajectoryPoint>) -> bool;
}

// Corresponds to moveit_msgs__msg__CartesianTrajectoryPoint
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// The definition of a cartesian point in a trajectory. Defines the cartesian state of the point and it's time,
/// following the pattern of the JointTrajectory message

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CartesianTrajectoryPoint {

    // This member is not documented.
    #[allow(missing_docs)]
    pub point: super::super::msg::rmw::CartesianPoint,


    // This member is not documented.
    #[allow(missing_docs)]
    pub time_from_start: builtin_interfaces::msg::rmw::Duration,

}



impl Default for CartesianTrajectoryPoint {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__msg__CartesianTrajectoryPoint__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__msg__CartesianTrajectoryPoint__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for CartesianTrajectoryPoint {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__CartesianTrajectoryPoint__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__CartesianTrajectoryPoint__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__CartesianTrajectoryPoint__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for CartesianTrajectoryPoint {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for CartesianTrajectoryPoint where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/msg/CartesianTrajectoryPoint";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__msg__CartesianTrajectoryPoint() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__msg__CollisionObject() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__msg__CollisionObject__init(msg: *mut CollisionObject) -> bool;
    fn moveit_msgs__msg__CollisionObject__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<CollisionObject>, size: usize) -> bool;
    fn moveit_msgs__msg__CollisionObject__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<CollisionObject>);
    fn moveit_msgs__msg__CollisionObject__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<CollisionObject>, out_seq: *mut rosidl_runtime_rs::Sequence<CollisionObject>) -> bool;
}

// Corresponds to moveit_msgs__msg__CollisionObject
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// a header, used for interpreting the poses

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CollisionObject {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,

    /// The object's pose relative to the header frame.
    /// The shapes and subframe poses are defined relative to this pose.
    pub pose: geometry_msgs::msg::rmw::Pose,

    /// The id of the object (name used in MoveIt)
    pub id: rosidl_runtime_rs::String,

    /// The object type in a database of known objects
    pub type_: object_recognition_msgs::msg::rmw::ObjectType,

    /// The collision geometries associated with the object.
    /// Their poses are with respect to the object's pose
    /// Solid geometric primitives
    pub primitives: rosidl_runtime_rs::Sequence<shape_msgs::msg::rmw::SolidPrimitive>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub primitive_poses: rosidl_runtime_rs::Sequence<geometry_msgs::msg::rmw::Pose>,

    /// Meshes
    pub meshes: rosidl_runtime_rs::Sequence<shape_msgs::msg::rmw::Mesh>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub mesh_poses: rosidl_runtime_rs::Sequence<geometry_msgs::msg::rmw::Pose>,

    /// Bounding planes (equation is specified, but the plane can be oriented using an additional pose)
    pub planes: rosidl_runtime_rs::Sequence<shape_msgs::msg::rmw::Plane>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub plane_poses: rosidl_runtime_rs::Sequence<geometry_msgs::msg::rmw::Pose>,

    /// Named subframes on the object. Use these to define points of interest on the object that you want
    /// to plan with (e.g. "tip", "spout", "handle"). The id of the object will be prepended to the subframe.
    /// If an object with the id "screwdriver" and a subframe "tip" is in the scene, you can use the frame
    /// "screwdriver/tip" for planning.
    /// The length of the subframe_names and subframe_poses has to be identical.
    pub subframe_names: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub subframe_poses: rosidl_runtime_rs::Sequence<geometry_msgs::msg::rmw::Pose>,

    /// Operation to be performed
    pub operation: u8,

}

impl CollisionObject {
    /// Adds the object to the planning scene. If the object previously existed, it is replaced.
    pub const ADD: u8 = 0;

    /// Removes the object from the environment entirely (everything that matches the specified id)
    pub const REMOVE: u8 = 1;

    /// Append to an object that already exists in the planning scene. If the object does not exist, it is added.
    pub const APPEND: u8 = 2;

    /// If an object already exists in the scene, new poses can be sent (the geometry arrays must be left empty)
    /// if solely moving the object is desired
    pub const MOVE: u8 = 3;

}


impl Default for CollisionObject {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__msg__CollisionObject__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__msg__CollisionObject__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for CollisionObject {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__CollisionObject__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__CollisionObject__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__CollisionObject__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for CollisionObject {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for CollisionObject where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/msg/CollisionObject";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__msg__CollisionObject() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__msg__ConstraintEvalResult() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__msg__ConstraintEvalResult__init(msg: *mut ConstraintEvalResult) -> bool;
    fn moveit_msgs__msg__ConstraintEvalResult__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ConstraintEvalResult>, size: usize) -> bool;
    fn moveit_msgs__msg__ConstraintEvalResult__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ConstraintEvalResult>);
    fn moveit_msgs__msg__ConstraintEvalResult__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ConstraintEvalResult>, out_seq: *mut rosidl_runtime_rs::Sequence<ConstraintEvalResult>) -> bool;
}

// Corresponds to moveit_msgs__msg__ConstraintEvalResult
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// This message contains result from constraint evaluation
/// result specifies the result of constraint evaluation
/// (true indicates state satisfies constraint, false indicates state violates constraint)
/// if false, distance specifies a measure of the distance of the state from the constraint
/// if true, distance is set to zero

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ConstraintEvalResult {

    // This member is not documented.
    #[allow(missing_docs)]
    pub result: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub distance: f64,

}



impl Default for ConstraintEvalResult {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__msg__ConstraintEvalResult__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__msg__ConstraintEvalResult__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ConstraintEvalResult {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__ConstraintEvalResult__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__ConstraintEvalResult__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__ConstraintEvalResult__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ConstraintEvalResult {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ConstraintEvalResult where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/msg/ConstraintEvalResult";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__msg__ConstraintEvalResult() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__msg__Constraints() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__msg__Constraints__init(msg: *mut Constraints) -> bool;
    fn moveit_msgs__msg__Constraints__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Constraints>, size: usize) -> bool;
    fn moveit_msgs__msg__Constraints__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Constraints>);
    fn moveit_msgs__msg__Constraints__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Constraints>, out_seq: *mut rosidl_runtime_rs::Sequence<Constraints>) -> bool;
}

// Corresponds to moveit_msgs__msg__Constraints
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// This message contains a list of motion planning constraints.
/// All constraints must be satisfied for a goal to be considered valid

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Constraints {

    // This member is not documented.
    #[allow(missing_docs)]
    pub name: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub joint_constraints: rosidl_runtime_rs::Sequence<super::super::msg::rmw::JointConstraint>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub position_constraints: rosidl_runtime_rs::Sequence<super::super::msg::rmw::PositionConstraint>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub orientation_constraints: rosidl_runtime_rs::Sequence<super::super::msg::rmw::OrientationConstraint>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub visibility_constraints: rosidl_runtime_rs::Sequence<super::super::msg::rmw::VisibilityConstraint>,

}



impl Default for Constraints {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__msg__Constraints__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__msg__Constraints__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Constraints {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__Constraints__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__Constraints__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__Constraints__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Constraints {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Constraints where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/msg/Constraints";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__msg__Constraints() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__msg__CostSource() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__msg__CostSource__init(msg: *mut CostSource) -> bool;
    fn moveit_msgs__msg__CostSource__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<CostSource>, size: usize) -> bool;
    fn moveit_msgs__msg__CostSource__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<CostSource>);
    fn moveit_msgs__msg__CostSource__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<CostSource>, out_seq: *mut rosidl_runtime_rs::Sequence<CostSource>) -> bool;
}

// Corresponds to moveit_msgs__msg__CostSource
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// The density of the cost source

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CostSource {

    // This member is not documented.
    #[allow(missing_docs)]
    pub cost_density: f64,

    /// The volume of the cost source is represented as an
    /// axis-aligned bounding box (AABB)
    /// The AABB is specified by two of its opposite corners
    pub aabb_min: geometry_msgs::msg::rmw::Vector3,


    // This member is not documented.
    #[allow(missing_docs)]
    pub aabb_max: geometry_msgs::msg::rmw::Vector3,

}



impl Default for CostSource {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__msg__CostSource__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__msg__CostSource__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for CostSource {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__CostSource__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__CostSource__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__CostSource__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for CostSource {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for CostSource where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/msg/CostSource";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__msg__CostSource() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__msg__ContactInformation() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__msg__ContactInformation__init(msg: *mut ContactInformation) -> bool;
    fn moveit_msgs__msg__ContactInformation__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ContactInformation>, size: usize) -> bool;
    fn moveit_msgs__msg__ContactInformation__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ContactInformation>);
    fn moveit_msgs__msg__ContactInformation__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ContactInformation>, out_seq: *mut rosidl_runtime_rs::Sequence<ContactInformation>) -> bool;
}

// Corresponds to moveit_msgs__msg__ContactInformation
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Standard ROS header contains information
/// about the frame in which this
/// contact is specified

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ContactInformation {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,

    /// Position of the contact point
    pub position: geometry_msgs::msg::rmw::Point,

    /// Normal corresponding to the contact point
    pub normal: geometry_msgs::msg::rmw::Vector3,

    /// Depth of contact point
    pub depth: f64,

    /// Name of the first body that is in contact
    /// This could be a link or a namespace that represents a body
    pub contact_body_1: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub body_type_1: u32,

    /// Name of the second body that is in contact
    /// This could be a link or a namespace that represents a body
    pub contact_body_2: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub body_type_2: u32,

}

impl ContactInformation {

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const ROBOT_LINK: u32 = 0;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const WORLD_OBJECT: u32 = 1;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const ROBOT_ATTACHED: u32 = 2;

}


impl Default for ContactInformation {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__msg__ContactInformation__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__msg__ContactInformation__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ContactInformation {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__ContactInformation__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__ContactInformation__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__ContactInformation__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ContactInformation {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ContactInformation where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/msg/ContactInformation";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__msg__ContactInformation() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__msg__DisplayTrajectory() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__msg__DisplayTrajectory__init(msg: *mut DisplayTrajectory) -> bool;
    fn moveit_msgs__msg__DisplayTrajectory__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DisplayTrajectory>, size: usize) -> bool;
    fn moveit_msgs__msg__DisplayTrajectory__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DisplayTrajectory>);
    fn moveit_msgs__msg__DisplayTrajectory__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DisplayTrajectory>, out_seq: *mut rosidl_runtime_rs::Sequence<DisplayTrajectory>) -> bool;
}

// Corresponds to moveit_msgs__msg__DisplayTrajectory
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// The model id for which this path has been generated

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DisplayTrajectory {

    // This member is not documented.
    #[allow(missing_docs)]
    pub model_id: rosidl_runtime_rs::String,

    /// The representation of the path contains position values for all the joints that are moving along the path;
    /// a sequence of trajectories may be specified
    pub trajectory: rosidl_runtime_rs::Sequence<super::super::msg::rmw::RobotTrajectory>,

    /// The robot state is used to obtain positions for all/some of the joints of the robot.
    /// It is used by the path display node to determine the positions of the joints that are not specified
    /// in the joint trajectory message above. The positions in the joint trajectory message
    /// take precedence (for joints that the message covers).
    pub trajectory_start: super::super::msg::rmw::RobotState,

}



impl Default for DisplayTrajectory {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__msg__DisplayTrajectory__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__msg__DisplayTrajectory__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DisplayTrajectory {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__DisplayTrajectory__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__DisplayTrajectory__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__DisplayTrajectory__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DisplayTrajectory {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DisplayTrajectory where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/msg/DisplayTrajectory";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__msg__DisplayTrajectory() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__msg__DisplayRobotState() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__msg__DisplayRobotState__init(msg: *mut DisplayRobotState) -> bool;
    fn moveit_msgs__msg__DisplayRobotState__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DisplayRobotState>, size: usize) -> bool;
    fn moveit_msgs__msg__DisplayRobotState__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DisplayRobotState>);
    fn moveit_msgs__msg__DisplayRobotState__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DisplayRobotState>, out_seq: *mut rosidl_runtime_rs::Sequence<DisplayRobotState>) -> bool;
}

// Corresponds to moveit_msgs__msg__DisplayRobotState
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// The robot state to display

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DisplayRobotState {

    // This member is not documented.
    #[allow(missing_docs)]
    pub state: super::super::msg::rmw::RobotState,

    /// Optionally, various links can be highlighted
    pub highlight_links: rosidl_runtime_rs::Sequence<super::super::msg::rmw::ObjectColor>,

    /// If true, suppress the display in visualizations (like rviz)
    pub hide: bool,

}



impl Default for DisplayRobotState {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__msg__DisplayRobotState__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__msg__DisplayRobotState__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DisplayRobotState {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__DisplayRobotState__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__DisplayRobotState__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__DisplayRobotState__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DisplayRobotState {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DisplayRobotState where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/msg/DisplayRobotState";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__msg__DisplayRobotState() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__msg__GenericTrajectory() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__msg__GenericTrajectory__init(msg: *mut GenericTrajectory) -> bool;
    fn moveit_msgs__msg__GenericTrajectory__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GenericTrajectory>, size: usize) -> bool;
    fn moveit_msgs__msg__GenericTrajectory__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GenericTrajectory>);
    fn moveit_msgs__msg__GenericTrajectory__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GenericTrajectory>, out_seq: *mut rosidl_runtime_rs::Sequence<GenericTrajectory>) -> bool;
}

// Corresponds to moveit_msgs__msg__GenericTrajectory
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// A generic trajectory message that may either encode a joint- or cartesian-space trajectory, or both
/// Trajectories encoded in this message are considered semantically equivalent

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GenericTrajectory {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub joint_trajectory: rosidl_runtime_rs::Sequence<trajectory_msgs::msg::rmw::JointTrajectory>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub cartesian_trajectory: rosidl_runtime_rs::Sequence<super::super::msg::rmw::CartesianTrajectory>,

}



impl Default for GenericTrajectory {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__msg__GenericTrajectory__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__msg__GenericTrajectory__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GenericTrajectory {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__GenericTrajectory__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__GenericTrajectory__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__GenericTrajectory__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GenericTrajectory {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GenericTrajectory where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/msg/GenericTrajectory";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__msg__GenericTrajectory() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__msg__Grasp() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__msg__Grasp__init(msg: *mut Grasp) -> bool;
    fn moveit_msgs__msg__Grasp__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Grasp>, size: usize) -> bool;
    fn moveit_msgs__msg__Grasp__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Grasp>);
    fn moveit_msgs__msg__Grasp__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Grasp>, out_seq: *mut rosidl_runtime_rs::Sequence<Grasp>) -> bool;
}

// Corresponds to moveit_msgs__msg__Grasp
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// This message contains a description of a grasp that would be used
/// with a particular end-effector to grasp an object, including how to
/// approach it, grip it, etc.  This message does not contain any
/// information about a "grasp point" (a position ON the object).
/// Whatever generates this message should have already combined
/// information about grasp points with information about the geometry
/// of the end-effector to compute the grasp_pose in this message.

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Grasp {
    /// A name for this grasp
    pub id: rosidl_runtime_rs::String,

    /// The internal posture of the hand for the pre-grasp
    /// only positions are used
    pub pre_grasp_posture: trajectory_msgs::msg::rmw::JointTrajectory,

    /// The internal posture of the hand for the grasp
    /// positions and efforts are used
    pub grasp_posture: trajectory_msgs::msg::rmw::JointTrajectory,

    /// The position of the end-effector for the grasp.  This is the pose of
    /// the "parent_link" of the end-effector, not actually the pose of any
    /// link *in* the end-effector.  Typically this would be the pose of the
    /// most distal wrist link before the hand (end-effector) links began.
    pub grasp_pose: geometry_msgs::msg::rmw::PoseStamped,

    /// The estimated probability of success for this grasp, or some other
    /// measure of how "good" it is.
    pub grasp_quality: f64,

    /// The approach direction to take before picking an object
    pub pre_grasp_approach: super::super::msg::rmw::GripperTranslation,

    /// The retreat direction to take after a grasp has been completed (object is attached)
    pub post_grasp_retreat: super::super::msg::rmw::GripperTranslation,

    /// The retreat motion to perform when releasing the object; this information
    /// is not necessary for the grasp itself, but when releasing the object,
    /// the information will be necessary. The grasp used to perform a pickup
    /// is returned as part of the result, so this information is available for
    /// later use.
    pub post_place_retreat: super::super::msg::rmw::GripperTranslation,

    /// the maximum contact force to use while grasping (<=0 to disable)
    pub max_contact_force: f32,

    /// an optional list of obstacles that we have semantic information about
    /// and that can be touched/pushed/moved in the course of grasping
    pub allowed_touch_objects: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,

}



impl Default for Grasp {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__msg__Grasp__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__msg__Grasp__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Grasp {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__Grasp__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__Grasp__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__Grasp__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Grasp {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Grasp where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/msg/Grasp";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__msg__Grasp() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__msg__GripperTranslation() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__msg__GripperTranslation__init(msg: *mut GripperTranslation) -> bool;
    fn moveit_msgs__msg__GripperTranslation__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GripperTranslation>, size: usize) -> bool;
    fn moveit_msgs__msg__GripperTranslation__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GripperTranslation>);
    fn moveit_msgs__msg__GripperTranslation__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GripperTranslation>, out_seq: *mut rosidl_runtime_rs::Sequence<GripperTranslation>) -> bool;
}

// Corresponds to moveit_msgs__msg__GripperTranslation
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// defines a translation for the gripper, used in pickup or place tasks
/// for example for lifting an object off a table or approaching the table for placing

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GripperTranslation {
    /// the direction of the translation
    pub direction: geometry_msgs::msg::rmw::Vector3Stamped,

    /// the desired translation distance
    pub desired_distance: f32,

    /// the min distance that must be considered feasible before the
    /// grasp is even attempted
    pub min_distance: f32,

}



impl Default for GripperTranslation {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__msg__GripperTranslation__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__msg__GripperTranslation__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GripperTranslation {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__GripperTranslation__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__GripperTranslation__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__GripperTranslation__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GripperTranslation {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GripperTranslation where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/msg/GripperTranslation";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__msg__GripperTranslation() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__msg__JointConstraint() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__msg__JointConstraint__init(msg: *mut JointConstraint) -> bool;
    fn moveit_msgs__msg__JointConstraint__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<JointConstraint>, size: usize) -> bool;
    fn moveit_msgs__msg__JointConstraint__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<JointConstraint>);
    fn moveit_msgs__msg__JointConstraint__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<JointConstraint>, out_seq: *mut rosidl_runtime_rs::Sequence<JointConstraint>) -> bool;
}

// Corresponds to moveit_msgs__msg__JointConstraint
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Constrain the position of a joint to be within a certain bound

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct JointConstraint {

    // This member is not documented.
    #[allow(missing_docs)]
    pub joint_name: rosidl_runtime_rs::String,

    /// the bound to be achieved is [position - tolerance_below, position + tolerance_above]
    pub position: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub tolerance_above: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub tolerance_below: f64,

    /// A weighting factor for this constraint (denotes relative importance to other constraints. Closer to zero means less important)
    pub weight: f64,

}



impl Default for JointConstraint {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__msg__JointConstraint__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__msg__JointConstraint__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for JointConstraint {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__JointConstraint__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__JointConstraint__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__JointConstraint__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for JointConstraint {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for JointConstraint where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/msg/JointConstraint";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__msg__JointConstraint() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__msg__JointLimits() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__msg__JointLimits__init(msg: *mut JointLimits) -> bool;
    fn moveit_msgs__msg__JointLimits__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<JointLimits>, size: usize) -> bool;
    fn moveit_msgs__msg__JointLimits__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<JointLimits>);
    fn moveit_msgs__msg__JointLimits__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<JointLimits>, out_seq: *mut rosidl_runtime_rs::Sequence<JointLimits>) -> bool;
}

// Corresponds to moveit_msgs__msg__JointLimits
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// This message contains information about limits of a particular joint (or control dimension)

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct JointLimits {

    // This member is not documented.
    #[allow(missing_docs)]
    pub joint_name: rosidl_runtime_rs::String,

    /// true if the joint has position limits
    pub has_position_limits: bool,

    /// min and max position limits
    pub min_position: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub max_position: f64,

    /// true if joint has velocity limits
    pub has_velocity_limits: bool,

    /// max velocity limit
    pub max_velocity: f64,

    /// min_velocity is assumed to be -max_velocity
    /// true if joint has acceleration limits
    pub has_acceleration_limits: bool,

    /// max acceleration limit
    pub max_acceleration: f64,

    /// min_acceleration is assumed to be -max_acceleration
    /// true if joint has jerk limits
    pub has_jerk_limits: bool,

    /// max jerk limit
    pub max_jerk: f64,

}



impl Default for JointLimits {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__msg__JointLimits__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__msg__JointLimits__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for JointLimits {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__JointLimits__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__JointLimits__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__JointLimits__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for JointLimits {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for JointLimits where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/msg/JointLimits";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__msg__JointLimits() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__msg__LinkPadding() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__msg__LinkPadding__init(msg: *mut LinkPadding) -> bool;
    fn moveit_msgs__msg__LinkPadding__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<LinkPadding>, size: usize) -> bool;
    fn moveit_msgs__msg__LinkPadding__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<LinkPadding>);
    fn moveit_msgs__msg__LinkPadding__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<LinkPadding>, out_seq: *mut rosidl_runtime_rs::Sequence<LinkPadding>) -> bool;
}

// Corresponds to moveit_msgs__msg__LinkPadding
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// name for the link

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LinkPadding {

    // This member is not documented.
    #[allow(missing_docs)]
    pub link_name: rosidl_runtime_rs::String,

    /// padding to apply to the link
    pub padding: f64,

}



impl Default for LinkPadding {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__msg__LinkPadding__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__msg__LinkPadding__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for LinkPadding {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__LinkPadding__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__LinkPadding__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__LinkPadding__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for LinkPadding {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for LinkPadding where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/msg/LinkPadding";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__msg__LinkPadding() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__msg__LinkScale() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__msg__LinkScale__init(msg: *mut LinkScale) -> bool;
    fn moveit_msgs__msg__LinkScale__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<LinkScale>, size: usize) -> bool;
    fn moveit_msgs__msg__LinkScale__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<LinkScale>);
    fn moveit_msgs__msg__LinkScale__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<LinkScale>, out_seq: *mut rosidl_runtime_rs::Sequence<LinkScale>) -> bool;
}

// Corresponds to moveit_msgs__msg__LinkScale
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// name for the link

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LinkScale {

    // This member is not documented.
    #[allow(missing_docs)]
    pub link_name: rosidl_runtime_rs::String,

    /// scaling to apply to the link
    pub scale: f64,

}



impl Default for LinkScale {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__msg__LinkScale__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__msg__LinkScale__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for LinkScale {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__LinkScale__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__LinkScale__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__LinkScale__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for LinkScale {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for LinkScale where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/msg/LinkScale";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__msg__LinkScale() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__msg__MotionPlanRequest() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__msg__MotionPlanRequest__init(msg: *mut MotionPlanRequest) -> bool;
    fn moveit_msgs__msg__MotionPlanRequest__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MotionPlanRequest>, size: usize) -> bool;
    fn moveit_msgs__msg__MotionPlanRequest__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MotionPlanRequest>);
    fn moveit_msgs__msg__MotionPlanRequest__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MotionPlanRequest>, out_seq: *mut rosidl_runtime_rs::Sequence<MotionPlanRequest>) -> bool;
}

// Corresponds to moveit_msgs__msg__MotionPlanRequest
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// This service contains the definition for a request to the motion
/// planner and the output it provides

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MotionPlanRequest {
    /// Parameters for the workspace that the planner should work inside
    pub workspace_parameters: super::super::msg::rmw::WorkspaceParameters,

    /// Starting state updates. If certain joints should be considered
    /// at positions other than the current ones, these positions should
    /// be set here
    pub start_state: super::super::msg::rmw::RobotState,

    /// The possible goal states for the model to plan for. Each element of
    /// the array defines a goal region. The goal is achieved
    /// if the constraints for a particular region are satisfied
    pub goal_constraints: rosidl_runtime_rs::Sequence<super::super::msg::rmw::Constraints>,

    /// No state at any point along the path in the produced motion plan will violate these constraints (this applies to all points, not just waypoints)
    pub path_constraints: super::super::msg::rmw::Constraints,

    /// The constraints the resulting trajectory must satisfy
    pub trajectory_constraints: super::super::msg::rmw::TrajectoryConstraints,

    /// A set of trajectories that may be used as reference or initial trajectories for (typically optimization-based) planners
    /// These trajectories do not override start_state or goal_constraints
    pub reference_trajectories: rosidl_runtime_rs::Sequence<super::super::msg::rmw::GenericTrajectory>,

    /// The name of the planning pipeline to use. If no name is specified,
    /// the configured planning pipeline will be used
    pub pipeline_id: rosidl_runtime_rs::String,

    /// The name of the planning algorithm to use. If no name is specified,
    /// the default planner of the planning pipeline will be used
    pub planner_id: rosidl_runtime_rs::String,

    /// The name of the group of joints on which this planner is operating
    pub group_name: rosidl_runtime_rs::String,

    /// The number of times this plan is to be computed. Shortest solution
    /// will be reported.
    pub num_planning_attempts: i32,

    /// The maximum amount of time the motion planner is allowed to plan for (in seconds)
    pub allowed_planning_time: f64,

    /// Scaling factors for optionally reducing the maximum joint velocities and
    /// accelerations.  Allowed values are in (0,1]. The maximum joint velocity and
    /// acceleration specified in the robot model are multiplied by their respective
    /// factors. If a factor is outside its valid range (importantly, this
    /// includes being set to 0.0), this factor is set to the default value of 1.0
    /// internally (i.e., maximum joint velocity or maximum joint acceleration).
    pub max_velocity_scaling_factor: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub max_acceleration_scaling_factor: f64,

    /// Maximum cartesian speed for the given link.
    /// If max_cartesian_speed <= 0 the trajectory is not modified.
    /// These fields require the following planning request adapter: default_planning_request_adapters/LimitMaxCartesianLinkSpeed
    pub cartesian_speed_limited_link: rosidl_runtime_rs::String,

    /// m/s
    pub max_cartesian_speed: f64,

    /// smoothness of trajectory for FREE command of Pilz planner
    /// Value should be between 0.01 and 1.0
    pub smoothness_level: f64,

}



impl Default for MotionPlanRequest {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__msg__MotionPlanRequest__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__msg__MotionPlanRequest__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MotionPlanRequest {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__MotionPlanRequest__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__MotionPlanRequest__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__MotionPlanRequest__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MotionPlanRequest {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MotionPlanRequest where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/msg/MotionPlanRequest";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__msg__MotionPlanRequest() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__msg__MotionPlanResponse() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__msg__MotionPlanResponse__init(msg: *mut MotionPlanResponse) -> bool;
    fn moveit_msgs__msg__MotionPlanResponse__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MotionPlanResponse>, size: usize) -> bool;
    fn moveit_msgs__msg__MotionPlanResponse__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MotionPlanResponse>);
    fn moveit_msgs__msg__MotionPlanResponse__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MotionPlanResponse>, out_seq: *mut rosidl_runtime_rs::Sequence<MotionPlanResponse>) -> bool;
}

// Corresponds to moveit_msgs__msg__MotionPlanResponse
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// The representation of a solution to a planning problem

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MotionPlanResponse {
    /// The corresponding robot state
    pub trajectory_start: super::super::msg::rmw::RobotState,

    /// The group used for planning (usually the same as in the request)
    pub group_name: rosidl_runtime_rs::String,

    /// A solution trajectory, if found
    pub trajectory: super::super::msg::rmw::RobotTrajectory,

    /// Planning time (seconds)
    pub planning_time: f64,

    /// Error code - encodes the overall reason for failure
    pub error_code: super::super::msg::rmw::MoveItErrorCodes,

}



impl Default for MotionPlanResponse {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__msg__MotionPlanResponse__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__msg__MotionPlanResponse__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MotionPlanResponse {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__MotionPlanResponse__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__MotionPlanResponse__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__MotionPlanResponse__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MotionPlanResponse {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MotionPlanResponse where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/msg/MotionPlanResponse";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__msg__MotionPlanResponse() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__msg__MotionPlanDetailedResponse() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__msg__MotionPlanDetailedResponse__init(msg: *mut MotionPlanDetailedResponse) -> bool;
    fn moveit_msgs__msg__MotionPlanDetailedResponse__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MotionPlanDetailedResponse>, size: usize) -> bool;
    fn moveit_msgs__msg__MotionPlanDetailedResponse__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MotionPlanDetailedResponse>);
    fn moveit_msgs__msg__MotionPlanDetailedResponse__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MotionPlanDetailedResponse>, out_seq: *mut rosidl_runtime_rs::Sequence<MotionPlanDetailedResponse>) -> bool;
}

// Corresponds to moveit_msgs__msg__MotionPlanDetailedResponse
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// The representation of a solution to a planning problem, including intermediate data

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MotionPlanDetailedResponse {
    /// The starting state considered for the robot solution path
    pub trajectory_start: super::super::msg::rmw::RobotState,

    /// The group used for planning (usually the same as in the request)
    pub group_name: rosidl_runtime_rs::String,

    /// Multiple solution paths are reported, each reflecting intermediate steps in the trajectory processing
    /// The list of reported trajectories
    pub trajectory: rosidl_runtime_rs::Sequence<super::super::msg::rmw::RobotTrajectory>,

    /// Description of the reported trajectories (name of processing step)
    pub description: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,

    /// The amount of time spent computing a particular step in motion plan computation
    pub processing_time: rosidl_runtime_rs::Sequence<f64>,

    /// Status at the end of this plan
    pub error_code: super::super::msg::rmw::MoveItErrorCodes,

}



impl Default for MotionPlanDetailedResponse {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__msg__MotionPlanDetailedResponse__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__msg__MotionPlanDetailedResponse__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MotionPlanDetailedResponse {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__MotionPlanDetailedResponse__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__MotionPlanDetailedResponse__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__MotionPlanDetailedResponse__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MotionPlanDetailedResponse {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MotionPlanDetailedResponse where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/msg/MotionPlanDetailedResponse";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__msg__MotionPlanDetailedResponse() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__msg__MotionSequenceItem() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__msg__MotionSequenceItem__init(msg: *mut MotionSequenceItem) -> bool;
    fn moveit_msgs__msg__MotionSequenceItem__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MotionSequenceItem>, size: usize) -> bool;
    fn moveit_msgs__msg__MotionSequenceItem__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MotionSequenceItem>);
    fn moveit_msgs__msg__MotionSequenceItem__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MotionSequenceItem>, out_seq: *mut rosidl_runtime_rs::Sequence<MotionSequenceItem>) -> bool;
}

// Corresponds to moveit_msgs__msg__MotionSequenceItem
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// The plan request for this item.
/// It is the planning request for this segment of the sequence, as if it were a solitary motion.

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MotionSequenceItem {

    // This member is not documented.
    #[allow(missing_docs)]
    pub req: super::super::msg::rmw::MotionPlanRequest,

    /// To blend between sequence items, the motion may be smoothed using a circular motion.
    /// The blend radius of the circle between this and the next command, where 0 means no blending.
    pub blend_radius: f64,

}



impl Default for MotionSequenceItem {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__msg__MotionSequenceItem__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__msg__MotionSequenceItem__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MotionSequenceItem {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__MotionSequenceItem__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__MotionSequenceItem__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__MotionSequenceItem__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MotionSequenceItem {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MotionSequenceItem where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/msg/MotionSequenceItem";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__msg__MotionSequenceItem() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__msg__MotionSequenceRequest() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__msg__MotionSequenceRequest__init(msg: *mut MotionSequenceRequest) -> bool;
    fn moveit_msgs__msg__MotionSequenceRequest__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MotionSequenceRequest>, size: usize) -> bool;
    fn moveit_msgs__msg__MotionSequenceRequest__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MotionSequenceRequest>);
    fn moveit_msgs__msg__MotionSequenceRequest__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MotionSequenceRequest>, out_seq: *mut rosidl_runtime_rs::Sequence<MotionSequenceRequest>) -> bool;
}

// Corresponds to moveit_msgs__msg__MotionSequenceRequest
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// List of motion planning request with a blend_radius for each.
/// In the response of the planner all of these will be executable as one sequence.

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MotionSequenceRequest {

    // This member is not documented.
    #[allow(missing_docs)]
    pub items: rosidl_runtime_rs::Sequence<super::super::msg::rmw::MotionSequenceItem>,

}



impl Default for MotionSequenceRequest {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__msg__MotionSequenceRequest__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__msg__MotionSequenceRequest__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MotionSequenceRequest {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__MotionSequenceRequest__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__MotionSequenceRequest__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__MotionSequenceRequest__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MotionSequenceRequest {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MotionSequenceRequest where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/msg/MotionSequenceRequest";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__msg__MotionSequenceRequest() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__msg__MotionSequenceResponse() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__msg__MotionSequenceResponse__init(msg: *mut MotionSequenceResponse) -> bool;
    fn moveit_msgs__msg__MotionSequenceResponse__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MotionSequenceResponse>, size: usize) -> bool;
    fn moveit_msgs__msg__MotionSequenceResponse__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MotionSequenceResponse>);
    fn moveit_msgs__msg__MotionSequenceResponse__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MotionSequenceResponse>, out_seq: *mut rosidl_runtime_rs::Sequence<MotionSequenceResponse>) -> bool;
}

// Corresponds to moveit_msgs__msg__MotionSequenceResponse
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// An error code reflecting what went wrong

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MotionSequenceResponse {

    // This member is not documented.
    #[allow(missing_docs)]
    pub error_code: super::super::msg::rmw::MoveItErrorCodes,

    /// The full starting state of the robot at the start of the sequence
    pub sequence_start: super::super::msg::rmw::RobotState,

    /// The trajectories that the planner produced for execution
    pub planned_trajectories: rosidl_runtime_rs::Sequence<super::super::msg::rmw::RobotTrajectory>,

    /// The amount of time it took to complete the motion plan
    pub planning_time: f64,

}



impl Default for MotionSequenceResponse {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__msg__MotionSequenceResponse__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__msg__MotionSequenceResponse__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MotionSequenceResponse {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__MotionSequenceResponse__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__MotionSequenceResponse__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__MotionSequenceResponse__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MotionSequenceResponse {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MotionSequenceResponse where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/msg/MotionSequenceResponse";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__msg__MotionSequenceResponse() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__msg__MoveItErrorCodes() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__msg__MoveItErrorCodes__init(msg: *mut MoveItErrorCodes) -> bool;
    fn moveit_msgs__msg__MoveItErrorCodes__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MoveItErrorCodes>, size: usize) -> bool;
    fn moveit_msgs__msg__MoveItErrorCodes__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MoveItErrorCodes>);
    fn moveit_msgs__msg__MoveItErrorCodes__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MoveItErrorCodes>, out_seq: *mut rosidl_runtime_rs::Sequence<MoveItErrorCodes>) -> bool;
}

// Corresponds to moveit_msgs__msg__MoveItErrorCodes
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MoveItErrorCodes {

    // This member is not documented.
    #[allow(missing_docs)]
    pub val: i32,

    /// A message to provide additional information.
    pub message: rosidl_runtime_rs::String,

    /// Name of the component that created the status.
    /// This is helpful to locate error source.
    pub source: rosidl_runtime_rs::String,

}

impl MoveItErrorCodes {
    /// overall behavior
    pub const SUCCESS: i32 = 1;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const UNDEFINED: i32 = 0;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const FAILURE: i32 = 99999;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const PLANNING_FAILED: i32 = -1;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const INVALID_MOTION_PLAN: i32 = -2;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const MOTION_PLAN_INVALIDATED_BY_ENVIRONMENT_CHANGE: i32 = -3;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const CONTROL_FAILED: i32 = -4;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const UNABLE_TO_AQUIRE_SENSOR_DATA: i32 = -5;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const TIMED_OUT: i32 = -6;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const PREEMPTED: i32 = -7;

    /// planning & kinematics request errors
    pub const START_STATE_IN_COLLISION: i32 = -10;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const START_STATE_VIOLATES_PATH_CONSTRAINTS: i32 = -11;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const START_STATE_INVALID: i32 = -26;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const GOAL_IN_COLLISION: i32 = -12;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const GOAL_VIOLATES_PATH_CONSTRAINTS: i32 = -13;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const GOAL_CONSTRAINTS_VIOLATED: i32 = -14;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const GOAL_STATE_INVALID: i32 = -27;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const UNRECOGNIZED_GOAL_TYPE: i32 = -28;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const INVALID_GROUP_NAME: i32 = -15;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const INVALID_GOAL_CONSTRAINTS: i32 = -16;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const INVALID_ROBOT_STATE: i32 = -17;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const INVALID_LINK_NAME: i32 = -18;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const INVALID_OBJECT_NAME: i32 = -19;

    /// system errors
    pub const FRAME_TRANSFORM_FAILURE: i32 = -21;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const COLLISION_CHECKING_UNAVAILABLE: i32 = -22;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const ROBOT_STATE_STALE: i32 = -23;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const SENSOR_INFO_STALE: i32 = -24;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const COMMUNICATION_FAILURE: i32 = -25;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const CRASH: i32 = -29;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const ABORT: i32 = -30;

    /// kinematics errors
    pub const NO_IK_SOLUTION: i32 = -31;

}


impl Default for MoveItErrorCodes {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__msg__MoveItErrorCodes__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__msg__MoveItErrorCodes__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MoveItErrorCodes {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__MoveItErrorCodes__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__MoveItErrorCodes__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__MoveItErrorCodes__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MoveItErrorCodes {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MoveItErrorCodes where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/msg/MoveItErrorCodes";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__msg__MoveItErrorCodes() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__msg__TrajectoryConstraints() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__msg__TrajectoryConstraints__init(msg: *mut TrajectoryConstraints) -> bool;
    fn moveit_msgs__msg__TrajectoryConstraints__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<TrajectoryConstraints>, size: usize) -> bool;
    fn moveit_msgs__msg__TrajectoryConstraints__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<TrajectoryConstraints>);
    fn moveit_msgs__msg__TrajectoryConstraints__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<TrajectoryConstraints>, out_seq: *mut rosidl_runtime_rs::Sequence<TrajectoryConstraints>) -> bool;
}

// Corresponds to moveit_msgs__msg__TrajectoryConstraints
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// The array of constraints to consider along the trajectory

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TrajectoryConstraints {

    // This member is not documented.
    #[allow(missing_docs)]
    pub constraints: rosidl_runtime_rs::Sequence<super::super::msg::rmw::Constraints>,

}



impl Default for TrajectoryConstraints {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__msg__TrajectoryConstraints__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__msg__TrajectoryConstraints__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for TrajectoryConstraints {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__TrajectoryConstraints__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__TrajectoryConstraints__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__TrajectoryConstraints__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for TrajectoryConstraints {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for TrajectoryConstraints where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/msg/TrajectoryConstraints";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__msg__TrajectoryConstraints() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__msg__ObjectColor() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__msg__ObjectColor__init(msg: *mut ObjectColor) -> bool;
    fn moveit_msgs__msg__ObjectColor__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ObjectColor>, size: usize) -> bool;
    fn moveit_msgs__msg__ObjectColor__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ObjectColor>);
    fn moveit_msgs__msg__ObjectColor__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ObjectColor>, out_seq: *mut rosidl_runtime_rs::Sequence<ObjectColor>) -> bool;
}

// Corresponds to moveit_msgs__msg__ObjectColor
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// The object id for which we specify color

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ObjectColor {

    // This member is not documented.
    #[allow(missing_docs)]
    pub id: rosidl_runtime_rs::String,

    /// The value of the color
    pub color: std_msgs::msg::rmw::ColorRGBA,

}



impl Default for ObjectColor {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__msg__ObjectColor__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__msg__ObjectColor__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ObjectColor {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__ObjectColor__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__ObjectColor__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__ObjectColor__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ObjectColor {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ObjectColor where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/msg/ObjectColor";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__msg__ObjectColor() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__msg__OrientationConstraint() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__msg__OrientationConstraint__init(msg: *mut OrientationConstraint) -> bool;
    fn moveit_msgs__msg__OrientationConstraint__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<OrientationConstraint>, size: usize) -> bool;
    fn moveit_msgs__msg__OrientationConstraint__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<OrientationConstraint>);
    fn moveit_msgs__msg__OrientationConstraint__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<OrientationConstraint>, out_seq: *mut rosidl_runtime_rs::Sequence<OrientationConstraint>) -> bool;
}

// Corresponds to moveit_msgs__msg__OrientationConstraint
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// This message contains the definition of an orientation constraint.

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct OrientationConstraint {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,

    /// The desired orientation of the robot link specified as a quaternion
    pub orientation: geometry_msgs::msg::rmw::Quaternion,

    /// The robot link this constraint refers to
    pub link_name: rosidl_runtime_rs::String,

    /// Tolerance on the three vector components of the orientation error (optional)
    /// This is a +/- tolerance i.e. (+ x_axis_tol, - x_axis_tol)
    pub absolute_x_axis_tolerance: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub absolute_y_axis_tolerance: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub absolute_z_axis_tolerance: f64,

    /// Defines how the orientation error is calculated
    /// The error is compared to the tolerance defined above
    pub parameterization: u8,

    /// A weighting factor for this constraint (denotes relative importance to other constraints. Closer to zero means less important)
    pub weight: f64,

}

impl OrientationConstraint {
    /// The different options for the orientation error parameterization
    /// - Intrinsic xyz Euler angles (default value)
    pub const XYZ_EULER_ANGLES: u8 = 0;

    /// - A rotation vector. This is similar to the angle-axis representation,
    /// but the magnitude of the vector represents the rotation angle.
    pub const ROTATION_VECTOR: u8 = 1;

}


impl Default for OrientationConstraint {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__msg__OrientationConstraint__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__msg__OrientationConstraint__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for OrientationConstraint {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__OrientationConstraint__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__OrientationConstraint__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__OrientationConstraint__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for OrientationConstraint {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for OrientationConstraint where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/msg/OrientationConstraint";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__msg__OrientationConstraint() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__msg__OrientedBoundingBox() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__msg__OrientedBoundingBox__init(msg: *mut OrientedBoundingBox) -> bool;
    fn moveit_msgs__msg__OrientedBoundingBox__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<OrientedBoundingBox>, size: usize) -> bool;
    fn moveit_msgs__msg__OrientedBoundingBox__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<OrientedBoundingBox>);
    fn moveit_msgs__msg__OrientedBoundingBox__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<OrientedBoundingBox>, out_seq: *mut rosidl_runtime_rs::Sequence<OrientedBoundingBox>) -> bool;
}

// Corresponds to moveit_msgs__msg__OrientedBoundingBox
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// the pose of the box

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct OrientedBoundingBox {

    // This member is not documented.
    #[allow(missing_docs)]
    pub pose: geometry_msgs::msg::rmw::Pose,

    /// the extents of the box, assuming the center is at the origin
    pub extents: geometry_msgs::msg::rmw::Point32,

}



impl Default for OrientedBoundingBox {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__msg__OrientedBoundingBox__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__msg__OrientedBoundingBox__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for OrientedBoundingBox {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__OrientedBoundingBox__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__OrientedBoundingBox__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__OrientedBoundingBox__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for OrientedBoundingBox {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for OrientedBoundingBox where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/msg/OrientedBoundingBox";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__msg__OrientedBoundingBox() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__msg__PlaceLocation() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__msg__PlaceLocation__init(msg: *mut PlaceLocation) -> bool;
    fn moveit_msgs__msg__PlaceLocation__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PlaceLocation>, size: usize) -> bool;
    fn moveit_msgs__msg__PlaceLocation__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PlaceLocation>);
    fn moveit_msgs__msg__PlaceLocation__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PlaceLocation>, out_seq: *mut rosidl_runtime_rs::Sequence<PlaceLocation>) -> bool;
}

// Corresponds to moveit_msgs__msg__PlaceLocation
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// A name for this grasp

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PlaceLocation {

    // This member is not documented.
    #[allow(missing_docs)]
    pub id: rosidl_runtime_rs::String,

    /// The internal posture of the hand for the grasp
    /// positions and efforts are used
    pub post_place_posture: trajectory_msgs::msg::rmw::JointTrajectory,

    /// The position of the end-effector for the grasp relative to a reference frame
    /// (that is always specified elsewhere, not in this message)
    pub place_pose: geometry_msgs::msg::rmw::PoseStamped,

    /// The estimated probability of success for this place, or some other
    /// measure of how "good" it is.
    pub quality: f64,

    /// The approach motion
    pub pre_place_approach: super::super::msg::rmw::GripperTranslation,

    /// The retreat motion
    pub post_place_retreat: super::super::msg::rmw::GripperTranslation,

    /// an optional list of obstacles that we have semantic information about
    /// and that can be touched/pushed/moved in the course of grasping
    pub allowed_touch_objects: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,

}



impl Default for PlaceLocation {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__msg__PlaceLocation__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__msg__PlaceLocation__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PlaceLocation {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__PlaceLocation__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__PlaceLocation__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__PlaceLocation__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PlaceLocation {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PlaceLocation where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/msg/PlaceLocation";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__msg__PlaceLocation() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__msg__PlannerInterfaceDescription() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__msg__PlannerInterfaceDescription__init(msg: *mut PlannerInterfaceDescription) -> bool;
    fn moveit_msgs__msg__PlannerInterfaceDescription__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PlannerInterfaceDescription>, size: usize) -> bool;
    fn moveit_msgs__msg__PlannerInterfaceDescription__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PlannerInterfaceDescription>);
    fn moveit_msgs__msg__PlannerInterfaceDescription__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PlannerInterfaceDescription>, out_seq: *mut rosidl_runtime_rs::Sequence<PlannerInterfaceDescription>) -> bool;
}

// Corresponds to moveit_msgs__msg__PlannerInterfaceDescription
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// The name of the planner interface

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PlannerInterfaceDescription {

    // This member is not documented.
    #[allow(missing_docs)]
    pub name: rosidl_runtime_rs::String,

    /// The name of the planning pipeline
    pub pipeline_id: rosidl_runtime_rs::String,

    /// The names of the planner ids within the interface
    pub planner_ids: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,

}



impl Default for PlannerInterfaceDescription {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__msg__PlannerInterfaceDescription__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__msg__PlannerInterfaceDescription__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PlannerInterfaceDescription {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__PlannerInterfaceDescription__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__PlannerInterfaceDescription__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__PlannerInterfaceDescription__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PlannerInterfaceDescription {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PlannerInterfaceDescription where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/msg/PlannerInterfaceDescription";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__msg__PlannerInterfaceDescription() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__msg__PlannerParams() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__msg__PlannerParams__init(msg: *mut PlannerParams) -> bool;
    fn moveit_msgs__msg__PlannerParams__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PlannerParams>, size: usize) -> bool;
    fn moveit_msgs__msg__PlannerParams__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PlannerParams>);
    fn moveit_msgs__msg__PlannerParams__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PlannerParams>, out_seq: *mut rosidl_runtime_rs::Sequence<PlannerParams>) -> bool;
}

// Corresponds to moveit_msgs__msg__PlannerParams
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// parameter names (same size as values)

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PlannerParams {

    // This member is not documented.
    #[allow(missing_docs)]
    pub keys: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,

    /// parameter values (same size as keys)
    pub values: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,

    /// parameter description (can be empty)
    pub descriptions: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,

}



impl Default for PlannerParams {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__msg__PlannerParams__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__msg__PlannerParams__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PlannerParams {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__PlannerParams__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__PlannerParams__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__PlannerParams__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PlannerParams {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PlannerParams where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/msg/PlannerParams";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__msg__PlannerParams() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__msg__PlanningScene() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__msg__PlanningScene__init(msg: *mut PlanningScene) -> bool;
    fn moveit_msgs__msg__PlanningScene__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PlanningScene>, size: usize) -> bool;
    fn moveit_msgs__msg__PlanningScene__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PlanningScene>);
    fn moveit_msgs__msg__PlanningScene__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PlanningScene>, out_seq: *mut rosidl_runtime_rs::Sequence<PlanningScene>) -> bool;
}

// Corresponds to moveit_msgs__msg__PlanningScene
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// name of planning scene

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PlanningScene {

    // This member is not documented.
    #[allow(missing_docs)]
    pub name: rosidl_runtime_rs::String,

    /// full robot state
    pub robot_state: super::super::msg::rmw::RobotState,

    /// The name of the robot model this scene is for
    pub robot_model_name: rosidl_runtime_rs::String,

    /// additional frames for duplicating tf (with respect to the planning frame)
    pub fixed_frame_transforms: rosidl_runtime_rs::Sequence<geometry_msgs::msg::rmw::TransformStamped>,

    /// full allowed collision matrix
    pub allowed_collision_matrix: super::super::msg::rmw::AllowedCollisionMatrix,

    /// all link paddings
    pub link_padding: rosidl_runtime_rs::Sequence<super::super::msg::rmw::LinkPadding>,

    /// all link scales
    pub link_scale: rosidl_runtime_rs::Sequence<super::super::msg::rmw::LinkScale>,

    /// Attached objects, collision objects, even the octomap or collision map can have
    /// colors associated to them. This array specifies them.
    pub object_colors: rosidl_runtime_rs::Sequence<super::super::msg::rmw::ObjectColor>,

    /// the collision map
    pub world: super::super::msg::rmw::PlanningSceneWorld,

    /// Flag indicating whether this scene is to be interpreted as a diff with respect to some other scene
    pub is_diff: bool,

}



impl Default for PlanningScene {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__msg__PlanningScene__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__msg__PlanningScene__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PlanningScene {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__PlanningScene__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__PlanningScene__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__PlanningScene__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PlanningScene {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PlanningScene where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/msg/PlanningScene";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__msg__PlanningScene() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__msg__PlanningSceneComponents() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__msg__PlanningSceneComponents__init(msg: *mut PlanningSceneComponents) -> bool;
    fn moveit_msgs__msg__PlanningSceneComponents__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PlanningSceneComponents>, size: usize) -> bool;
    fn moveit_msgs__msg__PlanningSceneComponents__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PlanningSceneComponents>);
    fn moveit_msgs__msg__PlanningSceneComponents__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PlanningSceneComponents>, out_seq: *mut rosidl_runtime_rs::Sequence<PlanningSceneComponents>) -> bool;
}

// Corresponds to moveit_msgs__msg__PlanningSceneComponents
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// This message defines the components that make up the PlanningScene message.
/// The values can be used as a bitfield to specify which parts of the PlanningScene message
/// are of interest

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PlanningSceneComponents {
    /// Bitfield combining options indicated above
    pub components: u32,

}

impl PlanningSceneComponents {
    /// Scene name, model name, model root
    pub const SCENE_SETTINGS: u32 = 1;

    /// Joint values of the robot state
    pub const ROBOT_STATE: u32 = 2;

    /// Attached objects (including geometry) for the robot state
    pub const ROBOT_STATE_ATTACHED_OBJECTS: u32 = 4;

    /// The names of the world objects
    pub const WORLD_OBJECT_NAMES: u32 = 8;

    /// The geometry of the world objects
    pub const WORLD_OBJECT_GEOMETRY: u32 = 16;

    /// The maintained octomap
    pub const OCTOMAP: u32 = 32;

    /// The maintained list of transforms
    pub const TRANSFORMS: u32 = 64;

    /// The allowed collision matrix
    pub const ALLOWED_COLLISION_MATRIX: u32 = 128;

    /// The default link padding and link scaling
    pub const LINK_PADDING_AND_SCALING: u32 = 256;

    /// The stored object colors
    pub const OBJECT_COLORS: u32 = 512;

}


impl Default for PlanningSceneComponents {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__msg__PlanningSceneComponents__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__msg__PlanningSceneComponents__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PlanningSceneComponents {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__PlanningSceneComponents__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__PlanningSceneComponents__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__PlanningSceneComponents__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PlanningSceneComponents {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PlanningSceneComponents where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/msg/PlanningSceneComponents";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__msg__PlanningSceneComponents() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__msg__PlanningSceneWorld() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__msg__PlanningSceneWorld__init(msg: *mut PlanningSceneWorld) -> bool;
    fn moveit_msgs__msg__PlanningSceneWorld__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PlanningSceneWorld>, size: usize) -> bool;
    fn moveit_msgs__msg__PlanningSceneWorld__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PlanningSceneWorld>);
    fn moveit_msgs__msg__PlanningSceneWorld__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PlanningSceneWorld>, out_seq: *mut rosidl_runtime_rs::Sequence<PlanningSceneWorld>) -> bool;
}

// Corresponds to moveit_msgs__msg__PlanningSceneWorld
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// collision objects

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PlanningSceneWorld {

    // This member is not documented.
    #[allow(missing_docs)]
    pub collision_objects: rosidl_runtime_rs::Sequence<super::super::msg::rmw::CollisionObject>,

    /// The octomap that represents additional collision data
    pub octomap: octomap_msgs::msg::rmw::OctomapWithPose,

}



impl Default for PlanningSceneWorld {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__msg__PlanningSceneWorld__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__msg__PlanningSceneWorld__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PlanningSceneWorld {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__PlanningSceneWorld__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__PlanningSceneWorld__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__PlanningSceneWorld__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PlanningSceneWorld {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PlanningSceneWorld where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/msg/PlanningSceneWorld";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__msg__PlanningSceneWorld() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__msg__PlanningOptions() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__msg__PlanningOptions__init(msg: *mut PlanningOptions) -> bool;
    fn moveit_msgs__msg__PlanningOptions__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PlanningOptions>, size: usize) -> bool;
    fn moveit_msgs__msg__PlanningOptions__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PlanningOptions>);
    fn moveit_msgs__msg__PlanningOptions__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PlanningOptions>, out_seq: *mut rosidl_runtime_rs::Sequence<PlanningOptions>) -> bool;
}

// Corresponds to moveit_msgs__msg__PlanningOptions
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PlanningOptions {
    /// The diff to consider for the planning scene (optional)
    pub planning_scene_diff: super::super::msg::rmw::PlanningScene,

    /// If this flag is set to true, the action
    /// returns an executable plan in the response but does not attempt execution
    pub plan_only: bool,

    /// If this flag is set to true, the action of planning &
    /// executing is allowed to look around  (move sensors) if
    /// it seems that not enough information is available about
    /// the environment
    pub look_around: bool,

    /// If this value is positive, the action of planning & executing
    /// is allowed to look around for a maximum number of attempts;
    /// If the value is left as 0, the default value is used, as set
    /// with dynamic_reconfigure
    pub look_around_attempts: i32,

    /// If set and if look_around is true, this value is used as
    /// the maximum cost allowed for a path to be considered executable.
    /// If the cost of a path is higher than this value, more sensing or
    /// a new plan needed. If left as 0.0 but look_around is true, then
    /// the default value set via dynamic_reconfigure is used
    pub max_safe_execution_cost: f64,

    /// If the plan becomes invalidated during execution, it is possible to have
    /// that plan recomputed and execution restarted. This flag enables this
    /// functionality
    pub replan: bool,

    /// The maximum number of replanning attempts
    pub replan_attempts: i32,

    /// The amount of time to wait in between replanning attempts (in seconds)
    pub replan_delay: f64,

}



impl Default for PlanningOptions {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__msg__PlanningOptions__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__msg__PlanningOptions__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PlanningOptions {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__PlanningOptions__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__PlanningOptions__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__PlanningOptions__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PlanningOptions {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PlanningOptions where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/msg/PlanningOptions";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__msg__PlanningOptions() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__msg__PositionConstraint() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__msg__PositionConstraint__init(msg: *mut PositionConstraint) -> bool;
    fn moveit_msgs__msg__PositionConstraint__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PositionConstraint>, size: usize) -> bool;
    fn moveit_msgs__msg__PositionConstraint__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PositionConstraint>);
    fn moveit_msgs__msg__PositionConstraint__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PositionConstraint>, out_seq: *mut rosidl_runtime_rs::Sequence<PositionConstraint>) -> bool;
}

// Corresponds to moveit_msgs__msg__PositionConstraint
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// This message contains the definition of a position constraint.

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PositionConstraint {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,

    /// The robot link this constraint refers to
    pub link_name: rosidl_runtime_rs::String,

    /// The offset (in the link frame) for the target point on the link we are planning for
    pub target_point_offset: geometry_msgs::msg::rmw::Vector3,

    /// The volume this constraint refers to
    pub constraint_region: super::super::msg::rmw::BoundingVolume,

    /// A weighting factor for this constraint (denotes relative importance to other constraints. Closer to zero means less important)
    pub weight: f64,

}



impl Default for PositionConstraint {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__msg__PositionConstraint__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__msg__PositionConstraint__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PositionConstraint {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__PositionConstraint__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__PositionConstraint__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__PositionConstraint__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PositionConstraint {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PositionConstraint where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/msg/PositionConstraint";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__msg__PositionConstraint() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__msg__RobotState() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__msg__RobotState__init(msg: *mut RobotState) -> bool;
    fn moveit_msgs__msg__RobotState__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<RobotState>, size: usize) -> bool;
    fn moveit_msgs__msg__RobotState__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<RobotState>);
    fn moveit_msgs__msg__RobotState__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<RobotState>, out_seq: *mut rosidl_runtime_rs::Sequence<RobotState>) -> bool;
}

// Corresponds to moveit_msgs__msg__RobotState
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// This message contains information about the robot state, i.e. the positions of its joints and links

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RobotState {

    // This member is not documented.
    #[allow(missing_docs)]
    pub joint_state: sensor_msgs::msg::rmw::JointState,

    /// Joints that may have multiple DOF are specified here
    pub multi_dof_joint_state: sensor_msgs::msg::rmw::MultiDOFJointState,

    /// Attached collision objects (attached to some link on the robot)
    pub attached_collision_objects: rosidl_runtime_rs::Sequence<super::super::msg::rmw::AttachedCollisionObject>,

    /// Flag indicating whether this scene is to be interpreted as a diff with respect to some other scene
    /// This is mostly important for handling the attached bodies (whether or not to clear the attached bodies
    /// of a moveit::core::RobotState before updating it with this message)
    pub is_diff: bool,

}



impl Default for RobotState {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__msg__RobotState__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__msg__RobotState__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for RobotState {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__RobotState__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__RobotState__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__RobotState__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for RobotState {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for RobotState where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/msg/RobotState";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__msg__RobotState() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__msg__RobotTrajectory() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__msg__RobotTrajectory__init(msg: *mut RobotTrajectory) -> bool;
    fn moveit_msgs__msg__RobotTrajectory__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<RobotTrajectory>, size: usize) -> bool;
    fn moveit_msgs__msg__RobotTrajectory__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<RobotTrajectory>);
    fn moveit_msgs__msg__RobotTrajectory__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<RobotTrajectory>, out_seq: *mut rosidl_runtime_rs::Sequence<RobotTrajectory>) -> bool;
}

// Corresponds to moveit_msgs__msg__RobotTrajectory
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RobotTrajectory {

    // This member is not documented.
    #[allow(missing_docs)]
    pub joint_trajectory: trajectory_msgs::msg::rmw::JointTrajectory,


    // This member is not documented.
    #[allow(missing_docs)]
    pub multi_dof_joint_trajectory: trajectory_msgs::msg::rmw::MultiDOFJointTrajectory,

}



impl Default for RobotTrajectory {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__msg__RobotTrajectory__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__msg__RobotTrajectory__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for RobotTrajectory {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__RobotTrajectory__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__RobotTrajectory__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__RobotTrajectory__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for RobotTrajectory {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for RobotTrajectory where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/msg/RobotTrajectory";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__msg__RobotTrajectory() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__msg__VisibilityConstraint() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__msg__VisibilityConstraint__init(msg: *mut VisibilityConstraint) -> bool;
    fn moveit_msgs__msg__VisibilityConstraint__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<VisibilityConstraint>, size: usize) -> bool;
    fn moveit_msgs__msg__VisibilityConstraint__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<VisibilityConstraint>);
    fn moveit_msgs__msg__VisibilityConstraint__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<VisibilityConstraint>, out_seq: *mut rosidl_runtime_rs::Sequence<VisibilityConstraint>) -> bool;
}

// Corresponds to moveit_msgs__msg__VisibilityConstraint
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// The constraint is useful to maintain visibility to a disc (the target) in a particular frame.
/// This disc forms the base of a visibiliy cone whose tip is at the origin of the sensor.
/// Maintaining visibility is done by ensuring the robot does not obstruct the visibility cone.
/// Note:
/// This constraint does NOT enforce minimum or maximum distances between the sensor
/// and the target, nor does it enforce the target to be in the field of view of
/// the sensor. A PositionConstraint can (and probably should) be used for such purposes.

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct VisibilityConstraint {
    /// The radius of the disc that should be maintained visible
    pub target_radius: f64,

    /// The pose of the disc; as the robot moves, the pose of the disc may change as well
    /// This can be in the frame of a particular robot link, for example
    pub target_pose: geometry_msgs::msg::rmw::PoseStamped,

    /// From the sensor origin towards the target, the disc forms a visibility cone
    /// This cone is approximated using many sides. For example, when using 4 sides,
    /// that in fact makes the visibility region be a pyramid.
    /// This value should always be 3 or more.
    pub cone_sides: i32,

    /// The pose in which visibility is to be maintained.
    /// The frame id should represent the robot link to which the sensor is attached.
    /// It is assumed the sensor can look directly at the target, in any direction.
    /// This assumption is usually not true, but additional PositionConstraints
    /// can resolve this issue.
    pub sensor_pose: geometry_msgs::msg::rmw::PoseStamped,

    /// Even though the disc is maintained visible, the visibility cone can be very small
    /// because of the orientation of the disc with respect to the sensor. It is possible
    /// for example to view the disk almost from a side, in which case the visibility cone
    /// can end up having close to 0 volume. The view angle is defined to be the angle between
    /// the normal to the visibility disc and the direction vector from the sensor origin.
    /// The value below represents the minimum desired view angle. For a perfect view,
    /// this value will be 0 (the two vectors are exact opposites). For a completely obstructed view
    /// this value will be Pi/2 (the vectors are perpendicular). This value defined below
    /// is the maximum view angle to be maintained. This should be a value in the open interval
    /// (0, Pi/2). If 0 is set, the view angle is NOT enforced.
    pub max_view_angle: f64,

    /// This angle is used similarly to max_view_angle but limits the maximum angle
    /// between the sensor origin direction vector and the axis that connects the
    /// sensor origin to the target frame origin. The value is again in the range (0, Pi/2)
    /// and is NOT enforced if set to 0.
    pub max_range_angle: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub sensor_view_direction: u8,

    /// A weighting factor for this constraint (denotes relative importance to other constraints. Closer to zero means less important)
    pub weight: f64,

}

impl VisibilityConstraint {
    /// The axis that is assumed to indicate the direction of view for the sensor
    /// X = 2, Y = 1, Z = 0
    pub const SENSOR_Z: u8 = 0;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const SENSOR_Y: u8 = 1;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const SENSOR_X: u8 = 2;

}


impl Default for VisibilityConstraint {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__msg__VisibilityConstraint__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__msg__VisibilityConstraint__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for VisibilityConstraint {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__VisibilityConstraint__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__VisibilityConstraint__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__VisibilityConstraint__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for VisibilityConstraint {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for VisibilityConstraint where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/msg/VisibilityConstraint";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__msg__VisibilityConstraint() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__msg__WorkspaceParameters() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__msg__WorkspaceParameters__init(msg: *mut WorkspaceParameters) -> bool;
    fn moveit_msgs__msg__WorkspaceParameters__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<WorkspaceParameters>, size: usize) -> bool;
    fn moveit_msgs__msg__WorkspaceParameters__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<WorkspaceParameters>);
    fn moveit_msgs__msg__WorkspaceParameters__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<WorkspaceParameters>, out_seq: *mut rosidl_runtime_rs::Sequence<WorkspaceParameters>) -> bool;
}

// Corresponds to moveit_msgs__msg__WorkspaceParameters
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// This message contains a set of parameters useful in
/// setting up the volume (a box) in which the robot is allowed to move.
/// This is useful only when planning for mobile parts of
/// the robot as well.

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct WorkspaceParameters {
    /// Define the frame of reference for the box corners
    pub header: std_msgs::msg::rmw::Header,

    /// The minumum corner of the box, with respect to the robot starting pose
    pub min_corner: geometry_msgs::msg::rmw::Vector3,

    /// The maximum corner of the box, with respect to the robot starting pose
    pub max_corner: geometry_msgs::msg::rmw::Vector3,

}



impl Default for WorkspaceParameters {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__msg__WorkspaceParameters__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__msg__WorkspaceParameters__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for WorkspaceParameters {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__WorkspaceParameters__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__WorkspaceParameters__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__WorkspaceParameters__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for WorkspaceParameters {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for WorkspaceParameters where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/msg/WorkspaceParameters";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__msg__WorkspaceParameters() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__msg__KinematicSolverInfo() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__msg__KinematicSolverInfo__init(msg: *mut KinematicSolverInfo) -> bool;
    fn moveit_msgs__msg__KinematicSolverInfo__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<KinematicSolverInfo>, size: usize) -> bool;
    fn moveit_msgs__msg__KinematicSolverInfo__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<KinematicSolverInfo>);
    fn moveit_msgs__msg__KinematicSolverInfo__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<KinematicSolverInfo>, out_seq: *mut rosidl_runtime_rs::Sequence<KinematicSolverInfo>) -> bool;
}

// Corresponds to moveit_msgs__msg__KinematicSolverInfo
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// A list of joints in the kinematic tree

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct KinematicSolverInfo {

    // This member is not documented.
    #[allow(missing_docs)]
    pub joint_names: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,

    /// A list of joint limits corresponding to the joint names
    pub limits: rosidl_runtime_rs::Sequence<super::super::msg::rmw::JointLimits>,

    /// A list of links that the kinematics node provides solutions for
    pub link_names: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,

}



impl Default for KinematicSolverInfo {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__msg__KinematicSolverInfo__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__msg__KinematicSolverInfo__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for KinematicSolverInfo {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__KinematicSolverInfo__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__KinematicSolverInfo__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__KinematicSolverInfo__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for KinematicSolverInfo {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for KinematicSolverInfo where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/msg/KinematicSolverInfo";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__msg__KinematicSolverInfo() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__msg__PositionIKRequest() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__msg__PositionIKRequest__init(msg: *mut PositionIKRequest) -> bool;
    fn moveit_msgs__msg__PositionIKRequest__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PositionIKRequest>, size: usize) -> bool;
    fn moveit_msgs__msg__PositionIKRequest__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PositionIKRequest>);
    fn moveit_msgs__msg__PositionIKRequest__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PositionIKRequest>, out_seq: *mut rosidl_runtime_rs::Sequence<PositionIKRequest>) -> bool;
}

// Corresponds to moveit_msgs__msg__PositionIKRequest
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// A Position IK request message

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PositionIKRequest {
    /// The name of the group which will be used to compute IK
    /// e.g. "right_arm", or "arms" - see IK specification for multiple-groups below
    /// Information from the SRDF will be used to automatically determine which link
    /// to solve IK for, unless ik_link_name is also specified
    pub group_name: rosidl_runtime_rs::String,

    /// A RobotState consisting of hint/seed positions for the IK computation and positions
    /// for all the other joints in the robot. Additional state information provided here is
    /// used to specify starting positions for other joints/links on the robot.
    /// This state MUST contain state for all joints to be used by the IK solver
    /// to compute IK. The list of joints that the IK solver deals with can be
    /// found using the SRDF for the corresponding group
    pub robot_state: super::super::msg::rmw::RobotState,

    /// A set of constraints that the IK must obey; by default, this set of constraints is empty
    pub constraints: super::super::msg::rmw::Constraints,

    /// Find an IK solution that avoids collisions. By default, this is false
    pub avoid_collisions: bool,

    /// (OPTIONAL) The name of the link for which we are computing IK
    /// If not specified, the link name will be inferred from a combination
    /// of the group name and the SRDF. If any values are specified for ik_link_names,
    /// this value is ignored
    pub ik_link_name: rosidl_runtime_rs::String,

    /// The stamped pose of the link, when the IK solver computes the joint values
    /// for all the joints in a group. This value is ignored if pose_stamped_vector
    /// has any elements specified.
    pub pose_stamped: geometry_msgs::msg::rmw::PoseStamped,

    /// Multi-group parameters
    /// (OPTIONAL) The names of the links for which we are computing IK
    /// If not specified, the link name will be inferred from a combination
    /// of the group name and the SRDF
    pub ik_link_names: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,

    /// (OPTIONAL) The (stamped) poses of the links we are computing IK for (when a group has more than one end effector)
    /// e.g. The "arms" group might consist of both the "right_arm" and the "left_arm"
    /// The order of the groups referred to is the same as the order setup in the SRDF
    pub pose_stamped_vector: rosidl_runtime_rs::Sequence<geometry_msgs::msg::rmw::PoseStamped>,

    /// Maximum allowed time for IK calculation
    pub timeout: builtin_interfaces::msg::rmw::Duration,

}



impl Default for PositionIKRequest {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__msg__PositionIKRequest__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__msg__PositionIKRequest__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PositionIKRequest {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__PositionIKRequest__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__PositionIKRequest__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__PositionIKRequest__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PositionIKRequest {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PositionIKRequest where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/msg/PositionIKRequest";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__msg__PositionIKRequest() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__msg__PipelineState() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__msg__PipelineState__init(msg: *mut PipelineState) -> bool;
    fn moveit_msgs__msg__PipelineState__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PipelineState>, size: usize) -> bool;
    fn moveit_msgs__msg__PipelineState__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PipelineState>);
    fn moveit_msgs__msg__PipelineState__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PipelineState>, out_seq: *mut rosidl_runtime_rs::Sequence<PipelineState>) -> bool;
}

// Corresponds to moveit_msgs__msg__PipelineState
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Motion plan response that is processed

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PipelineState {

    // This member is not documented.
    #[allow(missing_docs)]
    pub request: super::super::msg::rmw::MotionPlanRequest,

    /// Current solution
    pub response: super::super::msg::rmw::MotionPlanResponse,

    /// Latest stage of the planning pipeline that got invoked
    pub pipeline_stage: rosidl_runtime_rs::String,

}



impl Default for PipelineState {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__msg__PipelineState__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__msg__PipelineState__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PipelineState {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__PipelineState__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__PipelineState__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__PipelineState__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PipelineState {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PipelineState where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/msg/PipelineState";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__msg__PipelineState() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__msg__ServoStatus() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__msg__ServoStatus__init(msg: *mut ServoStatus) -> bool;
    fn moveit_msgs__msg__ServoStatus__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ServoStatus>, size: usize) -> bool;
    fn moveit_msgs__msg__ServoStatus__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ServoStatus>);
    fn moveit_msgs__msg__ServoStatus__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ServoStatus>, out_seq: *mut rosidl_runtime_rs::Sequence<ServoStatus>) -> bool;
}

// Corresponds to moveit_msgs__msg__ServoStatus
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Message

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ServoStatus {
    /// will contains integer code
    pub code: i8,

    /// will contain explanatory message
    pub message: rosidl_runtime_rs::String,

}

impl ServoStatus {
    /// Status types (should reflect StatusCode from moveit_servo/utils/datatype.hpp)
    pub const INVALID: i8 = -1;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const NO_WARNING: i8 = 0;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const DECELERATE_FOR_APPROACHING_SINGULARITY: i8 = 1;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const HALT_FOR_SINGULARITY: i8 = 2;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const DECELERATE_FOR_LEAVING_SINGULARITY: i8 = 3;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const DECELERATE_FOR_COLLISION: i8 = 4;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const HALT_FOR_COLLISION: i8 = 5;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const JOINT_BOUND: i8 = 6;

}


impl Default for ServoStatus {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__msg__ServoStatus__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__msg__ServoStatus__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ServoStatus {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__ServoStatus__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__ServoStatus__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__msg__ServoStatus__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ServoStatus {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ServoStatus where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/msg/ServoStatus";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__msg__ServoStatus() }
  }
}


