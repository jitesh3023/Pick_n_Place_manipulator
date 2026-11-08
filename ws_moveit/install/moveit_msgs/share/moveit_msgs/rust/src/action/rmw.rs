
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__ExecuteTrajectory_Goal() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__action__ExecuteTrajectory_Goal__init(msg: *mut ExecuteTrajectory_Goal) -> bool;
    fn moveit_msgs__action__ExecuteTrajectory_Goal__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ExecuteTrajectory_Goal>, size: usize) -> bool;
    fn moveit_msgs__action__ExecuteTrajectory_Goal__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ExecuteTrajectory_Goal>);
    fn moveit_msgs__action__ExecuteTrajectory_Goal__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ExecuteTrajectory_Goal>, out_seq: *mut rosidl_runtime_rs::Sequence<ExecuteTrajectory_Goal>) -> bool;
}

// Corresponds to moveit_msgs__action__ExecuteTrajectory_Goal
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ExecuteTrajectory_Goal {

    // This member is not documented.
    #[allow(missing_docs)]
    pub trajectory: super::super::msg::rmw::RobotTrajectory,

    /// The controller names to use for execution
    pub controller_names: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,

}



impl Default for ExecuteTrajectory_Goal {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__action__ExecuteTrajectory_Goal__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__action__ExecuteTrajectory_Goal__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ExecuteTrajectory_Goal {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__ExecuteTrajectory_Goal__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__ExecuteTrajectory_Goal__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__ExecuteTrajectory_Goal__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ExecuteTrajectory_Goal {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ExecuteTrajectory_Goal where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/action/ExecuteTrajectory_Goal";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__ExecuteTrajectory_Goal() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__ExecuteTrajectory_Result() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__action__ExecuteTrajectory_Result__init(msg: *mut ExecuteTrajectory_Result) -> bool;
    fn moveit_msgs__action__ExecuteTrajectory_Result__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ExecuteTrajectory_Result>, size: usize) -> bool;
    fn moveit_msgs__action__ExecuteTrajectory_Result__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ExecuteTrajectory_Result>);
    fn moveit_msgs__action__ExecuteTrajectory_Result__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ExecuteTrajectory_Result>, out_seq: *mut rosidl_runtime_rs::Sequence<ExecuteTrajectory_Result>) -> bool;
}

// Corresponds to moveit_msgs__action__ExecuteTrajectory_Result
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ExecuteTrajectory_Result {
    /// Error code - encodes the overall reason for failure
    pub error_code: super::super::msg::rmw::MoveItErrorCodes,

}



impl Default for ExecuteTrajectory_Result {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__action__ExecuteTrajectory_Result__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__action__ExecuteTrajectory_Result__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ExecuteTrajectory_Result {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__ExecuteTrajectory_Result__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__ExecuteTrajectory_Result__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__ExecuteTrajectory_Result__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ExecuteTrajectory_Result {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ExecuteTrajectory_Result where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/action/ExecuteTrajectory_Result";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__ExecuteTrajectory_Result() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__ExecuteTrajectory_Feedback() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__action__ExecuteTrajectory_Feedback__init(msg: *mut ExecuteTrajectory_Feedback) -> bool;
    fn moveit_msgs__action__ExecuteTrajectory_Feedback__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ExecuteTrajectory_Feedback>, size: usize) -> bool;
    fn moveit_msgs__action__ExecuteTrajectory_Feedback__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ExecuteTrajectory_Feedback>);
    fn moveit_msgs__action__ExecuteTrajectory_Feedback__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ExecuteTrajectory_Feedback>, out_seq: *mut rosidl_runtime_rs::Sequence<ExecuteTrajectory_Feedback>) -> bool;
}

// Corresponds to moveit_msgs__action__ExecuteTrajectory_Feedback
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ExecuteTrajectory_Feedback {
    /// The internal state that the move group action currently is in
    pub state: rosidl_runtime_rs::String,

}



impl Default for ExecuteTrajectory_Feedback {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__action__ExecuteTrajectory_Feedback__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__action__ExecuteTrajectory_Feedback__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ExecuteTrajectory_Feedback {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__ExecuteTrajectory_Feedback__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__ExecuteTrajectory_Feedback__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__ExecuteTrajectory_Feedback__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ExecuteTrajectory_Feedback {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ExecuteTrajectory_Feedback where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/action/ExecuteTrajectory_Feedback";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__ExecuteTrajectory_Feedback() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__ExecuteTrajectory_FeedbackMessage() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__action__ExecuteTrajectory_FeedbackMessage__init(msg: *mut ExecuteTrajectory_FeedbackMessage) -> bool;
    fn moveit_msgs__action__ExecuteTrajectory_FeedbackMessage__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ExecuteTrajectory_FeedbackMessage>, size: usize) -> bool;
    fn moveit_msgs__action__ExecuteTrajectory_FeedbackMessage__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ExecuteTrajectory_FeedbackMessage>);
    fn moveit_msgs__action__ExecuteTrajectory_FeedbackMessage__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ExecuteTrajectory_FeedbackMessage>, out_seq: *mut rosidl_runtime_rs::Sequence<ExecuteTrajectory_FeedbackMessage>) -> bool;
}

// Corresponds to moveit_msgs__action__ExecuteTrajectory_FeedbackMessage
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ExecuteTrajectory_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::super::action::rmw::ExecuteTrajectory_Feedback,

}



impl Default for ExecuteTrajectory_FeedbackMessage {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__action__ExecuteTrajectory_FeedbackMessage__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__action__ExecuteTrajectory_FeedbackMessage__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ExecuteTrajectory_FeedbackMessage {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__ExecuteTrajectory_FeedbackMessage__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__ExecuteTrajectory_FeedbackMessage__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__ExecuteTrajectory_FeedbackMessage__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ExecuteTrajectory_FeedbackMessage {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ExecuteTrajectory_FeedbackMessage where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/action/ExecuteTrajectory_FeedbackMessage";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__ExecuteTrajectory_FeedbackMessage() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__MoveGroup_Goal() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__action__MoveGroup_Goal__init(msg: *mut MoveGroup_Goal) -> bool;
    fn moveit_msgs__action__MoveGroup_Goal__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MoveGroup_Goal>, size: usize) -> bool;
    fn moveit_msgs__action__MoveGroup_Goal__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MoveGroup_Goal>);
    fn moveit_msgs__action__MoveGroup_Goal__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MoveGroup_Goal>, out_seq: *mut rosidl_runtime_rs::Sequence<MoveGroup_Goal>) -> bool;
}

// Corresponds to moveit_msgs__action__MoveGroup_Goal
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MoveGroup_Goal {

    // This member is not documented.
    #[allow(missing_docs)]
    pub request: super::super::msg::rmw::MotionPlanRequest,

    /// Planning options
    pub planning_options: super::super::msg::rmw::PlanningOptions,

}



impl Default for MoveGroup_Goal {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__action__MoveGroup_Goal__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__action__MoveGroup_Goal__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MoveGroup_Goal {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__MoveGroup_Goal__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__MoveGroup_Goal__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__MoveGroup_Goal__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MoveGroup_Goal {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MoveGroup_Goal where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/action/MoveGroup_Goal";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__MoveGroup_Goal() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__MoveGroup_Result() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__action__MoveGroup_Result__init(msg: *mut MoveGroup_Result) -> bool;
    fn moveit_msgs__action__MoveGroup_Result__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MoveGroup_Result>, size: usize) -> bool;
    fn moveit_msgs__action__MoveGroup_Result__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MoveGroup_Result>);
    fn moveit_msgs__action__MoveGroup_Result__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MoveGroup_Result>, out_seq: *mut rosidl_runtime_rs::Sequence<MoveGroup_Result>) -> bool;
}

// Corresponds to moveit_msgs__action__MoveGroup_Result
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MoveGroup_Result {
    /// An error code reflecting what went wrong
    pub error_code: super::super::msg::rmw::MoveItErrorCodes,

    /// The full starting state of the robot at the start of the trajectory
    pub trajectory_start: super::super::msg::rmw::RobotState,

    /// The trajectory that moved group produced for execution
    pub planned_trajectory: super::super::msg::rmw::RobotTrajectory,

    /// The trace of the trajectory recorded during execution
    pub executed_trajectory: super::super::msg::rmw::RobotTrajectory,

    /// The amount of time it took to complete the motion plan
    pub planning_time: f64,

}



impl Default for MoveGroup_Result {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__action__MoveGroup_Result__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__action__MoveGroup_Result__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MoveGroup_Result {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__MoveGroup_Result__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__MoveGroup_Result__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__MoveGroup_Result__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MoveGroup_Result {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MoveGroup_Result where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/action/MoveGroup_Result";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__MoveGroup_Result() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__MoveGroup_Feedback() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__action__MoveGroup_Feedback__init(msg: *mut MoveGroup_Feedback) -> bool;
    fn moveit_msgs__action__MoveGroup_Feedback__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MoveGroup_Feedback>, size: usize) -> bool;
    fn moveit_msgs__action__MoveGroup_Feedback__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MoveGroup_Feedback>);
    fn moveit_msgs__action__MoveGroup_Feedback__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MoveGroup_Feedback>, out_seq: *mut rosidl_runtime_rs::Sequence<MoveGroup_Feedback>) -> bool;
}

// Corresponds to moveit_msgs__action__MoveGroup_Feedback
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MoveGroup_Feedback {
    /// The internal state that the move group action currently is in
    pub state: rosidl_runtime_rs::String,

}



impl Default for MoveGroup_Feedback {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__action__MoveGroup_Feedback__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__action__MoveGroup_Feedback__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MoveGroup_Feedback {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__MoveGroup_Feedback__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__MoveGroup_Feedback__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__MoveGroup_Feedback__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MoveGroup_Feedback {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MoveGroup_Feedback where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/action/MoveGroup_Feedback";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__MoveGroup_Feedback() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__MoveGroup_FeedbackMessage() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__action__MoveGroup_FeedbackMessage__init(msg: *mut MoveGroup_FeedbackMessage) -> bool;
    fn moveit_msgs__action__MoveGroup_FeedbackMessage__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MoveGroup_FeedbackMessage>, size: usize) -> bool;
    fn moveit_msgs__action__MoveGroup_FeedbackMessage__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MoveGroup_FeedbackMessage>);
    fn moveit_msgs__action__MoveGroup_FeedbackMessage__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MoveGroup_FeedbackMessage>, out_seq: *mut rosidl_runtime_rs::Sequence<MoveGroup_FeedbackMessage>) -> bool;
}

// Corresponds to moveit_msgs__action__MoveGroup_FeedbackMessage
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MoveGroup_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::super::action::rmw::MoveGroup_Feedback,

}



impl Default for MoveGroup_FeedbackMessage {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__action__MoveGroup_FeedbackMessage__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__action__MoveGroup_FeedbackMessage__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MoveGroup_FeedbackMessage {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__MoveGroup_FeedbackMessage__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__MoveGroup_FeedbackMessage__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__MoveGroup_FeedbackMessage__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MoveGroup_FeedbackMessage {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MoveGroup_FeedbackMessage where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/action/MoveGroup_FeedbackMessage";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__MoveGroup_FeedbackMessage() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__MoveGroupSequence_Goal() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__action__MoveGroupSequence_Goal__init(msg: *mut MoveGroupSequence_Goal) -> bool;
    fn moveit_msgs__action__MoveGroupSequence_Goal__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MoveGroupSequence_Goal>, size: usize) -> bool;
    fn moveit_msgs__action__MoveGroupSequence_Goal__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MoveGroupSequence_Goal>);
    fn moveit_msgs__action__MoveGroupSequence_Goal__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MoveGroupSequence_Goal>, out_seq: *mut rosidl_runtime_rs::Sequence<MoveGroupSequence_Goal>) -> bool;
}

// Corresponds to moveit_msgs__action__MoveGroupSequence_Goal
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MoveGroupSequence_Goal {

    // This member is not documented.
    #[allow(missing_docs)]
    pub request: super::super::msg::rmw::MotionSequenceRequest,

    /// Planning options
    pub planning_options: super::super::msg::rmw::PlanningOptions,

}



impl Default for MoveGroupSequence_Goal {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__action__MoveGroupSequence_Goal__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__action__MoveGroupSequence_Goal__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MoveGroupSequence_Goal {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__MoveGroupSequence_Goal__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__MoveGroupSequence_Goal__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__MoveGroupSequence_Goal__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MoveGroupSequence_Goal {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MoveGroupSequence_Goal where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/action/MoveGroupSequence_Goal";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__MoveGroupSequence_Goal() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__MoveGroupSequence_Result() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__action__MoveGroupSequence_Result__init(msg: *mut MoveGroupSequence_Result) -> bool;
    fn moveit_msgs__action__MoveGroupSequence_Result__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MoveGroupSequence_Result>, size: usize) -> bool;
    fn moveit_msgs__action__MoveGroupSequence_Result__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MoveGroupSequence_Result>);
    fn moveit_msgs__action__MoveGroupSequence_Result__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MoveGroupSequence_Result>, out_seq: *mut rosidl_runtime_rs::Sequence<MoveGroupSequence_Result>) -> bool;
}

// Corresponds to moveit_msgs__action__MoveGroupSequence_Result
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MoveGroupSequence_Result {
    /// Response comprising information on all sections of the sequence
    pub response: super::super::msg::rmw::MotionSequenceResponse,

}



impl Default for MoveGroupSequence_Result {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__action__MoveGroupSequence_Result__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__action__MoveGroupSequence_Result__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MoveGroupSequence_Result {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__MoveGroupSequence_Result__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__MoveGroupSequence_Result__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__MoveGroupSequence_Result__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MoveGroupSequence_Result {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MoveGroupSequence_Result where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/action/MoveGroupSequence_Result";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__MoveGroupSequence_Result() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__MoveGroupSequence_Feedback() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__action__MoveGroupSequence_Feedback__init(msg: *mut MoveGroupSequence_Feedback) -> bool;
    fn moveit_msgs__action__MoveGroupSequence_Feedback__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MoveGroupSequence_Feedback>, size: usize) -> bool;
    fn moveit_msgs__action__MoveGroupSequence_Feedback__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MoveGroupSequence_Feedback>);
    fn moveit_msgs__action__MoveGroupSequence_Feedback__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MoveGroupSequence_Feedback>, out_seq: *mut rosidl_runtime_rs::Sequence<MoveGroupSequence_Feedback>) -> bool;
}

// Corresponds to moveit_msgs__action__MoveGroupSequence_Feedback
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MoveGroupSequence_Feedback {
    /// The internal state that the move group action currently is in
    pub state: rosidl_runtime_rs::String,

}



impl Default for MoveGroupSequence_Feedback {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__action__MoveGroupSequence_Feedback__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__action__MoveGroupSequence_Feedback__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MoveGroupSequence_Feedback {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__MoveGroupSequence_Feedback__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__MoveGroupSequence_Feedback__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__MoveGroupSequence_Feedback__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MoveGroupSequence_Feedback {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MoveGroupSequence_Feedback where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/action/MoveGroupSequence_Feedback";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__MoveGroupSequence_Feedback() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__MoveGroupSequence_FeedbackMessage() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__action__MoveGroupSequence_FeedbackMessage__init(msg: *mut MoveGroupSequence_FeedbackMessage) -> bool;
    fn moveit_msgs__action__MoveGroupSequence_FeedbackMessage__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MoveGroupSequence_FeedbackMessage>, size: usize) -> bool;
    fn moveit_msgs__action__MoveGroupSequence_FeedbackMessage__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MoveGroupSequence_FeedbackMessage>);
    fn moveit_msgs__action__MoveGroupSequence_FeedbackMessage__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MoveGroupSequence_FeedbackMessage>, out_seq: *mut rosidl_runtime_rs::Sequence<MoveGroupSequence_FeedbackMessage>) -> bool;
}

// Corresponds to moveit_msgs__action__MoveGroupSequence_FeedbackMessage
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MoveGroupSequence_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::super::action::rmw::MoveGroupSequence_Feedback,

}



impl Default for MoveGroupSequence_FeedbackMessage {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__action__MoveGroupSequence_FeedbackMessage__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__action__MoveGroupSequence_FeedbackMessage__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MoveGroupSequence_FeedbackMessage {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__MoveGroupSequence_FeedbackMessage__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__MoveGroupSequence_FeedbackMessage__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__MoveGroupSequence_FeedbackMessage__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MoveGroupSequence_FeedbackMessage {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MoveGroupSequence_FeedbackMessage where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/action/MoveGroupSequence_FeedbackMessage";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__MoveGroupSequence_FeedbackMessage() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__Pickup_Goal() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__action__Pickup_Goal__init(msg: *mut Pickup_Goal) -> bool;
    fn moveit_msgs__action__Pickup_Goal__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Pickup_Goal>, size: usize) -> bool;
    fn moveit_msgs__action__Pickup_Goal__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Pickup_Goal>);
    fn moveit_msgs__action__Pickup_Goal__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Pickup_Goal>, out_seq: *mut rosidl_runtime_rs::Sequence<Pickup_Goal>) -> bool;
}

// Corresponds to moveit_msgs__action__Pickup_Goal
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Pickup_Goal {
    /// The name of the object to pick up (as known in the planning scene)
    pub target_name: rosidl_runtime_rs::String,

    /// which group should be used to plan for pickup
    pub group_name: rosidl_runtime_rs::String,

    /// which end-effector to be used for pickup (ideally descending from the group above)
    pub end_effector: rosidl_runtime_rs::String,

    /// a list of possible grasps to be used. At least one grasp must be filled in
    pub possible_grasps: rosidl_runtime_rs::Sequence<super::super::msg::rmw::Grasp>,

    /// the name that the support surface (e.g. table) has in the collision map
    /// can be left empty if no name is available
    pub support_surface_name: rosidl_runtime_rs::String,

    /// whether collisions between the gripper and the support surface should be acceptable
    /// during move from pre-grasp to grasp and during lift. Collisions when moving to the
    /// pre-grasp location are still not allowed even if this is set to true.
    pub allow_gripper_support_collision: bool,

    /// The names of the links the object to be attached is allowed to touch;
    /// If this is left empty, it defaults to the links in the used end-effector
    pub attached_object_touch_links: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,

    /// Optionally notify the pick action that it should approach the object further,
    /// as much as possible (this minimizing the distance to the object before the grasp)
    /// along the approach direction; Note: this option changes the grasping poses
    /// supplied in possible_grasps[] such that they are closer to the object when possible.
    pub minimize_object_distance: bool,

    /// Optional constraints to be imposed on every point in the motion plan
    pub path_constraints: super::super::msg::rmw::Constraints,

    /// The name of the motion planner to use. If no name is specified,
    /// a default motion planner will be used
    pub planner_id: rosidl_runtime_rs::String,

    /// an optional list of obstacles that we have semantic information about
    /// and that can be touched/pushed/moved in the course of grasping
    pub allowed_touch_objects: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,

    /// The maximum amount of time the motion planner is allowed to plan for
    pub allowed_planning_time: f64,

    /// Planning options
    pub planning_options: super::super::msg::rmw::PlanningOptions,

}



impl Default for Pickup_Goal {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__action__Pickup_Goal__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__action__Pickup_Goal__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Pickup_Goal {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__Pickup_Goal__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__Pickup_Goal__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__Pickup_Goal__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Pickup_Goal {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Pickup_Goal where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/action/Pickup_Goal";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__Pickup_Goal() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__Pickup_Result() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__action__Pickup_Result__init(msg: *mut Pickup_Result) -> bool;
    fn moveit_msgs__action__Pickup_Result__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Pickup_Result>, size: usize) -> bool;
    fn moveit_msgs__action__Pickup_Result__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Pickup_Result>);
    fn moveit_msgs__action__Pickup_Result__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Pickup_Result>, out_seq: *mut rosidl_runtime_rs::Sequence<Pickup_Result>) -> bool;
}

// Corresponds to moveit_msgs__action__Pickup_Result
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Pickup_Result {
    /// The overall result of the pickup attempt
    pub error_code: super::super::msg::rmw::MoveItErrorCodes,

    /// The full starting state of the robot at the start of the trajectory
    pub trajectory_start: super::super::msg::rmw::RobotState,

    /// The trajectory that moved group produced for execution
    pub trajectory_stages: rosidl_runtime_rs::Sequence<super::super::msg::rmw::RobotTrajectory>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub trajectory_descriptions: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,

    /// The performed grasp, if attempt was successful
    pub grasp: super::super::msg::rmw::Grasp,

    /// The amount of time in seconds it took to complete the plan
    pub planning_time: f64,

}



impl Default for Pickup_Result {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__action__Pickup_Result__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__action__Pickup_Result__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Pickup_Result {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__Pickup_Result__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__Pickup_Result__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__Pickup_Result__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Pickup_Result {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Pickup_Result where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/action/Pickup_Result";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__Pickup_Result() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__Pickup_Feedback() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__action__Pickup_Feedback__init(msg: *mut Pickup_Feedback) -> bool;
    fn moveit_msgs__action__Pickup_Feedback__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Pickup_Feedback>, size: usize) -> bool;
    fn moveit_msgs__action__Pickup_Feedback__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Pickup_Feedback>);
    fn moveit_msgs__action__Pickup_Feedback__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Pickup_Feedback>, out_seq: *mut rosidl_runtime_rs::Sequence<Pickup_Feedback>) -> bool;
}

// Corresponds to moveit_msgs__action__Pickup_Feedback
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Pickup_Feedback {
    /// The internal state that the pickup action currently is in
    pub state: rosidl_runtime_rs::String,

}



impl Default for Pickup_Feedback {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__action__Pickup_Feedback__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__action__Pickup_Feedback__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Pickup_Feedback {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__Pickup_Feedback__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__Pickup_Feedback__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__Pickup_Feedback__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Pickup_Feedback {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Pickup_Feedback where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/action/Pickup_Feedback";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__Pickup_Feedback() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__Pickup_FeedbackMessage() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__action__Pickup_FeedbackMessage__init(msg: *mut Pickup_FeedbackMessage) -> bool;
    fn moveit_msgs__action__Pickup_FeedbackMessage__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Pickup_FeedbackMessage>, size: usize) -> bool;
    fn moveit_msgs__action__Pickup_FeedbackMessage__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Pickup_FeedbackMessage>);
    fn moveit_msgs__action__Pickup_FeedbackMessage__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Pickup_FeedbackMessage>, out_seq: *mut rosidl_runtime_rs::Sequence<Pickup_FeedbackMessage>) -> bool;
}

// Corresponds to moveit_msgs__action__Pickup_FeedbackMessage
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Pickup_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::super::action::rmw::Pickup_Feedback,

}



impl Default for Pickup_FeedbackMessage {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__action__Pickup_FeedbackMessage__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__action__Pickup_FeedbackMessage__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Pickup_FeedbackMessage {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__Pickup_FeedbackMessage__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__Pickup_FeedbackMessage__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__Pickup_FeedbackMessage__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Pickup_FeedbackMessage {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Pickup_FeedbackMessage where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/action/Pickup_FeedbackMessage";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__Pickup_FeedbackMessage() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__Place_Goal() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__action__Place_Goal__init(msg: *mut Place_Goal) -> bool;
    fn moveit_msgs__action__Place_Goal__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Place_Goal>, size: usize) -> bool;
    fn moveit_msgs__action__Place_Goal__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Place_Goal>);
    fn moveit_msgs__action__Place_Goal__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Place_Goal>, out_seq: *mut rosidl_runtime_rs::Sequence<Place_Goal>) -> bool;
}

// Corresponds to moveit_msgs__action__Place_Goal
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Place_Goal {
    /// which group to be used to plan for grasping
    pub group_name: rosidl_runtime_rs::String,

    /// the name of the attached object to place
    pub attached_object_name: rosidl_runtime_rs::String,

    /// a list of possible transformations for placing the object
    pub place_locations: rosidl_runtime_rs::Sequence<super::super::msg::rmw::PlaceLocation>,

    /// if the user prefers setting the eef pose (same as in pick) rather than
    /// the location of the object, this flag should be set to true
    pub place_eef: bool,

    /// the name that the support surface (e.g. table) has in the collision world
    /// can be left empty if no name is available
    pub support_surface_name: rosidl_runtime_rs::String,

    /// whether collisions between the gripper and the support surface should be acceptable
    /// during move from pre-place to place and during retreat. Collisions when moving to the
    /// pre-place location are still not allowed even if this is set to true.
    pub allow_gripper_support_collision: bool,

    /// Optional constraints to be imposed on every point in the motion plan
    pub path_constraints: super::super::msg::rmw::Constraints,

    /// The name of the motion planner to use. If no name is specified,
    /// a default motion planner will be used
    pub planner_id: rosidl_runtime_rs::String,

    /// an optional list of obstacles that we have semantic information about
    /// and that can be touched/pushed/moved in the course of placing
    pub allowed_touch_objects: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,

    /// The maximum amount of time the motion planner is allowed to plan for
    pub allowed_planning_time: f64,

    /// Planning options
    pub planning_options: super::super::msg::rmw::PlanningOptions,

}



impl Default for Place_Goal {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__action__Place_Goal__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__action__Place_Goal__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Place_Goal {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__Place_Goal__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__Place_Goal__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__Place_Goal__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Place_Goal {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Place_Goal where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/action/Place_Goal";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__Place_Goal() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__Place_Result() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__action__Place_Result__init(msg: *mut Place_Result) -> bool;
    fn moveit_msgs__action__Place_Result__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Place_Result>, size: usize) -> bool;
    fn moveit_msgs__action__Place_Result__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Place_Result>);
    fn moveit_msgs__action__Place_Result__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Place_Result>, out_seq: *mut rosidl_runtime_rs::Sequence<Place_Result>) -> bool;
}

// Corresponds to moveit_msgs__action__Place_Result
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Place_Result {
    /// The result of the place attempt
    pub error_code: super::super::msg::rmw::MoveItErrorCodes,

    /// The full starting state of the robot at the start of the trajectory
    pub trajectory_start: super::super::msg::rmw::RobotState,

    /// The trajectory that moved group produced for execution
    pub trajectory_stages: rosidl_runtime_rs::Sequence<super::super::msg::rmw::RobotTrajectory>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub trajectory_descriptions: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,

    /// The successful place location, if any
    pub place_location: super::super::msg::rmw::PlaceLocation,

    /// The amount of time in seconds it took to complete the plan
    pub planning_time: f64,

}



impl Default for Place_Result {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__action__Place_Result__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__action__Place_Result__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Place_Result {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__Place_Result__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__Place_Result__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__Place_Result__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Place_Result {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Place_Result where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/action/Place_Result";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__Place_Result() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__Place_Feedback() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__action__Place_Feedback__init(msg: *mut Place_Feedback) -> bool;
    fn moveit_msgs__action__Place_Feedback__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Place_Feedback>, size: usize) -> bool;
    fn moveit_msgs__action__Place_Feedback__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Place_Feedback>);
    fn moveit_msgs__action__Place_Feedback__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Place_Feedback>, out_seq: *mut rosidl_runtime_rs::Sequence<Place_Feedback>) -> bool;
}

// Corresponds to moveit_msgs__action__Place_Feedback
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Place_Feedback {
    /// The internal state that the place action currently is in
    pub state: rosidl_runtime_rs::String,

}



impl Default for Place_Feedback {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__action__Place_Feedback__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__action__Place_Feedback__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Place_Feedback {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__Place_Feedback__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__Place_Feedback__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__Place_Feedback__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Place_Feedback {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Place_Feedback where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/action/Place_Feedback";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__Place_Feedback() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__Place_FeedbackMessage() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__action__Place_FeedbackMessage__init(msg: *mut Place_FeedbackMessage) -> bool;
    fn moveit_msgs__action__Place_FeedbackMessage__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Place_FeedbackMessage>, size: usize) -> bool;
    fn moveit_msgs__action__Place_FeedbackMessage__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Place_FeedbackMessage>);
    fn moveit_msgs__action__Place_FeedbackMessage__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Place_FeedbackMessage>, out_seq: *mut rosidl_runtime_rs::Sequence<Place_FeedbackMessage>) -> bool;
}

// Corresponds to moveit_msgs__action__Place_FeedbackMessage
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Place_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::super::action::rmw::Place_Feedback,

}



impl Default for Place_FeedbackMessage {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__action__Place_FeedbackMessage__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__action__Place_FeedbackMessage__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Place_FeedbackMessage {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__Place_FeedbackMessage__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__Place_FeedbackMessage__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__Place_FeedbackMessage__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Place_FeedbackMessage {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Place_FeedbackMessage where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/action/Place_FeedbackMessage";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__Place_FeedbackMessage() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__LocalPlanner_Goal() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__action__LocalPlanner_Goal__init(msg: *mut LocalPlanner_Goal) -> bool;
    fn moveit_msgs__action__LocalPlanner_Goal__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<LocalPlanner_Goal>, size: usize) -> bool;
    fn moveit_msgs__action__LocalPlanner_Goal__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<LocalPlanner_Goal>);
    fn moveit_msgs__action__LocalPlanner_Goal__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<LocalPlanner_Goal>, out_seq: *mut rosidl_runtime_rs::Sequence<LocalPlanner_Goal>) -> bool;
}

// Corresponds to moveit_msgs__action__LocalPlanner_Goal
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LocalPlanner_Goal {

    // This member is not documented.
    #[allow(missing_docs)]
    pub local_constraints: rosidl_runtime_rs::Sequence<super::super::msg::rmw::Constraints>,

}



impl Default for LocalPlanner_Goal {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__action__LocalPlanner_Goal__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__action__LocalPlanner_Goal__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for LocalPlanner_Goal {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__LocalPlanner_Goal__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__LocalPlanner_Goal__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__LocalPlanner_Goal__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for LocalPlanner_Goal {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for LocalPlanner_Goal where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/action/LocalPlanner_Goal";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__LocalPlanner_Goal() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__LocalPlanner_Result() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__action__LocalPlanner_Result__init(msg: *mut LocalPlanner_Result) -> bool;
    fn moveit_msgs__action__LocalPlanner_Result__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<LocalPlanner_Result>, size: usize) -> bool;
    fn moveit_msgs__action__LocalPlanner_Result__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<LocalPlanner_Result>);
    fn moveit_msgs__action__LocalPlanner_Result__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<LocalPlanner_Result>, out_seq: *mut rosidl_runtime_rs::Sequence<LocalPlanner_Result>) -> bool;
}

// Corresponds to moveit_msgs__action__LocalPlanner_Result
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LocalPlanner_Result {

    // This member is not documented.
    #[allow(missing_docs)]
    pub error_code: super::super::msg::rmw::MoveItErrorCodes,

    /// Error message with more detailed information about the local planning result
    pub error_message: rosidl_runtime_rs::String,

}



impl Default for LocalPlanner_Result {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__action__LocalPlanner_Result__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__action__LocalPlanner_Result__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for LocalPlanner_Result {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__LocalPlanner_Result__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__LocalPlanner_Result__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__LocalPlanner_Result__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for LocalPlanner_Result {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for LocalPlanner_Result where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/action/LocalPlanner_Result";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__LocalPlanner_Result() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__LocalPlanner_Feedback() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__action__LocalPlanner_Feedback__init(msg: *mut LocalPlanner_Feedback) -> bool;
    fn moveit_msgs__action__LocalPlanner_Feedback__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<LocalPlanner_Feedback>, size: usize) -> bool;
    fn moveit_msgs__action__LocalPlanner_Feedback__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<LocalPlanner_Feedback>);
    fn moveit_msgs__action__LocalPlanner_Feedback__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<LocalPlanner_Feedback>, out_seq: *mut rosidl_runtime_rs::Sequence<LocalPlanner_Feedback>) -> bool;
}

// Corresponds to moveit_msgs__action__LocalPlanner_Feedback
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LocalPlanner_Feedback {

    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: rosidl_runtime_rs::String,

}



impl Default for LocalPlanner_Feedback {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__action__LocalPlanner_Feedback__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__action__LocalPlanner_Feedback__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for LocalPlanner_Feedback {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__LocalPlanner_Feedback__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__LocalPlanner_Feedback__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__LocalPlanner_Feedback__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for LocalPlanner_Feedback {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for LocalPlanner_Feedback where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/action/LocalPlanner_Feedback";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__LocalPlanner_Feedback() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__LocalPlanner_FeedbackMessage() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__action__LocalPlanner_FeedbackMessage__init(msg: *mut LocalPlanner_FeedbackMessage) -> bool;
    fn moveit_msgs__action__LocalPlanner_FeedbackMessage__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<LocalPlanner_FeedbackMessage>, size: usize) -> bool;
    fn moveit_msgs__action__LocalPlanner_FeedbackMessage__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<LocalPlanner_FeedbackMessage>);
    fn moveit_msgs__action__LocalPlanner_FeedbackMessage__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<LocalPlanner_FeedbackMessage>, out_seq: *mut rosidl_runtime_rs::Sequence<LocalPlanner_FeedbackMessage>) -> bool;
}

// Corresponds to moveit_msgs__action__LocalPlanner_FeedbackMessage
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LocalPlanner_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::super::action::rmw::LocalPlanner_Feedback,

}



impl Default for LocalPlanner_FeedbackMessage {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__action__LocalPlanner_FeedbackMessage__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__action__LocalPlanner_FeedbackMessage__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for LocalPlanner_FeedbackMessage {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__LocalPlanner_FeedbackMessage__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__LocalPlanner_FeedbackMessage__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__LocalPlanner_FeedbackMessage__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for LocalPlanner_FeedbackMessage {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for LocalPlanner_FeedbackMessage where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/action/LocalPlanner_FeedbackMessage";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__LocalPlanner_FeedbackMessage() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__GlobalPlanner_Goal() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__action__GlobalPlanner_Goal__init(msg: *mut GlobalPlanner_Goal) -> bool;
    fn moveit_msgs__action__GlobalPlanner_Goal__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GlobalPlanner_Goal>, size: usize) -> bool;
    fn moveit_msgs__action__GlobalPlanner_Goal__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GlobalPlanner_Goal>);
    fn moveit_msgs__action__GlobalPlanner_Goal__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GlobalPlanner_Goal>, out_seq: *mut rosidl_runtime_rs::Sequence<GlobalPlanner_Goal>) -> bool;
}

// Corresponds to moveit_msgs__action__GlobalPlanner_Goal
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GlobalPlanner_Goal {

    // This member is not documented.
    #[allow(missing_docs)]
    pub planning_group: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub motion_sequence: super::super::msg::rmw::MotionSequenceRequest,

}



impl Default for GlobalPlanner_Goal {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__action__GlobalPlanner_Goal__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__action__GlobalPlanner_Goal__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GlobalPlanner_Goal {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__GlobalPlanner_Goal__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__GlobalPlanner_Goal__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__GlobalPlanner_Goal__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GlobalPlanner_Goal {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GlobalPlanner_Goal where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/action/GlobalPlanner_Goal";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__GlobalPlanner_Goal() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__GlobalPlanner_Result() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__action__GlobalPlanner_Result__init(msg: *mut GlobalPlanner_Result) -> bool;
    fn moveit_msgs__action__GlobalPlanner_Result__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GlobalPlanner_Result>, size: usize) -> bool;
    fn moveit_msgs__action__GlobalPlanner_Result__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GlobalPlanner_Result>);
    fn moveit_msgs__action__GlobalPlanner_Result__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GlobalPlanner_Result>, out_seq: *mut rosidl_runtime_rs::Sequence<GlobalPlanner_Result>) -> bool;
}

// Corresponds to moveit_msgs__action__GlobalPlanner_Result
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GlobalPlanner_Result {

    // This member is not documented.
    #[allow(missing_docs)]
    pub response: super::super::msg::rmw::MotionPlanResponse,

}



impl Default for GlobalPlanner_Result {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__action__GlobalPlanner_Result__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__action__GlobalPlanner_Result__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GlobalPlanner_Result {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__GlobalPlanner_Result__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__GlobalPlanner_Result__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__GlobalPlanner_Result__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GlobalPlanner_Result {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GlobalPlanner_Result where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/action/GlobalPlanner_Result";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__GlobalPlanner_Result() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__GlobalPlanner_Feedback() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__action__GlobalPlanner_Feedback__init(msg: *mut GlobalPlanner_Feedback) -> bool;
    fn moveit_msgs__action__GlobalPlanner_Feedback__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GlobalPlanner_Feedback>, size: usize) -> bool;
    fn moveit_msgs__action__GlobalPlanner_Feedback__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GlobalPlanner_Feedback>);
    fn moveit_msgs__action__GlobalPlanner_Feedback__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GlobalPlanner_Feedback>, out_seq: *mut rosidl_runtime_rs::Sequence<GlobalPlanner_Feedback>) -> bool;
}

// Corresponds to moveit_msgs__action__GlobalPlanner_Feedback
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GlobalPlanner_Feedback {

    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: rosidl_runtime_rs::String,

}



impl Default for GlobalPlanner_Feedback {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__action__GlobalPlanner_Feedback__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__action__GlobalPlanner_Feedback__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GlobalPlanner_Feedback {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__GlobalPlanner_Feedback__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__GlobalPlanner_Feedback__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__GlobalPlanner_Feedback__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GlobalPlanner_Feedback {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GlobalPlanner_Feedback where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/action/GlobalPlanner_Feedback";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__GlobalPlanner_Feedback() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__GlobalPlanner_FeedbackMessage() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__action__GlobalPlanner_FeedbackMessage__init(msg: *mut GlobalPlanner_FeedbackMessage) -> bool;
    fn moveit_msgs__action__GlobalPlanner_FeedbackMessage__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GlobalPlanner_FeedbackMessage>, size: usize) -> bool;
    fn moveit_msgs__action__GlobalPlanner_FeedbackMessage__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GlobalPlanner_FeedbackMessage>);
    fn moveit_msgs__action__GlobalPlanner_FeedbackMessage__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GlobalPlanner_FeedbackMessage>, out_seq: *mut rosidl_runtime_rs::Sequence<GlobalPlanner_FeedbackMessage>) -> bool;
}

// Corresponds to moveit_msgs__action__GlobalPlanner_FeedbackMessage
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GlobalPlanner_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::super::action::rmw::GlobalPlanner_Feedback,

}



impl Default for GlobalPlanner_FeedbackMessage {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__action__GlobalPlanner_FeedbackMessage__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__action__GlobalPlanner_FeedbackMessage__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GlobalPlanner_FeedbackMessage {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__GlobalPlanner_FeedbackMessage__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__GlobalPlanner_FeedbackMessage__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__GlobalPlanner_FeedbackMessage__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GlobalPlanner_FeedbackMessage {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GlobalPlanner_FeedbackMessage where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/action/GlobalPlanner_FeedbackMessage";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__GlobalPlanner_FeedbackMessage() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__HybridPlanner_Goal() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__action__HybridPlanner_Goal__init(msg: *mut HybridPlanner_Goal) -> bool;
    fn moveit_msgs__action__HybridPlanner_Goal__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<HybridPlanner_Goal>, size: usize) -> bool;
    fn moveit_msgs__action__HybridPlanner_Goal__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<HybridPlanner_Goal>);
    fn moveit_msgs__action__HybridPlanner_Goal__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<HybridPlanner_Goal>, out_seq: *mut rosidl_runtime_rs::Sequence<HybridPlanner_Goal>) -> bool;
}

// Corresponds to moveit_msgs__action__HybridPlanner_Goal
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct HybridPlanner_Goal {

    // This member is not documented.
    #[allow(missing_docs)]
    pub planning_group: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub motion_sequence: super::super::msg::rmw::MotionSequenceRequest,

}



impl Default for HybridPlanner_Goal {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__action__HybridPlanner_Goal__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__action__HybridPlanner_Goal__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for HybridPlanner_Goal {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__HybridPlanner_Goal__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__HybridPlanner_Goal__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__HybridPlanner_Goal__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for HybridPlanner_Goal {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for HybridPlanner_Goal where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/action/HybridPlanner_Goal";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__HybridPlanner_Goal() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__HybridPlanner_Result() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__action__HybridPlanner_Result__init(msg: *mut HybridPlanner_Result) -> bool;
    fn moveit_msgs__action__HybridPlanner_Result__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<HybridPlanner_Result>, size: usize) -> bool;
    fn moveit_msgs__action__HybridPlanner_Result__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<HybridPlanner_Result>);
    fn moveit_msgs__action__HybridPlanner_Result__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<HybridPlanner_Result>, out_seq: *mut rosidl_runtime_rs::Sequence<HybridPlanner_Result>) -> bool;
}

// Corresponds to moveit_msgs__action__HybridPlanner_Result
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct HybridPlanner_Result {

    // This member is not documented.
    #[allow(missing_docs)]
    pub error_code: super::super::msg::rmw::MoveItErrorCodes,

    /// Error message with more detailed information about the hybrid planning result
    pub error_message: rosidl_runtime_rs::String,

}



impl Default for HybridPlanner_Result {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__action__HybridPlanner_Result__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__action__HybridPlanner_Result__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for HybridPlanner_Result {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__HybridPlanner_Result__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__HybridPlanner_Result__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__HybridPlanner_Result__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for HybridPlanner_Result {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for HybridPlanner_Result where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/action/HybridPlanner_Result";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__HybridPlanner_Result() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__HybridPlanner_Feedback() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__action__HybridPlanner_Feedback__init(msg: *mut HybridPlanner_Feedback) -> bool;
    fn moveit_msgs__action__HybridPlanner_Feedback__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<HybridPlanner_Feedback>, size: usize) -> bool;
    fn moveit_msgs__action__HybridPlanner_Feedback__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<HybridPlanner_Feedback>);
    fn moveit_msgs__action__HybridPlanner_Feedback__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<HybridPlanner_Feedback>, out_seq: *mut rosidl_runtime_rs::Sequence<HybridPlanner_Feedback>) -> bool;
}

// Corresponds to moveit_msgs__action__HybridPlanner_Feedback
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct HybridPlanner_Feedback {

    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: rosidl_runtime_rs::String,

}



impl Default for HybridPlanner_Feedback {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__action__HybridPlanner_Feedback__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__action__HybridPlanner_Feedback__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for HybridPlanner_Feedback {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__HybridPlanner_Feedback__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__HybridPlanner_Feedback__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__HybridPlanner_Feedback__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for HybridPlanner_Feedback {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for HybridPlanner_Feedback where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/action/HybridPlanner_Feedback";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__HybridPlanner_Feedback() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__HybridPlanner_FeedbackMessage() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__action__HybridPlanner_FeedbackMessage__init(msg: *mut HybridPlanner_FeedbackMessage) -> bool;
    fn moveit_msgs__action__HybridPlanner_FeedbackMessage__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<HybridPlanner_FeedbackMessage>, size: usize) -> bool;
    fn moveit_msgs__action__HybridPlanner_FeedbackMessage__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<HybridPlanner_FeedbackMessage>);
    fn moveit_msgs__action__HybridPlanner_FeedbackMessage__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<HybridPlanner_FeedbackMessage>, out_seq: *mut rosidl_runtime_rs::Sequence<HybridPlanner_FeedbackMessage>) -> bool;
}

// Corresponds to moveit_msgs__action__HybridPlanner_FeedbackMessage
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct HybridPlanner_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::super::action::rmw::HybridPlanner_Feedback,

}



impl Default for HybridPlanner_FeedbackMessage {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__action__HybridPlanner_FeedbackMessage__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__action__HybridPlanner_FeedbackMessage__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for HybridPlanner_FeedbackMessage {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__HybridPlanner_FeedbackMessage__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__HybridPlanner_FeedbackMessage__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__HybridPlanner_FeedbackMessage__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for HybridPlanner_FeedbackMessage {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for HybridPlanner_FeedbackMessage where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/action/HybridPlanner_FeedbackMessage";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__HybridPlanner_FeedbackMessage() }
  }
}




#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__ExecuteTrajectory_SendGoal_Request() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__action__ExecuteTrajectory_SendGoal_Request__init(msg: *mut ExecuteTrajectory_SendGoal_Request) -> bool;
    fn moveit_msgs__action__ExecuteTrajectory_SendGoal_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ExecuteTrajectory_SendGoal_Request>, size: usize) -> bool;
    fn moveit_msgs__action__ExecuteTrajectory_SendGoal_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ExecuteTrajectory_SendGoal_Request>);
    fn moveit_msgs__action__ExecuteTrajectory_SendGoal_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ExecuteTrajectory_SendGoal_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<ExecuteTrajectory_SendGoal_Request>) -> bool;
}

// Corresponds to moveit_msgs__action__ExecuteTrajectory_SendGoal_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ExecuteTrajectory_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::super::action::rmw::ExecuteTrajectory_Goal,

}



impl Default for ExecuteTrajectory_SendGoal_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__action__ExecuteTrajectory_SendGoal_Request__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__action__ExecuteTrajectory_SendGoal_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ExecuteTrajectory_SendGoal_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__ExecuteTrajectory_SendGoal_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__ExecuteTrajectory_SendGoal_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__ExecuteTrajectory_SendGoal_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ExecuteTrajectory_SendGoal_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ExecuteTrajectory_SendGoal_Request where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/action/ExecuteTrajectory_SendGoal_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__ExecuteTrajectory_SendGoal_Request() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__ExecuteTrajectory_SendGoal_Response() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__action__ExecuteTrajectory_SendGoal_Response__init(msg: *mut ExecuteTrajectory_SendGoal_Response) -> bool;
    fn moveit_msgs__action__ExecuteTrajectory_SendGoal_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ExecuteTrajectory_SendGoal_Response>, size: usize) -> bool;
    fn moveit_msgs__action__ExecuteTrajectory_SendGoal_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ExecuteTrajectory_SendGoal_Response>);
    fn moveit_msgs__action__ExecuteTrajectory_SendGoal_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ExecuteTrajectory_SendGoal_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<ExecuteTrajectory_SendGoal_Response>) -> bool;
}

// Corresponds to moveit_msgs__action__ExecuteTrajectory_SendGoal_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ExecuteTrajectory_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::rmw::Time,

}



impl Default for ExecuteTrajectory_SendGoal_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__action__ExecuteTrajectory_SendGoal_Response__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__action__ExecuteTrajectory_SendGoal_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ExecuteTrajectory_SendGoal_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__ExecuteTrajectory_SendGoal_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__ExecuteTrajectory_SendGoal_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__ExecuteTrajectory_SendGoal_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ExecuteTrajectory_SendGoal_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ExecuteTrajectory_SendGoal_Response where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/action/ExecuteTrajectory_SendGoal_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__ExecuteTrajectory_SendGoal_Response() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__ExecuteTrajectory_GetResult_Request() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__action__ExecuteTrajectory_GetResult_Request__init(msg: *mut ExecuteTrajectory_GetResult_Request) -> bool;
    fn moveit_msgs__action__ExecuteTrajectory_GetResult_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ExecuteTrajectory_GetResult_Request>, size: usize) -> bool;
    fn moveit_msgs__action__ExecuteTrajectory_GetResult_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ExecuteTrajectory_GetResult_Request>);
    fn moveit_msgs__action__ExecuteTrajectory_GetResult_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ExecuteTrajectory_GetResult_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<ExecuteTrajectory_GetResult_Request>) -> bool;
}

// Corresponds to moveit_msgs__action__ExecuteTrajectory_GetResult_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ExecuteTrajectory_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,

}



impl Default for ExecuteTrajectory_GetResult_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__action__ExecuteTrajectory_GetResult_Request__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__action__ExecuteTrajectory_GetResult_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ExecuteTrajectory_GetResult_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__ExecuteTrajectory_GetResult_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__ExecuteTrajectory_GetResult_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__ExecuteTrajectory_GetResult_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ExecuteTrajectory_GetResult_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ExecuteTrajectory_GetResult_Request where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/action/ExecuteTrajectory_GetResult_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__ExecuteTrajectory_GetResult_Request() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__ExecuteTrajectory_GetResult_Response() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__action__ExecuteTrajectory_GetResult_Response__init(msg: *mut ExecuteTrajectory_GetResult_Response) -> bool;
    fn moveit_msgs__action__ExecuteTrajectory_GetResult_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ExecuteTrajectory_GetResult_Response>, size: usize) -> bool;
    fn moveit_msgs__action__ExecuteTrajectory_GetResult_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ExecuteTrajectory_GetResult_Response>);
    fn moveit_msgs__action__ExecuteTrajectory_GetResult_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ExecuteTrajectory_GetResult_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<ExecuteTrajectory_GetResult_Response>) -> bool;
}

// Corresponds to moveit_msgs__action__ExecuteTrajectory_GetResult_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ExecuteTrajectory_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::super::action::rmw::ExecuteTrajectory_Result,

}



impl Default for ExecuteTrajectory_GetResult_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__action__ExecuteTrajectory_GetResult_Response__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__action__ExecuteTrajectory_GetResult_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ExecuteTrajectory_GetResult_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__ExecuteTrajectory_GetResult_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__ExecuteTrajectory_GetResult_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__ExecuteTrajectory_GetResult_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ExecuteTrajectory_GetResult_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ExecuteTrajectory_GetResult_Response where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/action/ExecuteTrajectory_GetResult_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__ExecuteTrajectory_GetResult_Response() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__MoveGroup_SendGoal_Request() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__action__MoveGroup_SendGoal_Request__init(msg: *mut MoveGroup_SendGoal_Request) -> bool;
    fn moveit_msgs__action__MoveGroup_SendGoal_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MoveGroup_SendGoal_Request>, size: usize) -> bool;
    fn moveit_msgs__action__MoveGroup_SendGoal_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MoveGroup_SendGoal_Request>);
    fn moveit_msgs__action__MoveGroup_SendGoal_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MoveGroup_SendGoal_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<MoveGroup_SendGoal_Request>) -> bool;
}

// Corresponds to moveit_msgs__action__MoveGroup_SendGoal_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MoveGroup_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::super::action::rmw::MoveGroup_Goal,

}



impl Default for MoveGroup_SendGoal_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__action__MoveGroup_SendGoal_Request__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__action__MoveGroup_SendGoal_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MoveGroup_SendGoal_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__MoveGroup_SendGoal_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__MoveGroup_SendGoal_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__MoveGroup_SendGoal_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MoveGroup_SendGoal_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MoveGroup_SendGoal_Request where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/action/MoveGroup_SendGoal_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__MoveGroup_SendGoal_Request() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__MoveGroup_SendGoal_Response() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__action__MoveGroup_SendGoal_Response__init(msg: *mut MoveGroup_SendGoal_Response) -> bool;
    fn moveit_msgs__action__MoveGroup_SendGoal_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MoveGroup_SendGoal_Response>, size: usize) -> bool;
    fn moveit_msgs__action__MoveGroup_SendGoal_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MoveGroup_SendGoal_Response>);
    fn moveit_msgs__action__MoveGroup_SendGoal_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MoveGroup_SendGoal_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<MoveGroup_SendGoal_Response>) -> bool;
}

// Corresponds to moveit_msgs__action__MoveGroup_SendGoal_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MoveGroup_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::rmw::Time,

}



impl Default for MoveGroup_SendGoal_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__action__MoveGroup_SendGoal_Response__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__action__MoveGroup_SendGoal_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MoveGroup_SendGoal_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__MoveGroup_SendGoal_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__MoveGroup_SendGoal_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__MoveGroup_SendGoal_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MoveGroup_SendGoal_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MoveGroup_SendGoal_Response where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/action/MoveGroup_SendGoal_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__MoveGroup_SendGoal_Response() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__MoveGroup_GetResult_Request() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__action__MoveGroup_GetResult_Request__init(msg: *mut MoveGroup_GetResult_Request) -> bool;
    fn moveit_msgs__action__MoveGroup_GetResult_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MoveGroup_GetResult_Request>, size: usize) -> bool;
    fn moveit_msgs__action__MoveGroup_GetResult_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MoveGroup_GetResult_Request>);
    fn moveit_msgs__action__MoveGroup_GetResult_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MoveGroup_GetResult_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<MoveGroup_GetResult_Request>) -> bool;
}

// Corresponds to moveit_msgs__action__MoveGroup_GetResult_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MoveGroup_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,

}



impl Default for MoveGroup_GetResult_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__action__MoveGroup_GetResult_Request__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__action__MoveGroup_GetResult_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MoveGroup_GetResult_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__MoveGroup_GetResult_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__MoveGroup_GetResult_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__MoveGroup_GetResult_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MoveGroup_GetResult_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MoveGroup_GetResult_Request where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/action/MoveGroup_GetResult_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__MoveGroup_GetResult_Request() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__MoveGroup_GetResult_Response() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__action__MoveGroup_GetResult_Response__init(msg: *mut MoveGroup_GetResult_Response) -> bool;
    fn moveit_msgs__action__MoveGroup_GetResult_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MoveGroup_GetResult_Response>, size: usize) -> bool;
    fn moveit_msgs__action__MoveGroup_GetResult_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MoveGroup_GetResult_Response>);
    fn moveit_msgs__action__MoveGroup_GetResult_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MoveGroup_GetResult_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<MoveGroup_GetResult_Response>) -> bool;
}

// Corresponds to moveit_msgs__action__MoveGroup_GetResult_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MoveGroup_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::super::action::rmw::MoveGroup_Result,

}



impl Default for MoveGroup_GetResult_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__action__MoveGroup_GetResult_Response__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__action__MoveGroup_GetResult_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MoveGroup_GetResult_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__MoveGroup_GetResult_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__MoveGroup_GetResult_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__MoveGroup_GetResult_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MoveGroup_GetResult_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MoveGroup_GetResult_Response where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/action/MoveGroup_GetResult_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__MoveGroup_GetResult_Response() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__MoveGroupSequence_SendGoal_Request() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__action__MoveGroupSequence_SendGoal_Request__init(msg: *mut MoveGroupSequence_SendGoal_Request) -> bool;
    fn moveit_msgs__action__MoveGroupSequence_SendGoal_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MoveGroupSequence_SendGoal_Request>, size: usize) -> bool;
    fn moveit_msgs__action__MoveGroupSequence_SendGoal_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MoveGroupSequence_SendGoal_Request>);
    fn moveit_msgs__action__MoveGroupSequence_SendGoal_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MoveGroupSequence_SendGoal_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<MoveGroupSequence_SendGoal_Request>) -> bool;
}

// Corresponds to moveit_msgs__action__MoveGroupSequence_SendGoal_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MoveGroupSequence_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::super::action::rmw::MoveGroupSequence_Goal,

}



impl Default for MoveGroupSequence_SendGoal_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__action__MoveGroupSequence_SendGoal_Request__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__action__MoveGroupSequence_SendGoal_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MoveGroupSequence_SendGoal_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__MoveGroupSequence_SendGoal_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__MoveGroupSequence_SendGoal_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__MoveGroupSequence_SendGoal_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MoveGroupSequence_SendGoal_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MoveGroupSequence_SendGoal_Request where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/action/MoveGroupSequence_SendGoal_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__MoveGroupSequence_SendGoal_Request() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__MoveGroupSequence_SendGoal_Response() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__action__MoveGroupSequence_SendGoal_Response__init(msg: *mut MoveGroupSequence_SendGoal_Response) -> bool;
    fn moveit_msgs__action__MoveGroupSequence_SendGoal_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MoveGroupSequence_SendGoal_Response>, size: usize) -> bool;
    fn moveit_msgs__action__MoveGroupSequence_SendGoal_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MoveGroupSequence_SendGoal_Response>);
    fn moveit_msgs__action__MoveGroupSequence_SendGoal_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MoveGroupSequence_SendGoal_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<MoveGroupSequence_SendGoal_Response>) -> bool;
}

// Corresponds to moveit_msgs__action__MoveGroupSequence_SendGoal_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MoveGroupSequence_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::rmw::Time,

}



impl Default for MoveGroupSequence_SendGoal_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__action__MoveGroupSequence_SendGoal_Response__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__action__MoveGroupSequence_SendGoal_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MoveGroupSequence_SendGoal_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__MoveGroupSequence_SendGoal_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__MoveGroupSequence_SendGoal_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__MoveGroupSequence_SendGoal_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MoveGroupSequence_SendGoal_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MoveGroupSequence_SendGoal_Response where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/action/MoveGroupSequence_SendGoal_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__MoveGroupSequence_SendGoal_Response() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__MoveGroupSequence_GetResult_Request() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__action__MoveGroupSequence_GetResult_Request__init(msg: *mut MoveGroupSequence_GetResult_Request) -> bool;
    fn moveit_msgs__action__MoveGroupSequence_GetResult_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MoveGroupSequence_GetResult_Request>, size: usize) -> bool;
    fn moveit_msgs__action__MoveGroupSequence_GetResult_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MoveGroupSequence_GetResult_Request>);
    fn moveit_msgs__action__MoveGroupSequence_GetResult_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MoveGroupSequence_GetResult_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<MoveGroupSequence_GetResult_Request>) -> bool;
}

// Corresponds to moveit_msgs__action__MoveGroupSequence_GetResult_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MoveGroupSequence_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,

}



impl Default for MoveGroupSequence_GetResult_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__action__MoveGroupSequence_GetResult_Request__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__action__MoveGroupSequence_GetResult_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MoveGroupSequence_GetResult_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__MoveGroupSequence_GetResult_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__MoveGroupSequence_GetResult_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__MoveGroupSequence_GetResult_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MoveGroupSequence_GetResult_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MoveGroupSequence_GetResult_Request where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/action/MoveGroupSequence_GetResult_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__MoveGroupSequence_GetResult_Request() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__MoveGroupSequence_GetResult_Response() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__action__MoveGroupSequence_GetResult_Response__init(msg: *mut MoveGroupSequence_GetResult_Response) -> bool;
    fn moveit_msgs__action__MoveGroupSequence_GetResult_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MoveGroupSequence_GetResult_Response>, size: usize) -> bool;
    fn moveit_msgs__action__MoveGroupSequence_GetResult_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MoveGroupSequence_GetResult_Response>);
    fn moveit_msgs__action__MoveGroupSequence_GetResult_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MoveGroupSequence_GetResult_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<MoveGroupSequence_GetResult_Response>) -> bool;
}

// Corresponds to moveit_msgs__action__MoveGroupSequence_GetResult_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MoveGroupSequence_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::super::action::rmw::MoveGroupSequence_Result,

}



impl Default for MoveGroupSequence_GetResult_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__action__MoveGroupSequence_GetResult_Response__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__action__MoveGroupSequence_GetResult_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MoveGroupSequence_GetResult_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__MoveGroupSequence_GetResult_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__MoveGroupSequence_GetResult_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__MoveGroupSequence_GetResult_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MoveGroupSequence_GetResult_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MoveGroupSequence_GetResult_Response where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/action/MoveGroupSequence_GetResult_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__MoveGroupSequence_GetResult_Response() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__Pickup_SendGoal_Request() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__action__Pickup_SendGoal_Request__init(msg: *mut Pickup_SendGoal_Request) -> bool;
    fn moveit_msgs__action__Pickup_SendGoal_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Pickup_SendGoal_Request>, size: usize) -> bool;
    fn moveit_msgs__action__Pickup_SendGoal_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Pickup_SendGoal_Request>);
    fn moveit_msgs__action__Pickup_SendGoal_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Pickup_SendGoal_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<Pickup_SendGoal_Request>) -> bool;
}

// Corresponds to moveit_msgs__action__Pickup_SendGoal_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Pickup_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::super::action::rmw::Pickup_Goal,

}



impl Default for Pickup_SendGoal_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__action__Pickup_SendGoal_Request__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__action__Pickup_SendGoal_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Pickup_SendGoal_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__Pickup_SendGoal_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__Pickup_SendGoal_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__Pickup_SendGoal_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Pickup_SendGoal_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Pickup_SendGoal_Request where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/action/Pickup_SendGoal_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__Pickup_SendGoal_Request() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__Pickup_SendGoal_Response() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__action__Pickup_SendGoal_Response__init(msg: *mut Pickup_SendGoal_Response) -> bool;
    fn moveit_msgs__action__Pickup_SendGoal_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Pickup_SendGoal_Response>, size: usize) -> bool;
    fn moveit_msgs__action__Pickup_SendGoal_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Pickup_SendGoal_Response>);
    fn moveit_msgs__action__Pickup_SendGoal_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Pickup_SendGoal_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<Pickup_SendGoal_Response>) -> bool;
}

// Corresponds to moveit_msgs__action__Pickup_SendGoal_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Pickup_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::rmw::Time,

}



impl Default for Pickup_SendGoal_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__action__Pickup_SendGoal_Response__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__action__Pickup_SendGoal_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Pickup_SendGoal_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__Pickup_SendGoal_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__Pickup_SendGoal_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__Pickup_SendGoal_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Pickup_SendGoal_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Pickup_SendGoal_Response where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/action/Pickup_SendGoal_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__Pickup_SendGoal_Response() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__Pickup_GetResult_Request() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__action__Pickup_GetResult_Request__init(msg: *mut Pickup_GetResult_Request) -> bool;
    fn moveit_msgs__action__Pickup_GetResult_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Pickup_GetResult_Request>, size: usize) -> bool;
    fn moveit_msgs__action__Pickup_GetResult_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Pickup_GetResult_Request>);
    fn moveit_msgs__action__Pickup_GetResult_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Pickup_GetResult_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<Pickup_GetResult_Request>) -> bool;
}

// Corresponds to moveit_msgs__action__Pickup_GetResult_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Pickup_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,

}



impl Default for Pickup_GetResult_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__action__Pickup_GetResult_Request__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__action__Pickup_GetResult_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Pickup_GetResult_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__Pickup_GetResult_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__Pickup_GetResult_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__Pickup_GetResult_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Pickup_GetResult_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Pickup_GetResult_Request where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/action/Pickup_GetResult_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__Pickup_GetResult_Request() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__Pickup_GetResult_Response() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__action__Pickup_GetResult_Response__init(msg: *mut Pickup_GetResult_Response) -> bool;
    fn moveit_msgs__action__Pickup_GetResult_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Pickup_GetResult_Response>, size: usize) -> bool;
    fn moveit_msgs__action__Pickup_GetResult_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Pickup_GetResult_Response>);
    fn moveit_msgs__action__Pickup_GetResult_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Pickup_GetResult_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<Pickup_GetResult_Response>) -> bool;
}

// Corresponds to moveit_msgs__action__Pickup_GetResult_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Pickup_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::super::action::rmw::Pickup_Result,

}



impl Default for Pickup_GetResult_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__action__Pickup_GetResult_Response__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__action__Pickup_GetResult_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Pickup_GetResult_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__Pickup_GetResult_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__Pickup_GetResult_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__Pickup_GetResult_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Pickup_GetResult_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Pickup_GetResult_Response where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/action/Pickup_GetResult_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__Pickup_GetResult_Response() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__Place_SendGoal_Request() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__action__Place_SendGoal_Request__init(msg: *mut Place_SendGoal_Request) -> bool;
    fn moveit_msgs__action__Place_SendGoal_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Place_SendGoal_Request>, size: usize) -> bool;
    fn moveit_msgs__action__Place_SendGoal_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Place_SendGoal_Request>);
    fn moveit_msgs__action__Place_SendGoal_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Place_SendGoal_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<Place_SendGoal_Request>) -> bool;
}

// Corresponds to moveit_msgs__action__Place_SendGoal_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Place_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::super::action::rmw::Place_Goal,

}



impl Default for Place_SendGoal_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__action__Place_SendGoal_Request__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__action__Place_SendGoal_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Place_SendGoal_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__Place_SendGoal_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__Place_SendGoal_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__Place_SendGoal_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Place_SendGoal_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Place_SendGoal_Request where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/action/Place_SendGoal_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__Place_SendGoal_Request() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__Place_SendGoal_Response() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__action__Place_SendGoal_Response__init(msg: *mut Place_SendGoal_Response) -> bool;
    fn moveit_msgs__action__Place_SendGoal_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Place_SendGoal_Response>, size: usize) -> bool;
    fn moveit_msgs__action__Place_SendGoal_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Place_SendGoal_Response>);
    fn moveit_msgs__action__Place_SendGoal_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Place_SendGoal_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<Place_SendGoal_Response>) -> bool;
}

// Corresponds to moveit_msgs__action__Place_SendGoal_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Place_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::rmw::Time,

}



impl Default for Place_SendGoal_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__action__Place_SendGoal_Response__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__action__Place_SendGoal_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Place_SendGoal_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__Place_SendGoal_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__Place_SendGoal_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__Place_SendGoal_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Place_SendGoal_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Place_SendGoal_Response where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/action/Place_SendGoal_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__Place_SendGoal_Response() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__Place_GetResult_Request() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__action__Place_GetResult_Request__init(msg: *mut Place_GetResult_Request) -> bool;
    fn moveit_msgs__action__Place_GetResult_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Place_GetResult_Request>, size: usize) -> bool;
    fn moveit_msgs__action__Place_GetResult_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Place_GetResult_Request>);
    fn moveit_msgs__action__Place_GetResult_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Place_GetResult_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<Place_GetResult_Request>) -> bool;
}

// Corresponds to moveit_msgs__action__Place_GetResult_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Place_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,

}



impl Default for Place_GetResult_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__action__Place_GetResult_Request__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__action__Place_GetResult_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Place_GetResult_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__Place_GetResult_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__Place_GetResult_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__Place_GetResult_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Place_GetResult_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Place_GetResult_Request where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/action/Place_GetResult_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__Place_GetResult_Request() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__Place_GetResult_Response() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__action__Place_GetResult_Response__init(msg: *mut Place_GetResult_Response) -> bool;
    fn moveit_msgs__action__Place_GetResult_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Place_GetResult_Response>, size: usize) -> bool;
    fn moveit_msgs__action__Place_GetResult_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Place_GetResult_Response>);
    fn moveit_msgs__action__Place_GetResult_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Place_GetResult_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<Place_GetResult_Response>) -> bool;
}

// Corresponds to moveit_msgs__action__Place_GetResult_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Place_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::super::action::rmw::Place_Result,

}



impl Default for Place_GetResult_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__action__Place_GetResult_Response__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__action__Place_GetResult_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Place_GetResult_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__Place_GetResult_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__Place_GetResult_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__Place_GetResult_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Place_GetResult_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Place_GetResult_Response where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/action/Place_GetResult_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__Place_GetResult_Response() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__LocalPlanner_SendGoal_Request() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__action__LocalPlanner_SendGoal_Request__init(msg: *mut LocalPlanner_SendGoal_Request) -> bool;
    fn moveit_msgs__action__LocalPlanner_SendGoal_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<LocalPlanner_SendGoal_Request>, size: usize) -> bool;
    fn moveit_msgs__action__LocalPlanner_SendGoal_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<LocalPlanner_SendGoal_Request>);
    fn moveit_msgs__action__LocalPlanner_SendGoal_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<LocalPlanner_SendGoal_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<LocalPlanner_SendGoal_Request>) -> bool;
}

// Corresponds to moveit_msgs__action__LocalPlanner_SendGoal_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LocalPlanner_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::super::action::rmw::LocalPlanner_Goal,

}



impl Default for LocalPlanner_SendGoal_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__action__LocalPlanner_SendGoal_Request__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__action__LocalPlanner_SendGoal_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for LocalPlanner_SendGoal_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__LocalPlanner_SendGoal_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__LocalPlanner_SendGoal_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__LocalPlanner_SendGoal_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for LocalPlanner_SendGoal_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for LocalPlanner_SendGoal_Request where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/action/LocalPlanner_SendGoal_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__LocalPlanner_SendGoal_Request() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__LocalPlanner_SendGoal_Response() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__action__LocalPlanner_SendGoal_Response__init(msg: *mut LocalPlanner_SendGoal_Response) -> bool;
    fn moveit_msgs__action__LocalPlanner_SendGoal_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<LocalPlanner_SendGoal_Response>, size: usize) -> bool;
    fn moveit_msgs__action__LocalPlanner_SendGoal_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<LocalPlanner_SendGoal_Response>);
    fn moveit_msgs__action__LocalPlanner_SendGoal_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<LocalPlanner_SendGoal_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<LocalPlanner_SendGoal_Response>) -> bool;
}

// Corresponds to moveit_msgs__action__LocalPlanner_SendGoal_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LocalPlanner_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::rmw::Time,

}



impl Default for LocalPlanner_SendGoal_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__action__LocalPlanner_SendGoal_Response__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__action__LocalPlanner_SendGoal_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for LocalPlanner_SendGoal_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__LocalPlanner_SendGoal_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__LocalPlanner_SendGoal_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__LocalPlanner_SendGoal_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for LocalPlanner_SendGoal_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for LocalPlanner_SendGoal_Response where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/action/LocalPlanner_SendGoal_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__LocalPlanner_SendGoal_Response() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__LocalPlanner_GetResult_Request() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__action__LocalPlanner_GetResult_Request__init(msg: *mut LocalPlanner_GetResult_Request) -> bool;
    fn moveit_msgs__action__LocalPlanner_GetResult_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<LocalPlanner_GetResult_Request>, size: usize) -> bool;
    fn moveit_msgs__action__LocalPlanner_GetResult_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<LocalPlanner_GetResult_Request>);
    fn moveit_msgs__action__LocalPlanner_GetResult_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<LocalPlanner_GetResult_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<LocalPlanner_GetResult_Request>) -> bool;
}

// Corresponds to moveit_msgs__action__LocalPlanner_GetResult_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LocalPlanner_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,

}



impl Default for LocalPlanner_GetResult_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__action__LocalPlanner_GetResult_Request__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__action__LocalPlanner_GetResult_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for LocalPlanner_GetResult_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__LocalPlanner_GetResult_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__LocalPlanner_GetResult_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__LocalPlanner_GetResult_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for LocalPlanner_GetResult_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for LocalPlanner_GetResult_Request where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/action/LocalPlanner_GetResult_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__LocalPlanner_GetResult_Request() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__LocalPlanner_GetResult_Response() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__action__LocalPlanner_GetResult_Response__init(msg: *mut LocalPlanner_GetResult_Response) -> bool;
    fn moveit_msgs__action__LocalPlanner_GetResult_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<LocalPlanner_GetResult_Response>, size: usize) -> bool;
    fn moveit_msgs__action__LocalPlanner_GetResult_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<LocalPlanner_GetResult_Response>);
    fn moveit_msgs__action__LocalPlanner_GetResult_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<LocalPlanner_GetResult_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<LocalPlanner_GetResult_Response>) -> bool;
}

// Corresponds to moveit_msgs__action__LocalPlanner_GetResult_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LocalPlanner_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::super::action::rmw::LocalPlanner_Result,

}



impl Default for LocalPlanner_GetResult_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__action__LocalPlanner_GetResult_Response__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__action__LocalPlanner_GetResult_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for LocalPlanner_GetResult_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__LocalPlanner_GetResult_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__LocalPlanner_GetResult_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__LocalPlanner_GetResult_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for LocalPlanner_GetResult_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for LocalPlanner_GetResult_Response where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/action/LocalPlanner_GetResult_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__LocalPlanner_GetResult_Response() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__GlobalPlanner_SendGoal_Request() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__action__GlobalPlanner_SendGoal_Request__init(msg: *mut GlobalPlanner_SendGoal_Request) -> bool;
    fn moveit_msgs__action__GlobalPlanner_SendGoal_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GlobalPlanner_SendGoal_Request>, size: usize) -> bool;
    fn moveit_msgs__action__GlobalPlanner_SendGoal_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GlobalPlanner_SendGoal_Request>);
    fn moveit_msgs__action__GlobalPlanner_SendGoal_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GlobalPlanner_SendGoal_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GlobalPlanner_SendGoal_Request>) -> bool;
}

// Corresponds to moveit_msgs__action__GlobalPlanner_SendGoal_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GlobalPlanner_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::super::action::rmw::GlobalPlanner_Goal,

}



impl Default for GlobalPlanner_SendGoal_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__action__GlobalPlanner_SendGoal_Request__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__action__GlobalPlanner_SendGoal_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GlobalPlanner_SendGoal_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__GlobalPlanner_SendGoal_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__GlobalPlanner_SendGoal_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__GlobalPlanner_SendGoal_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GlobalPlanner_SendGoal_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GlobalPlanner_SendGoal_Request where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/action/GlobalPlanner_SendGoal_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__GlobalPlanner_SendGoal_Request() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__GlobalPlanner_SendGoal_Response() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__action__GlobalPlanner_SendGoal_Response__init(msg: *mut GlobalPlanner_SendGoal_Response) -> bool;
    fn moveit_msgs__action__GlobalPlanner_SendGoal_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GlobalPlanner_SendGoal_Response>, size: usize) -> bool;
    fn moveit_msgs__action__GlobalPlanner_SendGoal_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GlobalPlanner_SendGoal_Response>);
    fn moveit_msgs__action__GlobalPlanner_SendGoal_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GlobalPlanner_SendGoal_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GlobalPlanner_SendGoal_Response>) -> bool;
}

// Corresponds to moveit_msgs__action__GlobalPlanner_SendGoal_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GlobalPlanner_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::rmw::Time,

}



impl Default for GlobalPlanner_SendGoal_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__action__GlobalPlanner_SendGoal_Response__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__action__GlobalPlanner_SendGoal_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GlobalPlanner_SendGoal_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__GlobalPlanner_SendGoal_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__GlobalPlanner_SendGoal_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__GlobalPlanner_SendGoal_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GlobalPlanner_SendGoal_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GlobalPlanner_SendGoal_Response where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/action/GlobalPlanner_SendGoal_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__GlobalPlanner_SendGoal_Response() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__GlobalPlanner_GetResult_Request() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__action__GlobalPlanner_GetResult_Request__init(msg: *mut GlobalPlanner_GetResult_Request) -> bool;
    fn moveit_msgs__action__GlobalPlanner_GetResult_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GlobalPlanner_GetResult_Request>, size: usize) -> bool;
    fn moveit_msgs__action__GlobalPlanner_GetResult_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GlobalPlanner_GetResult_Request>);
    fn moveit_msgs__action__GlobalPlanner_GetResult_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GlobalPlanner_GetResult_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GlobalPlanner_GetResult_Request>) -> bool;
}

// Corresponds to moveit_msgs__action__GlobalPlanner_GetResult_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GlobalPlanner_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,

}



impl Default for GlobalPlanner_GetResult_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__action__GlobalPlanner_GetResult_Request__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__action__GlobalPlanner_GetResult_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GlobalPlanner_GetResult_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__GlobalPlanner_GetResult_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__GlobalPlanner_GetResult_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__GlobalPlanner_GetResult_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GlobalPlanner_GetResult_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GlobalPlanner_GetResult_Request where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/action/GlobalPlanner_GetResult_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__GlobalPlanner_GetResult_Request() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__GlobalPlanner_GetResult_Response() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__action__GlobalPlanner_GetResult_Response__init(msg: *mut GlobalPlanner_GetResult_Response) -> bool;
    fn moveit_msgs__action__GlobalPlanner_GetResult_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GlobalPlanner_GetResult_Response>, size: usize) -> bool;
    fn moveit_msgs__action__GlobalPlanner_GetResult_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GlobalPlanner_GetResult_Response>);
    fn moveit_msgs__action__GlobalPlanner_GetResult_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GlobalPlanner_GetResult_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GlobalPlanner_GetResult_Response>) -> bool;
}

// Corresponds to moveit_msgs__action__GlobalPlanner_GetResult_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GlobalPlanner_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::super::action::rmw::GlobalPlanner_Result,

}



impl Default for GlobalPlanner_GetResult_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__action__GlobalPlanner_GetResult_Response__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__action__GlobalPlanner_GetResult_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GlobalPlanner_GetResult_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__GlobalPlanner_GetResult_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__GlobalPlanner_GetResult_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__GlobalPlanner_GetResult_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GlobalPlanner_GetResult_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GlobalPlanner_GetResult_Response where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/action/GlobalPlanner_GetResult_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__GlobalPlanner_GetResult_Response() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__HybridPlanner_SendGoal_Request() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__action__HybridPlanner_SendGoal_Request__init(msg: *mut HybridPlanner_SendGoal_Request) -> bool;
    fn moveit_msgs__action__HybridPlanner_SendGoal_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<HybridPlanner_SendGoal_Request>, size: usize) -> bool;
    fn moveit_msgs__action__HybridPlanner_SendGoal_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<HybridPlanner_SendGoal_Request>);
    fn moveit_msgs__action__HybridPlanner_SendGoal_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<HybridPlanner_SendGoal_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<HybridPlanner_SendGoal_Request>) -> bool;
}

// Corresponds to moveit_msgs__action__HybridPlanner_SendGoal_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct HybridPlanner_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::super::action::rmw::HybridPlanner_Goal,

}



impl Default for HybridPlanner_SendGoal_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__action__HybridPlanner_SendGoal_Request__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__action__HybridPlanner_SendGoal_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for HybridPlanner_SendGoal_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__HybridPlanner_SendGoal_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__HybridPlanner_SendGoal_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__HybridPlanner_SendGoal_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for HybridPlanner_SendGoal_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for HybridPlanner_SendGoal_Request where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/action/HybridPlanner_SendGoal_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__HybridPlanner_SendGoal_Request() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__HybridPlanner_SendGoal_Response() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__action__HybridPlanner_SendGoal_Response__init(msg: *mut HybridPlanner_SendGoal_Response) -> bool;
    fn moveit_msgs__action__HybridPlanner_SendGoal_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<HybridPlanner_SendGoal_Response>, size: usize) -> bool;
    fn moveit_msgs__action__HybridPlanner_SendGoal_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<HybridPlanner_SendGoal_Response>);
    fn moveit_msgs__action__HybridPlanner_SendGoal_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<HybridPlanner_SendGoal_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<HybridPlanner_SendGoal_Response>) -> bool;
}

// Corresponds to moveit_msgs__action__HybridPlanner_SendGoal_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct HybridPlanner_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::rmw::Time,

}



impl Default for HybridPlanner_SendGoal_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__action__HybridPlanner_SendGoal_Response__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__action__HybridPlanner_SendGoal_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for HybridPlanner_SendGoal_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__HybridPlanner_SendGoal_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__HybridPlanner_SendGoal_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__HybridPlanner_SendGoal_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for HybridPlanner_SendGoal_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for HybridPlanner_SendGoal_Response where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/action/HybridPlanner_SendGoal_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__HybridPlanner_SendGoal_Response() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__HybridPlanner_GetResult_Request() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__action__HybridPlanner_GetResult_Request__init(msg: *mut HybridPlanner_GetResult_Request) -> bool;
    fn moveit_msgs__action__HybridPlanner_GetResult_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<HybridPlanner_GetResult_Request>, size: usize) -> bool;
    fn moveit_msgs__action__HybridPlanner_GetResult_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<HybridPlanner_GetResult_Request>);
    fn moveit_msgs__action__HybridPlanner_GetResult_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<HybridPlanner_GetResult_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<HybridPlanner_GetResult_Request>) -> bool;
}

// Corresponds to moveit_msgs__action__HybridPlanner_GetResult_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct HybridPlanner_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,

}



impl Default for HybridPlanner_GetResult_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__action__HybridPlanner_GetResult_Request__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__action__HybridPlanner_GetResult_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for HybridPlanner_GetResult_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__HybridPlanner_GetResult_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__HybridPlanner_GetResult_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__HybridPlanner_GetResult_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for HybridPlanner_GetResult_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for HybridPlanner_GetResult_Request where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/action/HybridPlanner_GetResult_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__HybridPlanner_GetResult_Request() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__HybridPlanner_GetResult_Response() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__action__HybridPlanner_GetResult_Response__init(msg: *mut HybridPlanner_GetResult_Response) -> bool;
    fn moveit_msgs__action__HybridPlanner_GetResult_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<HybridPlanner_GetResult_Response>, size: usize) -> bool;
    fn moveit_msgs__action__HybridPlanner_GetResult_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<HybridPlanner_GetResult_Response>);
    fn moveit_msgs__action__HybridPlanner_GetResult_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<HybridPlanner_GetResult_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<HybridPlanner_GetResult_Response>) -> bool;
}

// Corresponds to moveit_msgs__action__HybridPlanner_GetResult_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct HybridPlanner_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::super::action::rmw::HybridPlanner_Result,

}



impl Default for HybridPlanner_GetResult_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__action__HybridPlanner_GetResult_Response__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__action__HybridPlanner_GetResult_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for HybridPlanner_GetResult_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__HybridPlanner_GetResult_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__HybridPlanner_GetResult_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__action__HybridPlanner_GetResult_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for HybridPlanner_GetResult_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for HybridPlanner_GetResult_Response where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/action/HybridPlanner_GetResult_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__action__HybridPlanner_GetResult_Response() }
  }
}






#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__moveit_msgs__action__ExecuteTrajectory_SendGoal() -> *const std::ffi::c_void;
}

// Corresponds to moveit_msgs__action__ExecuteTrajectory_SendGoal
#[allow(missing_docs, non_camel_case_types)]
pub struct ExecuteTrajectory_SendGoal;

impl rosidl_runtime_rs::Service for ExecuteTrajectory_SendGoal {
    type Request = ExecuteTrajectory_SendGoal_Request;
    type Response = ExecuteTrajectory_SendGoal_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__moveit_msgs__action__ExecuteTrajectory_SendGoal() }
    }
}




#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__moveit_msgs__action__ExecuteTrajectory_GetResult() -> *const std::ffi::c_void;
}

// Corresponds to moveit_msgs__action__ExecuteTrajectory_GetResult
#[allow(missing_docs, non_camel_case_types)]
pub struct ExecuteTrajectory_GetResult;

impl rosidl_runtime_rs::Service for ExecuteTrajectory_GetResult {
    type Request = ExecuteTrajectory_GetResult_Request;
    type Response = ExecuteTrajectory_GetResult_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__moveit_msgs__action__ExecuteTrajectory_GetResult() }
    }
}




#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__moveit_msgs__action__MoveGroup_SendGoal() -> *const std::ffi::c_void;
}

// Corresponds to moveit_msgs__action__MoveGroup_SendGoal
#[allow(missing_docs, non_camel_case_types)]
pub struct MoveGroup_SendGoal;

impl rosidl_runtime_rs::Service for MoveGroup_SendGoal {
    type Request = MoveGroup_SendGoal_Request;
    type Response = MoveGroup_SendGoal_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__moveit_msgs__action__MoveGroup_SendGoal() }
    }
}




#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__moveit_msgs__action__MoveGroup_GetResult() -> *const std::ffi::c_void;
}

// Corresponds to moveit_msgs__action__MoveGroup_GetResult
#[allow(missing_docs, non_camel_case_types)]
pub struct MoveGroup_GetResult;

impl rosidl_runtime_rs::Service for MoveGroup_GetResult {
    type Request = MoveGroup_GetResult_Request;
    type Response = MoveGroup_GetResult_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__moveit_msgs__action__MoveGroup_GetResult() }
    }
}




#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__moveit_msgs__action__MoveGroupSequence_SendGoal() -> *const std::ffi::c_void;
}

// Corresponds to moveit_msgs__action__MoveGroupSequence_SendGoal
#[allow(missing_docs, non_camel_case_types)]
pub struct MoveGroupSequence_SendGoal;

impl rosidl_runtime_rs::Service for MoveGroupSequence_SendGoal {
    type Request = MoveGroupSequence_SendGoal_Request;
    type Response = MoveGroupSequence_SendGoal_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__moveit_msgs__action__MoveGroupSequence_SendGoal() }
    }
}




#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__moveit_msgs__action__MoveGroupSequence_GetResult() -> *const std::ffi::c_void;
}

// Corresponds to moveit_msgs__action__MoveGroupSequence_GetResult
#[allow(missing_docs, non_camel_case_types)]
pub struct MoveGroupSequence_GetResult;

impl rosidl_runtime_rs::Service for MoveGroupSequence_GetResult {
    type Request = MoveGroupSequence_GetResult_Request;
    type Response = MoveGroupSequence_GetResult_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__moveit_msgs__action__MoveGroupSequence_GetResult() }
    }
}




#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__moveit_msgs__action__Pickup_SendGoal() -> *const std::ffi::c_void;
}

// Corresponds to moveit_msgs__action__Pickup_SendGoal
#[allow(missing_docs, non_camel_case_types)]
pub struct Pickup_SendGoal;

impl rosidl_runtime_rs::Service for Pickup_SendGoal {
    type Request = Pickup_SendGoal_Request;
    type Response = Pickup_SendGoal_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__moveit_msgs__action__Pickup_SendGoal() }
    }
}




#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__moveit_msgs__action__Pickup_GetResult() -> *const std::ffi::c_void;
}

// Corresponds to moveit_msgs__action__Pickup_GetResult
#[allow(missing_docs, non_camel_case_types)]
pub struct Pickup_GetResult;

impl rosidl_runtime_rs::Service for Pickup_GetResult {
    type Request = Pickup_GetResult_Request;
    type Response = Pickup_GetResult_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__moveit_msgs__action__Pickup_GetResult() }
    }
}




#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__moveit_msgs__action__Place_SendGoal() -> *const std::ffi::c_void;
}

// Corresponds to moveit_msgs__action__Place_SendGoal
#[allow(missing_docs, non_camel_case_types)]
pub struct Place_SendGoal;

impl rosidl_runtime_rs::Service for Place_SendGoal {
    type Request = Place_SendGoal_Request;
    type Response = Place_SendGoal_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__moveit_msgs__action__Place_SendGoal() }
    }
}




#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__moveit_msgs__action__Place_GetResult() -> *const std::ffi::c_void;
}

// Corresponds to moveit_msgs__action__Place_GetResult
#[allow(missing_docs, non_camel_case_types)]
pub struct Place_GetResult;

impl rosidl_runtime_rs::Service for Place_GetResult {
    type Request = Place_GetResult_Request;
    type Response = Place_GetResult_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__moveit_msgs__action__Place_GetResult() }
    }
}




#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__moveit_msgs__action__LocalPlanner_SendGoal() -> *const std::ffi::c_void;
}

// Corresponds to moveit_msgs__action__LocalPlanner_SendGoal
#[allow(missing_docs, non_camel_case_types)]
pub struct LocalPlanner_SendGoal;

impl rosidl_runtime_rs::Service for LocalPlanner_SendGoal {
    type Request = LocalPlanner_SendGoal_Request;
    type Response = LocalPlanner_SendGoal_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__moveit_msgs__action__LocalPlanner_SendGoal() }
    }
}




#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__moveit_msgs__action__LocalPlanner_GetResult() -> *const std::ffi::c_void;
}

// Corresponds to moveit_msgs__action__LocalPlanner_GetResult
#[allow(missing_docs, non_camel_case_types)]
pub struct LocalPlanner_GetResult;

impl rosidl_runtime_rs::Service for LocalPlanner_GetResult {
    type Request = LocalPlanner_GetResult_Request;
    type Response = LocalPlanner_GetResult_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__moveit_msgs__action__LocalPlanner_GetResult() }
    }
}




#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__moveit_msgs__action__GlobalPlanner_SendGoal() -> *const std::ffi::c_void;
}

// Corresponds to moveit_msgs__action__GlobalPlanner_SendGoal
#[allow(missing_docs, non_camel_case_types)]
pub struct GlobalPlanner_SendGoal;

impl rosidl_runtime_rs::Service for GlobalPlanner_SendGoal {
    type Request = GlobalPlanner_SendGoal_Request;
    type Response = GlobalPlanner_SendGoal_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__moveit_msgs__action__GlobalPlanner_SendGoal() }
    }
}




#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__moveit_msgs__action__GlobalPlanner_GetResult() -> *const std::ffi::c_void;
}

// Corresponds to moveit_msgs__action__GlobalPlanner_GetResult
#[allow(missing_docs, non_camel_case_types)]
pub struct GlobalPlanner_GetResult;

impl rosidl_runtime_rs::Service for GlobalPlanner_GetResult {
    type Request = GlobalPlanner_GetResult_Request;
    type Response = GlobalPlanner_GetResult_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__moveit_msgs__action__GlobalPlanner_GetResult() }
    }
}




#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__moveit_msgs__action__HybridPlanner_SendGoal() -> *const std::ffi::c_void;
}

// Corresponds to moveit_msgs__action__HybridPlanner_SendGoal
#[allow(missing_docs, non_camel_case_types)]
pub struct HybridPlanner_SendGoal;

impl rosidl_runtime_rs::Service for HybridPlanner_SendGoal {
    type Request = HybridPlanner_SendGoal_Request;
    type Response = HybridPlanner_SendGoal_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__moveit_msgs__action__HybridPlanner_SendGoal() }
    }
}




#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__moveit_msgs__action__HybridPlanner_GetResult() -> *const std::ffi::c_void;
}

// Corresponds to moveit_msgs__action__HybridPlanner_GetResult
#[allow(missing_docs, non_camel_case_types)]
pub struct HybridPlanner_GetResult;

impl rosidl_runtime_rs::Service for HybridPlanner_GetResult {
    type Request = HybridPlanner_GetResult_Request;
    type Response = HybridPlanner_GetResult_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__moveit_msgs__action__HybridPlanner_GetResult() }
    }
}


