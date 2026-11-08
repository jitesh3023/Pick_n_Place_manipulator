#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__GetMotionPlan_Request() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__srv__GetMotionPlan_Request__init(msg: *mut GetMotionPlan_Request) -> bool;
    fn moveit_msgs__srv__GetMotionPlan_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetMotionPlan_Request>, size: usize) -> bool;
    fn moveit_msgs__srv__GetMotionPlan_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetMotionPlan_Request>);
    fn moveit_msgs__srv__GetMotionPlan_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetMotionPlan_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GetMotionPlan_Request>) -> bool;
}

// Corresponds to moveit_msgs__srv__GetMotionPlan_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetMotionPlan_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub motion_plan_request: super::super::msg::rmw::MotionPlanRequest,

}



impl Default for GetMotionPlan_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__srv__GetMotionPlan_Request__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__srv__GetMotionPlan_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetMotionPlan_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__GetMotionPlan_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__GetMotionPlan_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__GetMotionPlan_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetMotionPlan_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetMotionPlan_Request where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/srv/GetMotionPlan_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__GetMotionPlan_Request() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__GetMotionPlan_Response() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__srv__GetMotionPlan_Response__init(msg: *mut GetMotionPlan_Response) -> bool;
    fn moveit_msgs__srv__GetMotionPlan_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetMotionPlan_Response>, size: usize) -> bool;
    fn moveit_msgs__srv__GetMotionPlan_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetMotionPlan_Response>);
    fn moveit_msgs__srv__GetMotionPlan_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetMotionPlan_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GetMotionPlan_Response>) -> bool;
}

// Corresponds to moveit_msgs__srv__GetMotionPlan_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetMotionPlan_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub motion_plan_response: super::super::msg::rmw::MotionPlanResponse,

}



impl Default for GetMotionPlan_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__srv__GetMotionPlan_Response__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__srv__GetMotionPlan_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetMotionPlan_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__GetMotionPlan_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__GetMotionPlan_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__GetMotionPlan_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetMotionPlan_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetMotionPlan_Response where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/srv/GetMotionPlan_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__GetMotionPlan_Response() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__GetMultiStateValidity_Request() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__srv__GetMultiStateValidity_Request__init(msg: *mut GetMultiStateValidity_Request) -> bool;
    fn moveit_msgs__srv__GetMultiStateValidity_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetMultiStateValidity_Request>, size: usize) -> bool;
    fn moveit_msgs__srv__GetMultiStateValidity_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetMultiStateValidity_Request>);
    fn moveit_msgs__srv__GetMultiStateValidity_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetMultiStateValidity_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GetMultiStateValidity_Request>) -> bool;
}

// Corresponds to moveit_msgs__srv__GetMultiStateValidity_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetMultiStateValidity_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub robot_state: super::super::msg::rmw::RobotState,


    // This member is not documented.
    #[allow(missing_docs)]
    pub joint_states: rosidl_runtime_rs::Sequence<sensor_msgs::msg::rmw::JointState>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub group_name: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub constraints: super::super::msg::rmw::Constraints,

}



impl Default for GetMultiStateValidity_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__srv__GetMultiStateValidity_Request__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__srv__GetMultiStateValidity_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetMultiStateValidity_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__GetMultiStateValidity_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__GetMultiStateValidity_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__GetMultiStateValidity_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetMultiStateValidity_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetMultiStateValidity_Request where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/srv/GetMultiStateValidity_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__GetMultiStateValidity_Request() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__GetMultiStateValidity_Response() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__srv__GetMultiStateValidity_Response__init(msg: *mut GetMultiStateValidity_Response) -> bool;
    fn moveit_msgs__srv__GetMultiStateValidity_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetMultiStateValidity_Response>, size: usize) -> bool;
    fn moveit_msgs__srv__GetMultiStateValidity_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetMultiStateValidity_Response>);
    fn moveit_msgs__srv__GetMultiStateValidity_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetMultiStateValidity_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GetMultiStateValidity_Response>) -> bool;
}

// Corresponds to moveit_msgs__srv__GetMultiStateValidity_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetMultiStateValidity_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub valid: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub contacts: rosidl_runtime_rs::Sequence<super::super::msg::rmw::ContactInformation>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub cost_sources: rosidl_runtime_rs::Sequence<super::super::msg::rmw::CostSource>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub constraint_result: rosidl_runtime_rs::Sequence<super::super::msg::rmw::ConstraintEvalResult>,

}



impl Default for GetMultiStateValidity_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__srv__GetMultiStateValidity_Response__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__srv__GetMultiStateValidity_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetMultiStateValidity_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__GetMultiStateValidity_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__GetMultiStateValidity_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__GetMultiStateValidity_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetMultiStateValidity_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetMultiStateValidity_Response where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/srv/GetMultiStateValidity_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__GetMultiStateValidity_Response() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__GetStateValidity_Request() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__srv__GetStateValidity_Request__init(msg: *mut GetStateValidity_Request) -> bool;
    fn moveit_msgs__srv__GetStateValidity_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetStateValidity_Request>, size: usize) -> bool;
    fn moveit_msgs__srv__GetStateValidity_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetStateValidity_Request>);
    fn moveit_msgs__srv__GetStateValidity_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetStateValidity_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GetStateValidity_Request>) -> bool;
}

// Corresponds to moveit_msgs__srv__GetStateValidity_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetStateValidity_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub robot_state: super::super::msg::rmw::RobotState,


    // This member is not documented.
    #[allow(missing_docs)]
    pub group_name: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub constraints: super::super::msg::rmw::Constraints,

}



impl Default for GetStateValidity_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__srv__GetStateValidity_Request__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__srv__GetStateValidity_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetStateValidity_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__GetStateValidity_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__GetStateValidity_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__GetStateValidity_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetStateValidity_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetStateValidity_Request where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/srv/GetStateValidity_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__GetStateValidity_Request() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__GetStateValidity_Response() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__srv__GetStateValidity_Response__init(msg: *mut GetStateValidity_Response) -> bool;
    fn moveit_msgs__srv__GetStateValidity_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetStateValidity_Response>, size: usize) -> bool;
    fn moveit_msgs__srv__GetStateValidity_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetStateValidity_Response>);
    fn moveit_msgs__srv__GetStateValidity_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetStateValidity_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GetStateValidity_Response>) -> bool;
}

// Corresponds to moveit_msgs__srv__GetStateValidity_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetStateValidity_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub valid: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub contacts: rosidl_runtime_rs::Sequence<super::super::msg::rmw::ContactInformation>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub cost_sources: rosidl_runtime_rs::Sequence<super::super::msg::rmw::CostSource>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub constraint_result: rosidl_runtime_rs::Sequence<super::super::msg::rmw::ConstraintEvalResult>,

}



impl Default for GetStateValidity_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__srv__GetStateValidity_Response__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__srv__GetStateValidity_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetStateValidity_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__GetStateValidity_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__GetStateValidity_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__GetStateValidity_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetStateValidity_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetStateValidity_Response where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/srv/GetStateValidity_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__GetStateValidity_Response() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__GetCartesianPath_Request() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__srv__GetCartesianPath_Request__init(msg: *mut GetCartesianPath_Request) -> bool;
    fn moveit_msgs__srv__GetCartesianPath_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetCartesianPath_Request>, size: usize) -> bool;
    fn moveit_msgs__srv__GetCartesianPath_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetCartesianPath_Request>);
    fn moveit_msgs__srv__GetCartesianPath_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetCartesianPath_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GetCartesianPath_Request>) -> bool;
}

// Corresponds to moveit_msgs__srv__GetCartesianPath_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetCartesianPath_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,

    /// The start at which to start the Cartesian path
    pub start_state: super::super::msg::rmw::RobotState,

    /// Mandatory name of group to compute the path for
    pub group_name: rosidl_runtime_rs::String,

    /// Optional name of IK link for which waypoints are specified.
    /// If not specified, the tip of the group (which is assumed to be a chain)
    /// is assumed to be the link
    pub link_name: rosidl_runtime_rs::String,

    /// A sequence of waypoints to be followed by the specified link,
    /// while moving the specified group, such that the group moves only
    /// in a straight line between waypoints
    pub waypoints: rosidl_runtime_rs::Sequence<geometry_msgs::msg::rmw::Pose>,

    /// The maximum distance (in Cartesian space) between consecutive points
    /// in the returned path. This must always be specified and > 0
    pub max_step: f64,

    /// If jump_threshold is set > 0, it acts as a scaling factor that is used to
    /// filter out large relative joint-space jumps in the generated Cartesian path.
    /// To this end, the average joint-space distance between consecutive waypoints
    /// is computed. If any joint-space distance is larger than this average distance
    /// by a factor of jump_threshold_factor, this step is considered a jump
    /// and the returned path is truncated before the step.
    pub jump_threshold: f64,

    /// If prismatic_jump_threshold or revolute_jump_threshold are set > 0, then for
    /// all active prismatic or revolute joints, the joint-space difference between
    /// consecutive waypoints is compared to the respective absolute threshold.
    /// If any threshold is exceeded, this step is considered a jump and the returned path
    /// is truncated before the step.
    pub prismatic_jump_threshold: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub revolute_jump_threshold: f64,

    /// Set to true if collisions should be avoided when possible
    pub avoid_collisions: bool,

    /// Specify additional constraints to be met by the Cartesian path
    pub path_constraints: super::super::msg::rmw::Constraints,

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
    pub cartesian_speed_limited_link: rosidl_runtime_rs::String,

    /// m/s
    pub max_cartesian_speed: f64,

}



impl Default for GetCartesianPath_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__srv__GetCartesianPath_Request__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__srv__GetCartesianPath_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetCartesianPath_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__GetCartesianPath_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__GetCartesianPath_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__GetCartesianPath_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetCartesianPath_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetCartesianPath_Request where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/srv/GetCartesianPath_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__GetCartesianPath_Request() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__GetCartesianPath_Response() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__srv__GetCartesianPath_Response__init(msg: *mut GetCartesianPath_Response) -> bool;
    fn moveit_msgs__srv__GetCartesianPath_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetCartesianPath_Response>, size: usize) -> bool;
    fn moveit_msgs__srv__GetCartesianPath_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetCartesianPath_Response>);
    fn moveit_msgs__srv__GetCartesianPath_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetCartesianPath_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GetCartesianPath_Response>) -> bool;
}

// Corresponds to moveit_msgs__srv__GetCartesianPath_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetCartesianPath_Response {
    /// The state at which the computed path starts
    pub start_state: super::super::msg::rmw::RobotState,

    /// The computed solution trajectory, for the desired group, in configuration space
    pub solution: super::super::msg::rmw::RobotTrajectory,

    /// If the computation was incomplete, this value indicates the fraction of the path
    /// that was in fact computed (number of waypoints traveled through)
    pub fraction: f64,

    /// The error code of the computation
    pub error_code: super::super::msg::rmw::MoveItErrorCodes,

}



impl Default for GetCartesianPath_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__srv__GetCartesianPath_Response__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__srv__GetCartesianPath_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetCartesianPath_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__GetCartesianPath_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__GetCartesianPath_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__GetCartesianPath_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetCartesianPath_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetCartesianPath_Response where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/srv/GetCartesianPath_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__GetCartesianPath_Response() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__GetPlanningScene_Request() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__srv__GetPlanningScene_Request__init(msg: *mut GetPlanningScene_Request) -> bool;
    fn moveit_msgs__srv__GetPlanningScene_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetPlanningScene_Request>, size: usize) -> bool;
    fn moveit_msgs__srv__GetPlanningScene_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetPlanningScene_Request>);
    fn moveit_msgs__srv__GetPlanningScene_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetPlanningScene_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GetPlanningScene_Request>) -> bool;
}

// Corresponds to moveit_msgs__srv__GetPlanningScene_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetPlanningScene_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub components: super::super::msg::rmw::PlanningSceneComponents,

}



impl Default for GetPlanningScene_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__srv__GetPlanningScene_Request__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__srv__GetPlanningScene_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetPlanningScene_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__GetPlanningScene_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__GetPlanningScene_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__GetPlanningScene_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetPlanningScene_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetPlanningScene_Request where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/srv/GetPlanningScene_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__GetPlanningScene_Request() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__GetPlanningScene_Response() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__srv__GetPlanningScene_Response__init(msg: *mut GetPlanningScene_Response) -> bool;
    fn moveit_msgs__srv__GetPlanningScene_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetPlanningScene_Response>, size: usize) -> bool;
    fn moveit_msgs__srv__GetPlanningScene_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetPlanningScene_Response>);
    fn moveit_msgs__srv__GetPlanningScene_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetPlanningScene_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GetPlanningScene_Response>) -> bool;
}

// Corresponds to moveit_msgs__srv__GetPlanningScene_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetPlanningScene_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub scene: super::super::msg::rmw::PlanningScene,

}



impl Default for GetPlanningScene_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__srv__GetPlanningScene_Response__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__srv__GetPlanningScene_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetPlanningScene_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__GetPlanningScene_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__GetPlanningScene_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__GetPlanningScene_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetPlanningScene_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetPlanningScene_Response where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/srv/GetPlanningScene_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__GetPlanningScene_Response() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__GraspPlanning_Request() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__srv__GraspPlanning_Request__init(msg: *mut GraspPlanning_Request) -> bool;
    fn moveit_msgs__srv__GraspPlanning_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GraspPlanning_Request>, size: usize) -> bool;
    fn moveit_msgs__srv__GraspPlanning_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GraspPlanning_Request>);
    fn moveit_msgs__srv__GraspPlanning_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GraspPlanning_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GraspPlanning_Request>) -> bool;
}

// Corresponds to moveit_msgs__srv__GraspPlanning_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GraspPlanning_Request {
    /// the planning group used
    pub group_name: rosidl_runtime_rs::String,

    /// the object to be grasped
    pub target: super::super::msg::rmw::CollisionObject,

    /// the names of the relevant support surfaces (e.g. tables) in the collision map
    /// can be left empty if no names are available
    pub support_surfaces: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,

    /// an optional list of grasps to be evaluated by the planner
    pub candidate_grasps: rosidl_runtime_rs::Sequence<super::super::msg::rmw::Grasp>,

    /// an optional list of obstacles that we have semantic information about
    /// and that can be moved in the course of grasping
    pub movable_obstacles: rosidl_runtime_rs::Sequence<super::super::msg::rmw::CollisionObject>,

}



impl Default for GraspPlanning_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__srv__GraspPlanning_Request__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__srv__GraspPlanning_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GraspPlanning_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__GraspPlanning_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__GraspPlanning_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__GraspPlanning_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GraspPlanning_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GraspPlanning_Request where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/srv/GraspPlanning_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__GraspPlanning_Request() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__GraspPlanning_Response() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__srv__GraspPlanning_Response__init(msg: *mut GraspPlanning_Response) -> bool;
    fn moveit_msgs__srv__GraspPlanning_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GraspPlanning_Response>, size: usize) -> bool;
    fn moveit_msgs__srv__GraspPlanning_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GraspPlanning_Response>);
    fn moveit_msgs__srv__GraspPlanning_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GraspPlanning_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GraspPlanning_Response>) -> bool;
}

// Corresponds to moveit_msgs__srv__GraspPlanning_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GraspPlanning_Response {
    /// the list of planned grasps
    pub grasps: rosidl_runtime_rs::Sequence<super::super::msg::rmw::Grasp>,

    /// whether an error occurred
    pub error_code: super::super::msg::rmw::MoveItErrorCodes,

}



impl Default for GraspPlanning_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__srv__GraspPlanning_Response__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__srv__GraspPlanning_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GraspPlanning_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__GraspPlanning_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__GraspPlanning_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__GraspPlanning_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GraspPlanning_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GraspPlanning_Response where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/srv/GraspPlanning_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__GraspPlanning_Response() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__ApplyPlanningScene_Request() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__srv__ApplyPlanningScene_Request__init(msg: *mut ApplyPlanningScene_Request) -> bool;
    fn moveit_msgs__srv__ApplyPlanningScene_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ApplyPlanningScene_Request>, size: usize) -> bool;
    fn moveit_msgs__srv__ApplyPlanningScene_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ApplyPlanningScene_Request>);
    fn moveit_msgs__srv__ApplyPlanningScene_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ApplyPlanningScene_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<ApplyPlanningScene_Request>) -> bool;
}

// Corresponds to moveit_msgs__srv__ApplyPlanningScene_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ApplyPlanningScene_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub scene: super::super::msg::rmw::PlanningScene,

}



impl Default for ApplyPlanningScene_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__srv__ApplyPlanningScene_Request__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__srv__ApplyPlanningScene_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ApplyPlanningScene_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__ApplyPlanningScene_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__ApplyPlanningScene_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__ApplyPlanningScene_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ApplyPlanningScene_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ApplyPlanningScene_Request where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/srv/ApplyPlanningScene_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__ApplyPlanningScene_Request() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__ApplyPlanningScene_Response() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__srv__ApplyPlanningScene_Response__init(msg: *mut ApplyPlanningScene_Response) -> bool;
    fn moveit_msgs__srv__ApplyPlanningScene_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ApplyPlanningScene_Response>, size: usize) -> bool;
    fn moveit_msgs__srv__ApplyPlanningScene_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ApplyPlanningScene_Response>);
    fn moveit_msgs__srv__ApplyPlanningScene_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ApplyPlanningScene_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<ApplyPlanningScene_Response>) -> bool;
}

// Corresponds to moveit_msgs__srv__ApplyPlanningScene_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ApplyPlanningScene_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,

}



impl Default for ApplyPlanningScene_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__srv__ApplyPlanningScene_Response__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__srv__ApplyPlanningScene_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ApplyPlanningScene_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__ApplyPlanningScene_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__ApplyPlanningScene_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__ApplyPlanningScene_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ApplyPlanningScene_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ApplyPlanningScene_Response where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/srv/ApplyPlanningScene_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__ApplyPlanningScene_Response() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__QueryPlannerInterfaces_Request() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__srv__QueryPlannerInterfaces_Request__init(msg: *mut QueryPlannerInterfaces_Request) -> bool;
    fn moveit_msgs__srv__QueryPlannerInterfaces_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<QueryPlannerInterfaces_Request>, size: usize) -> bool;
    fn moveit_msgs__srv__QueryPlannerInterfaces_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<QueryPlannerInterfaces_Request>);
    fn moveit_msgs__srv__QueryPlannerInterfaces_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<QueryPlannerInterfaces_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<QueryPlannerInterfaces_Request>) -> bool;
}

// Corresponds to moveit_msgs__srv__QueryPlannerInterfaces_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct QueryPlannerInterfaces_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for QueryPlannerInterfaces_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__srv__QueryPlannerInterfaces_Request__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__srv__QueryPlannerInterfaces_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for QueryPlannerInterfaces_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__QueryPlannerInterfaces_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__QueryPlannerInterfaces_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__QueryPlannerInterfaces_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for QueryPlannerInterfaces_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for QueryPlannerInterfaces_Request where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/srv/QueryPlannerInterfaces_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__QueryPlannerInterfaces_Request() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__QueryPlannerInterfaces_Response() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__srv__QueryPlannerInterfaces_Response__init(msg: *mut QueryPlannerInterfaces_Response) -> bool;
    fn moveit_msgs__srv__QueryPlannerInterfaces_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<QueryPlannerInterfaces_Response>, size: usize) -> bool;
    fn moveit_msgs__srv__QueryPlannerInterfaces_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<QueryPlannerInterfaces_Response>);
    fn moveit_msgs__srv__QueryPlannerInterfaces_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<QueryPlannerInterfaces_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<QueryPlannerInterfaces_Response>) -> bool;
}

// Corresponds to moveit_msgs__srv__QueryPlannerInterfaces_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct QueryPlannerInterfaces_Response {
    /// The planning instances that could be used in the benchmark
    pub planner_interfaces: rosidl_runtime_rs::Sequence<super::super::msg::rmw::PlannerInterfaceDescription>,

}



impl Default for QueryPlannerInterfaces_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__srv__QueryPlannerInterfaces_Response__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__srv__QueryPlannerInterfaces_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for QueryPlannerInterfaces_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__QueryPlannerInterfaces_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__QueryPlannerInterfaces_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__QueryPlannerInterfaces_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for QueryPlannerInterfaces_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for QueryPlannerInterfaces_Response where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/srv/QueryPlannerInterfaces_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__QueryPlannerInterfaces_Response() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__GetMotionSequence_Request() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__srv__GetMotionSequence_Request__init(msg: *mut GetMotionSequence_Request) -> bool;
    fn moveit_msgs__srv__GetMotionSequence_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetMotionSequence_Request>, size: usize) -> bool;
    fn moveit_msgs__srv__GetMotionSequence_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetMotionSequence_Request>);
    fn moveit_msgs__srv__GetMotionSequence_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetMotionSequence_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GetMotionSequence_Request>) -> bool;
}

// Corresponds to moveit_msgs__srv__GetMotionSequence_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetMotionSequence_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub request: super::super::msg::rmw::MotionSequenceRequest,

}



impl Default for GetMotionSequence_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__srv__GetMotionSequence_Request__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__srv__GetMotionSequence_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetMotionSequence_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__GetMotionSequence_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__GetMotionSequence_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__GetMotionSequence_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetMotionSequence_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetMotionSequence_Request where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/srv/GetMotionSequence_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__GetMotionSequence_Request() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__GetMotionSequence_Response() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__srv__GetMotionSequence_Response__init(msg: *mut GetMotionSequence_Response) -> bool;
    fn moveit_msgs__srv__GetMotionSequence_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetMotionSequence_Response>, size: usize) -> bool;
    fn moveit_msgs__srv__GetMotionSequence_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetMotionSequence_Response>);
    fn moveit_msgs__srv__GetMotionSequence_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetMotionSequence_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GetMotionSequence_Response>) -> bool;
}

// Corresponds to moveit_msgs__srv__GetMotionSequence_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetMotionSequence_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub response: super::super::msg::rmw::MotionSequenceResponse,

}



impl Default for GetMotionSequence_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__srv__GetMotionSequence_Response__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__srv__GetMotionSequence_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetMotionSequence_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__GetMotionSequence_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__GetMotionSequence_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__GetMotionSequence_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetMotionSequence_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetMotionSequence_Response where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/srv/GetMotionSequence_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__GetMotionSequence_Response() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__GetPositionFK_Request() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__srv__GetPositionFK_Request__init(msg: *mut GetPositionFK_Request) -> bool;
    fn moveit_msgs__srv__GetPositionFK_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetPositionFK_Request>, size: usize) -> bool;
    fn moveit_msgs__srv__GetPositionFK_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetPositionFK_Request>);
    fn moveit_msgs__srv__GetPositionFK_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetPositionFK_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GetPositionFK_Request>) -> bool;
}

// Corresponds to moveit_msgs__srv__GetPositionFK_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetPositionFK_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,

    /// A vector of link name for which forward kinematics must be computed
    pub fk_link_names: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,

    /// A robot state consisting of joint names and joint positions to be used for forward kinematics
    pub robot_state: super::super::msg::rmw::RobotState,

}



impl Default for GetPositionFK_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__srv__GetPositionFK_Request__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__srv__GetPositionFK_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetPositionFK_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__GetPositionFK_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__GetPositionFK_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__GetPositionFK_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetPositionFK_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetPositionFK_Request where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/srv/GetPositionFK_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__GetPositionFK_Request() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__GetPositionFK_Response() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__srv__GetPositionFK_Response__init(msg: *mut GetPositionFK_Response) -> bool;
    fn moveit_msgs__srv__GetPositionFK_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetPositionFK_Response>, size: usize) -> bool;
    fn moveit_msgs__srv__GetPositionFK_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetPositionFK_Response>);
    fn moveit_msgs__srv__GetPositionFK_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetPositionFK_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GetPositionFK_Response>) -> bool;
}

// Corresponds to moveit_msgs__srv__GetPositionFK_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetPositionFK_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub pose_stamped: rosidl_runtime_rs::Sequence<geometry_msgs::msg::rmw::PoseStamped>,

    /// The list of link names corresponding to the poses
    pub fk_link_names: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub error_code: super::super::msg::rmw::MoveItErrorCodes,

}



impl Default for GetPositionFK_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__srv__GetPositionFK_Response__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__srv__GetPositionFK_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetPositionFK_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__GetPositionFK_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__GetPositionFK_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__GetPositionFK_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetPositionFK_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetPositionFK_Response where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/srv/GetPositionFK_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__GetPositionFK_Response() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__GetPositionIK_Request() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__srv__GetPositionIK_Request__init(msg: *mut GetPositionIK_Request) -> bool;
    fn moveit_msgs__srv__GetPositionIK_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetPositionIK_Request>, size: usize) -> bool;
    fn moveit_msgs__srv__GetPositionIK_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetPositionIK_Request>);
    fn moveit_msgs__srv__GetPositionIK_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetPositionIK_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GetPositionIK_Request>) -> bool;
}

// Corresponds to moveit_msgs__srv__GetPositionIK_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetPositionIK_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub ik_request: super::super::msg::rmw::PositionIKRequest,

}



impl Default for GetPositionIK_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__srv__GetPositionIK_Request__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__srv__GetPositionIK_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetPositionIK_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__GetPositionIK_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__GetPositionIK_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__GetPositionIK_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetPositionIK_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetPositionIK_Request where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/srv/GetPositionIK_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__GetPositionIK_Request() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__GetPositionIK_Response() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__srv__GetPositionIK_Response__init(msg: *mut GetPositionIK_Response) -> bool;
    fn moveit_msgs__srv__GetPositionIK_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetPositionIK_Response>, size: usize) -> bool;
    fn moveit_msgs__srv__GetPositionIK_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetPositionIK_Response>);
    fn moveit_msgs__srv__GetPositionIK_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetPositionIK_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GetPositionIK_Response>) -> bool;
}

// Corresponds to moveit_msgs__srv__GetPositionIK_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetPositionIK_Response {
    /// The returned solution
    /// (in the same order as the list of joints specified in the IKRequest message)
    pub solution: super::super::msg::rmw::RobotState,


    // This member is not documented.
    #[allow(missing_docs)]
    pub error_code: super::super::msg::rmw::MoveItErrorCodes,

}



impl Default for GetPositionIK_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__srv__GetPositionIK_Response__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__srv__GetPositionIK_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetPositionIK_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__GetPositionIK_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__GetPositionIK_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__GetPositionIK_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetPositionIK_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetPositionIK_Response where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/srv/GetPositionIK_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__GetPositionIK_Response() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__GetPlannerParams_Request() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__srv__GetPlannerParams_Request__init(msg: *mut GetPlannerParams_Request) -> bool;
    fn moveit_msgs__srv__GetPlannerParams_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetPlannerParams_Request>, size: usize) -> bool;
    fn moveit_msgs__srv__GetPlannerParams_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetPlannerParams_Request>);
    fn moveit_msgs__srv__GetPlannerParams_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetPlannerParams_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GetPlannerParams_Request>) -> bool;
}

// Corresponds to moveit_msgs__srv__GetPlannerParams_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetPlannerParams_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub pipeline_id: rosidl_runtime_rs::String,

    /// Name of planning config
    pub planner_config: rosidl_runtime_rs::String,

    /// Optional name of planning group (return global defaults if empty)
    pub group: rosidl_runtime_rs::String,

}



impl Default for GetPlannerParams_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__srv__GetPlannerParams_Request__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__srv__GetPlannerParams_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetPlannerParams_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__GetPlannerParams_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__GetPlannerParams_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__GetPlannerParams_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetPlannerParams_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetPlannerParams_Request where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/srv/GetPlannerParams_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__GetPlannerParams_Request() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__GetPlannerParams_Response() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__srv__GetPlannerParams_Response__init(msg: *mut GetPlannerParams_Response) -> bool;
    fn moveit_msgs__srv__GetPlannerParams_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetPlannerParams_Response>, size: usize) -> bool;
    fn moveit_msgs__srv__GetPlannerParams_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetPlannerParams_Response>);
    fn moveit_msgs__srv__GetPlannerParams_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetPlannerParams_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GetPlannerParams_Response>) -> bool;
}

// Corresponds to moveit_msgs__srv__GetPlannerParams_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetPlannerParams_Response {
    /// parameters as key-value pairs
    pub params: super::super::msg::rmw::PlannerParams,

}



impl Default for GetPlannerParams_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__srv__GetPlannerParams_Response__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__srv__GetPlannerParams_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetPlannerParams_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__GetPlannerParams_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__GetPlannerParams_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__GetPlannerParams_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetPlannerParams_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetPlannerParams_Response where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/srv/GetPlannerParams_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__GetPlannerParams_Response() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__SetPlannerParams_Request() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__srv__SetPlannerParams_Request__init(msg: *mut SetPlannerParams_Request) -> bool;
    fn moveit_msgs__srv__SetPlannerParams_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetPlannerParams_Request>, size: usize) -> bool;
    fn moveit_msgs__srv__SetPlannerParams_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetPlannerParams_Request>);
    fn moveit_msgs__srv__SetPlannerParams_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetPlannerParams_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SetPlannerParams_Request>) -> bool;
}

// Corresponds to moveit_msgs__srv__SetPlannerParams_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetPlannerParams_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub pipeline_id: rosidl_runtime_rs::String,

    /// Name of planning config
    pub planner_config: rosidl_runtime_rs::String,

    /// Optional name of planning group (set global defaults if empty)
    pub group: rosidl_runtime_rs::String,

    /// parameters as key-value pairs
    pub params: super::super::msg::rmw::PlannerParams,

    /// replace params or augment existing ones?
    pub replace: bool,

}



impl Default for SetPlannerParams_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__srv__SetPlannerParams_Request__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__srv__SetPlannerParams_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetPlannerParams_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__SetPlannerParams_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__SetPlannerParams_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__SetPlannerParams_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetPlannerParams_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetPlannerParams_Request where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/srv/SetPlannerParams_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__SetPlannerParams_Request() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__SetPlannerParams_Response() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__srv__SetPlannerParams_Response__init(msg: *mut SetPlannerParams_Response) -> bool;
    fn moveit_msgs__srv__SetPlannerParams_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetPlannerParams_Response>, size: usize) -> bool;
    fn moveit_msgs__srv__SetPlannerParams_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetPlannerParams_Response>);
    fn moveit_msgs__srv__SetPlannerParams_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetPlannerParams_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SetPlannerParams_Response>) -> bool;
}

// Corresponds to moveit_msgs__srv__SetPlannerParams_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetPlannerParams_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for SetPlannerParams_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__srv__SetPlannerParams_Response__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__srv__SetPlannerParams_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetPlannerParams_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__SetPlannerParams_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__SetPlannerParams_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__SetPlannerParams_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetPlannerParams_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetPlannerParams_Response where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/srv/SetPlannerParams_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__SetPlannerParams_Response() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__UpdatePointcloudOctomap_Request() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__srv__UpdatePointcloudOctomap_Request__init(msg: *mut UpdatePointcloudOctomap_Request) -> bool;
    fn moveit_msgs__srv__UpdatePointcloudOctomap_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<UpdatePointcloudOctomap_Request>, size: usize) -> bool;
    fn moveit_msgs__srv__UpdatePointcloudOctomap_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<UpdatePointcloudOctomap_Request>);
    fn moveit_msgs__srv__UpdatePointcloudOctomap_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<UpdatePointcloudOctomap_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<UpdatePointcloudOctomap_Request>) -> bool;
}

// Corresponds to moveit_msgs__srv__UpdatePointcloudOctomap_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct UpdatePointcloudOctomap_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub cloud: sensor_msgs::msg::rmw::PointCloud2,

}



impl Default for UpdatePointcloudOctomap_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__srv__UpdatePointcloudOctomap_Request__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__srv__UpdatePointcloudOctomap_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for UpdatePointcloudOctomap_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__UpdatePointcloudOctomap_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__UpdatePointcloudOctomap_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__UpdatePointcloudOctomap_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for UpdatePointcloudOctomap_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for UpdatePointcloudOctomap_Request where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/srv/UpdatePointcloudOctomap_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__UpdatePointcloudOctomap_Request() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__UpdatePointcloudOctomap_Response() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__srv__UpdatePointcloudOctomap_Response__init(msg: *mut UpdatePointcloudOctomap_Response) -> bool;
    fn moveit_msgs__srv__UpdatePointcloudOctomap_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<UpdatePointcloudOctomap_Response>, size: usize) -> bool;
    fn moveit_msgs__srv__UpdatePointcloudOctomap_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<UpdatePointcloudOctomap_Response>);
    fn moveit_msgs__srv__UpdatePointcloudOctomap_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<UpdatePointcloudOctomap_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<UpdatePointcloudOctomap_Response>) -> bool;
}

// Corresponds to moveit_msgs__srv__UpdatePointcloudOctomap_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct UpdatePointcloudOctomap_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,

}



impl Default for UpdatePointcloudOctomap_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__srv__UpdatePointcloudOctomap_Response__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__srv__UpdatePointcloudOctomap_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for UpdatePointcloudOctomap_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__UpdatePointcloudOctomap_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__UpdatePointcloudOctomap_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__UpdatePointcloudOctomap_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for UpdatePointcloudOctomap_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for UpdatePointcloudOctomap_Response where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/srv/UpdatePointcloudOctomap_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__UpdatePointcloudOctomap_Response() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__SaveGeometryToFile_Request() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__srv__SaveGeometryToFile_Request__init(msg: *mut SaveGeometryToFile_Request) -> bool;
    fn moveit_msgs__srv__SaveGeometryToFile_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SaveGeometryToFile_Request>, size: usize) -> bool;
    fn moveit_msgs__srv__SaveGeometryToFile_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SaveGeometryToFile_Request>);
    fn moveit_msgs__srv__SaveGeometryToFile_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SaveGeometryToFile_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SaveGeometryToFile_Request>) -> bool;
}

// Corresponds to moveit_msgs__srv__SaveGeometryToFile_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SaveGeometryToFile_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub file_path_and_name: rosidl_runtime_rs::String,

}



impl Default for SaveGeometryToFile_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__srv__SaveGeometryToFile_Request__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__srv__SaveGeometryToFile_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SaveGeometryToFile_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__SaveGeometryToFile_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__SaveGeometryToFile_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__SaveGeometryToFile_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SaveGeometryToFile_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SaveGeometryToFile_Request where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/srv/SaveGeometryToFile_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__SaveGeometryToFile_Request() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__SaveGeometryToFile_Response() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__srv__SaveGeometryToFile_Response__init(msg: *mut SaveGeometryToFile_Response) -> bool;
    fn moveit_msgs__srv__SaveGeometryToFile_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SaveGeometryToFile_Response>, size: usize) -> bool;
    fn moveit_msgs__srv__SaveGeometryToFile_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SaveGeometryToFile_Response>);
    fn moveit_msgs__srv__SaveGeometryToFile_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SaveGeometryToFile_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SaveGeometryToFile_Response>) -> bool;
}

// Corresponds to moveit_msgs__srv__SaveGeometryToFile_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SaveGeometryToFile_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,

}



impl Default for SaveGeometryToFile_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__srv__SaveGeometryToFile_Response__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__srv__SaveGeometryToFile_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SaveGeometryToFile_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__SaveGeometryToFile_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__SaveGeometryToFile_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__SaveGeometryToFile_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SaveGeometryToFile_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SaveGeometryToFile_Response where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/srv/SaveGeometryToFile_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__SaveGeometryToFile_Response() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__SaveMap_Request() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__srv__SaveMap_Request__init(msg: *mut SaveMap_Request) -> bool;
    fn moveit_msgs__srv__SaveMap_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SaveMap_Request>, size: usize) -> bool;
    fn moveit_msgs__srv__SaveMap_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SaveMap_Request>);
    fn moveit_msgs__srv__SaveMap_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SaveMap_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SaveMap_Request>) -> bool;
}

// Corresponds to moveit_msgs__srv__SaveMap_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SaveMap_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub filename: rosidl_runtime_rs::String,

}



impl Default for SaveMap_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__srv__SaveMap_Request__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__srv__SaveMap_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SaveMap_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__SaveMap_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__SaveMap_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__SaveMap_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SaveMap_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SaveMap_Request where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/srv/SaveMap_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__SaveMap_Request() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__SaveMap_Response() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__srv__SaveMap_Response__init(msg: *mut SaveMap_Response) -> bool;
    fn moveit_msgs__srv__SaveMap_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SaveMap_Response>, size: usize) -> bool;
    fn moveit_msgs__srv__SaveMap_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SaveMap_Response>);
    fn moveit_msgs__srv__SaveMap_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SaveMap_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SaveMap_Response>) -> bool;
}

// Corresponds to moveit_msgs__srv__SaveMap_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SaveMap_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,

}



impl Default for SaveMap_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__srv__SaveMap_Response__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__srv__SaveMap_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SaveMap_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__SaveMap_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__SaveMap_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__SaveMap_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SaveMap_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SaveMap_Response where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/srv/SaveMap_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__SaveMap_Response() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__LoadGeometryFromFile_Request() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__srv__LoadGeometryFromFile_Request__init(msg: *mut LoadGeometryFromFile_Request) -> bool;
    fn moveit_msgs__srv__LoadGeometryFromFile_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<LoadGeometryFromFile_Request>, size: usize) -> bool;
    fn moveit_msgs__srv__LoadGeometryFromFile_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<LoadGeometryFromFile_Request>);
    fn moveit_msgs__srv__LoadGeometryFromFile_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<LoadGeometryFromFile_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<LoadGeometryFromFile_Request>) -> bool;
}

// Corresponds to moveit_msgs__srv__LoadGeometryFromFile_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LoadGeometryFromFile_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub file_path_and_name: rosidl_runtime_rs::String,

}



impl Default for LoadGeometryFromFile_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__srv__LoadGeometryFromFile_Request__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__srv__LoadGeometryFromFile_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for LoadGeometryFromFile_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__LoadGeometryFromFile_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__LoadGeometryFromFile_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__LoadGeometryFromFile_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for LoadGeometryFromFile_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for LoadGeometryFromFile_Request where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/srv/LoadGeometryFromFile_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__LoadGeometryFromFile_Request() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__LoadGeometryFromFile_Response() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__srv__LoadGeometryFromFile_Response__init(msg: *mut LoadGeometryFromFile_Response) -> bool;
    fn moveit_msgs__srv__LoadGeometryFromFile_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<LoadGeometryFromFile_Response>, size: usize) -> bool;
    fn moveit_msgs__srv__LoadGeometryFromFile_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<LoadGeometryFromFile_Response>);
    fn moveit_msgs__srv__LoadGeometryFromFile_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<LoadGeometryFromFile_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<LoadGeometryFromFile_Response>) -> bool;
}

// Corresponds to moveit_msgs__srv__LoadGeometryFromFile_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LoadGeometryFromFile_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,

}



impl Default for LoadGeometryFromFile_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__srv__LoadGeometryFromFile_Response__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__srv__LoadGeometryFromFile_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for LoadGeometryFromFile_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__LoadGeometryFromFile_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__LoadGeometryFromFile_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__LoadGeometryFromFile_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for LoadGeometryFromFile_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for LoadGeometryFromFile_Response where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/srv/LoadGeometryFromFile_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__LoadGeometryFromFile_Response() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__LoadMap_Request() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__srv__LoadMap_Request__init(msg: *mut LoadMap_Request) -> bool;
    fn moveit_msgs__srv__LoadMap_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<LoadMap_Request>, size: usize) -> bool;
    fn moveit_msgs__srv__LoadMap_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<LoadMap_Request>);
    fn moveit_msgs__srv__LoadMap_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<LoadMap_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<LoadMap_Request>) -> bool;
}

// Corresponds to moveit_msgs__srv__LoadMap_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LoadMap_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub filename: rosidl_runtime_rs::String,

}



impl Default for LoadMap_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__srv__LoadMap_Request__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__srv__LoadMap_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for LoadMap_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__LoadMap_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__LoadMap_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__LoadMap_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for LoadMap_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for LoadMap_Request where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/srv/LoadMap_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__LoadMap_Request() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__LoadMap_Response() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__srv__LoadMap_Response__init(msg: *mut LoadMap_Response) -> bool;
    fn moveit_msgs__srv__LoadMap_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<LoadMap_Response>, size: usize) -> bool;
    fn moveit_msgs__srv__LoadMap_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<LoadMap_Response>);
    fn moveit_msgs__srv__LoadMap_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<LoadMap_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<LoadMap_Response>) -> bool;
}

// Corresponds to moveit_msgs__srv__LoadMap_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LoadMap_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,

}



impl Default for LoadMap_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__srv__LoadMap_Response__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__srv__LoadMap_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for LoadMap_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__LoadMap_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__LoadMap_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__LoadMap_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for LoadMap_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for LoadMap_Response where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/srv/LoadMap_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__LoadMap_Response() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__SaveRobotStateToWarehouse_Request() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__srv__SaveRobotStateToWarehouse_Request__init(msg: *mut SaveRobotStateToWarehouse_Request) -> bool;
    fn moveit_msgs__srv__SaveRobotStateToWarehouse_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SaveRobotStateToWarehouse_Request>, size: usize) -> bool;
    fn moveit_msgs__srv__SaveRobotStateToWarehouse_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SaveRobotStateToWarehouse_Request>);
    fn moveit_msgs__srv__SaveRobotStateToWarehouse_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SaveRobotStateToWarehouse_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SaveRobotStateToWarehouse_Request>) -> bool;
}

// Corresponds to moveit_msgs__srv__SaveRobotStateToWarehouse_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SaveRobotStateToWarehouse_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub name: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub robot: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub state: super::super::msg::rmw::RobotState,

}



impl Default for SaveRobotStateToWarehouse_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__srv__SaveRobotStateToWarehouse_Request__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__srv__SaveRobotStateToWarehouse_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SaveRobotStateToWarehouse_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__SaveRobotStateToWarehouse_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__SaveRobotStateToWarehouse_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__SaveRobotStateToWarehouse_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SaveRobotStateToWarehouse_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SaveRobotStateToWarehouse_Request where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/srv/SaveRobotStateToWarehouse_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__SaveRobotStateToWarehouse_Request() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__SaveRobotStateToWarehouse_Response() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__srv__SaveRobotStateToWarehouse_Response__init(msg: *mut SaveRobotStateToWarehouse_Response) -> bool;
    fn moveit_msgs__srv__SaveRobotStateToWarehouse_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SaveRobotStateToWarehouse_Response>, size: usize) -> bool;
    fn moveit_msgs__srv__SaveRobotStateToWarehouse_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SaveRobotStateToWarehouse_Response>);
    fn moveit_msgs__srv__SaveRobotStateToWarehouse_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SaveRobotStateToWarehouse_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SaveRobotStateToWarehouse_Response>) -> bool;
}

// Corresponds to moveit_msgs__srv__SaveRobotStateToWarehouse_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SaveRobotStateToWarehouse_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,

}



impl Default for SaveRobotStateToWarehouse_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__srv__SaveRobotStateToWarehouse_Response__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__srv__SaveRobotStateToWarehouse_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SaveRobotStateToWarehouse_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__SaveRobotStateToWarehouse_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__SaveRobotStateToWarehouse_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__SaveRobotStateToWarehouse_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SaveRobotStateToWarehouse_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SaveRobotStateToWarehouse_Response where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/srv/SaveRobotStateToWarehouse_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__SaveRobotStateToWarehouse_Response() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__ListRobotStatesInWarehouse_Request() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__srv__ListRobotStatesInWarehouse_Request__init(msg: *mut ListRobotStatesInWarehouse_Request) -> bool;
    fn moveit_msgs__srv__ListRobotStatesInWarehouse_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ListRobotStatesInWarehouse_Request>, size: usize) -> bool;
    fn moveit_msgs__srv__ListRobotStatesInWarehouse_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ListRobotStatesInWarehouse_Request>);
    fn moveit_msgs__srv__ListRobotStatesInWarehouse_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ListRobotStatesInWarehouse_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<ListRobotStatesInWarehouse_Request>) -> bool;
}

// Corresponds to moveit_msgs__srv__ListRobotStatesInWarehouse_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ListRobotStatesInWarehouse_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub regex: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub robot: rosidl_runtime_rs::String,

}



impl Default for ListRobotStatesInWarehouse_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__srv__ListRobotStatesInWarehouse_Request__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__srv__ListRobotStatesInWarehouse_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ListRobotStatesInWarehouse_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__ListRobotStatesInWarehouse_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__ListRobotStatesInWarehouse_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__ListRobotStatesInWarehouse_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ListRobotStatesInWarehouse_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ListRobotStatesInWarehouse_Request where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/srv/ListRobotStatesInWarehouse_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__ListRobotStatesInWarehouse_Request() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__ListRobotStatesInWarehouse_Response() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__srv__ListRobotStatesInWarehouse_Response__init(msg: *mut ListRobotStatesInWarehouse_Response) -> bool;
    fn moveit_msgs__srv__ListRobotStatesInWarehouse_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ListRobotStatesInWarehouse_Response>, size: usize) -> bool;
    fn moveit_msgs__srv__ListRobotStatesInWarehouse_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ListRobotStatesInWarehouse_Response>);
    fn moveit_msgs__srv__ListRobotStatesInWarehouse_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ListRobotStatesInWarehouse_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<ListRobotStatesInWarehouse_Response>) -> bool;
}

// Corresponds to moveit_msgs__srv__ListRobotStatesInWarehouse_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ListRobotStatesInWarehouse_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub states: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,

}



impl Default for ListRobotStatesInWarehouse_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__srv__ListRobotStatesInWarehouse_Response__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__srv__ListRobotStatesInWarehouse_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ListRobotStatesInWarehouse_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__ListRobotStatesInWarehouse_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__ListRobotStatesInWarehouse_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__ListRobotStatesInWarehouse_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ListRobotStatesInWarehouse_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ListRobotStatesInWarehouse_Response where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/srv/ListRobotStatesInWarehouse_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__ListRobotStatesInWarehouse_Response() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__GetRobotStateFromWarehouse_Request() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__srv__GetRobotStateFromWarehouse_Request__init(msg: *mut GetRobotStateFromWarehouse_Request) -> bool;
    fn moveit_msgs__srv__GetRobotStateFromWarehouse_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetRobotStateFromWarehouse_Request>, size: usize) -> bool;
    fn moveit_msgs__srv__GetRobotStateFromWarehouse_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetRobotStateFromWarehouse_Request>);
    fn moveit_msgs__srv__GetRobotStateFromWarehouse_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetRobotStateFromWarehouse_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GetRobotStateFromWarehouse_Request>) -> bool;
}

// Corresponds to moveit_msgs__srv__GetRobotStateFromWarehouse_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetRobotStateFromWarehouse_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub name: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub robot: rosidl_runtime_rs::String,

}



impl Default for GetRobotStateFromWarehouse_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__srv__GetRobotStateFromWarehouse_Request__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__srv__GetRobotStateFromWarehouse_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetRobotStateFromWarehouse_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__GetRobotStateFromWarehouse_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__GetRobotStateFromWarehouse_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__GetRobotStateFromWarehouse_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetRobotStateFromWarehouse_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetRobotStateFromWarehouse_Request where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/srv/GetRobotStateFromWarehouse_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__GetRobotStateFromWarehouse_Request() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__GetRobotStateFromWarehouse_Response() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__srv__GetRobotStateFromWarehouse_Response__init(msg: *mut GetRobotStateFromWarehouse_Response) -> bool;
    fn moveit_msgs__srv__GetRobotStateFromWarehouse_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetRobotStateFromWarehouse_Response>, size: usize) -> bool;
    fn moveit_msgs__srv__GetRobotStateFromWarehouse_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetRobotStateFromWarehouse_Response>);
    fn moveit_msgs__srv__GetRobotStateFromWarehouse_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetRobotStateFromWarehouse_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GetRobotStateFromWarehouse_Response>) -> bool;
}

// Corresponds to moveit_msgs__srv__GetRobotStateFromWarehouse_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetRobotStateFromWarehouse_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub state: super::super::msg::rmw::RobotState,

}



impl Default for GetRobotStateFromWarehouse_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__srv__GetRobotStateFromWarehouse_Response__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__srv__GetRobotStateFromWarehouse_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetRobotStateFromWarehouse_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__GetRobotStateFromWarehouse_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__GetRobotStateFromWarehouse_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__GetRobotStateFromWarehouse_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetRobotStateFromWarehouse_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetRobotStateFromWarehouse_Response where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/srv/GetRobotStateFromWarehouse_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__GetRobotStateFromWarehouse_Response() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__CheckIfRobotStateExistsInWarehouse_Request() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__srv__CheckIfRobotStateExistsInWarehouse_Request__init(msg: *mut CheckIfRobotStateExistsInWarehouse_Request) -> bool;
    fn moveit_msgs__srv__CheckIfRobotStateExistsInWarehouse_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<CheckIfRobotStateExistsInWarehouse_Request>, size: usize) -> bool;
    fn moveit_msgs__srv__CheckIfRobotStateExistsInWarehouse_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<CheckIfRobotStateExistsInWarehouse_Request>);
    fn moveit_msgs__srv__CheckIfRobotStateExistsInWarehouse_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<CheckIfRobotStateExistsInWarehouse_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<CheckIfRobotStateExistsInWarehouse_Request>) -> bool;
}

// Corresponds to moveit_msgs__srv__CheckIfRobotStateExistsInWarehouse_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CheckIfRobotStateExistsInWarehouse_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub name: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub robot: rosidl_runtime_rs::String,

}



impl Default for CheckIfRobotStateExistsInWarehouse_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__srv__CheckIfRobotStateExistsInWarehouse_Request__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__srv__CheckIfRobotStateExistsInWarehouse_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for CheckIfRobotStateExistsInWarehouse_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__CheckIfRobotStateExistsInWarehouse_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__CheckIfRobotStateExistsInWarehouse_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__CheckIfRobotStateExistsInWarehouse_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for CheckIfRobotStateExistsInWarehouse_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for CheckIfRobotStateExistsInWarehouse_Request where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/srv/CheckIfRobotStateExistsInWarehouse_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__CheckIfRobotStateExistsInWarehouse_Request() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__CheckIfRobotStateExistsInWarehouse_Response() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__srv__CheckIfRobotStateExistsInWarehouse_Response__init(msg: *mut CheckIfRobotStateExistsInWarehouse_Response) -> bool;
    fn moveit_msgs__srv__CheckIfRobotStateExistsInWarehouse_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<CheckIfRobotStateExistsInWarehouse_Response>, size: usize) -> bool;
    fn moveit_msgs__srv__CheckIfRobotStateExistsInWarehouse_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<CheckIfRobotStateExistsInWarehouse_Response>);
    fn moveit_msgs__srv__CheckIfRobotStateExistsInWarehouse_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<CheckIfRobotStateExistsInWarehouse_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<CheckIfRobotStateExistsInWarehouse_Response>) -> bool;
}

// Corresponds to moveit_msgs__srv__CheckIfRobotStateExistsInWarehouse_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CheckIfRobotStateExistsInWarehouse_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub exists: bool,

}



impl Default for CheckIfRobotStateExistsInWarehouse_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__srv__CheckIfRobotStateExistsInWarehouse_Response__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__srv__CheckIfRobotStateExistsInWarehouse_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for CheckIfRobotStateExistsInWarehouse_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__CheckIfRobotStateExistsInWarehouse_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__CheckIfRobotStateExistsInWarehouse_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__CheckIfRobotStateExistsInWarehouse_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for CheckIfRobotStateExistsInWarehouse_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for CheckIfRobotStateExistsInWarehouse_Response where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/srv/CheckIfRobotStateExistsInWarehouse_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__CheckIfRobotStateExistsInWarehouse_Response() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__RenameRobotStateInWarehouse_Request() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__srv__RenameRobotStateInWarehouse_Request__init(msg: *mut RenameRobotStateInWarehouse_Request) -> bool;
    fn moveit_msgs__srv__RenameRobotStateInWarehouse_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<RenameRobotStateInWarehouse_Request>, size: usize) -> bool;
    fn moveit_msgs__srv__RenameRobotStateInWarehouse_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<RenameRobotStateInWarehouse_Request>);
    fn moveit_msgs__srv__RenameRobotStateInWarehouse_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<RenameRobotStateInWarehouse_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<RenameRobotStateInWarehouse_Request>) -> bool;
}

// Corresponds to moveit_msgs__srv__RenameRobotStateInWarehouse_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RenameRobotStateInWarehouse_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub old_name: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub new_name: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub robot: rosidl_runtime_rs::String,

}



impl Default for RenameRobotStateInWarehouse_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__srv__RenameRobotStateInWarehouse_Request__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__srv__RenameRobotStateInWarehouse_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for RenameRobotStateInWarehouse_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__RenameRobotStateInWarehouse_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__RenameRobotStateInWarehouse_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__RenameRobotStateInWarehouse_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for RenameRobotStateInWarehouse_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for RenameRobotStateInWarehouse_Request where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/srv/RenameRobotStateInWarehouse_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__RenameRobotStateInWarehouse_Request() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__RenameRobotStateInWarehouse_Response() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__srv__RenameRobotStateInWarehouse_Response__init(msg: *mut RenameRobotStateInWarehouse_Response) -> bool;
    fn moveit_msgs__srv__RenameRobotStateInWarehouse_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<RenameRobotStateInWarehouse_Response>, size: usize) -> bool;
    fn moveit_msgs__srv__RenameRobotStateInWarehouse_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<RenameRobotStateInWarehouse_Response>);
    fn moveit_msgs__srv__RenameRobotStateInWarehouse_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<RenameRobotStateInWarehouse_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<RenameRobotStateInWarehouse_Response>) -> bool;
}

// Corresponds to moveit_msgs__srv__RenameRobotStateInWarehouse_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RenameRobotStateInWarehouse_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for RenameRobotStateInWarehouse_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__srv__RenameRobotStateInWarehouse_Response__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__srv__RenameRobotStateInWarehouse_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for RenameRobotStateInWarehouse_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__RenameRobotStateInWarehouse_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__RenameRobotStateInWarehouse_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__RenameRobotStateInWarehouse_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for RenameRobotStateInWarehouse_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for RenameRobotStateInWarehouse_Response where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/srv/RenameRobotStateInWarehouse_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__RenameRobotStateInWarehouse_Response() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__DeleteRobotStateFromWarehouse_Request() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__srv__DeleteRobotStateFromWarehouse_Request__init(msg: *mut DeleteRobotStateFromWarehouse_Request) -> bool;
    fn moveit_msgs__srv__DeleteRobotStateFromWarehouse_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DeleteRobotStateFromWarehouse_Request>, size: usize) -> bool;
    fn moveit_msgs__srv__DeleteRobotStateFromWarehouse_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DeleteRobotStateFromWarehouse_Request>);
    fn moveit_msgs__srv__DeleteRobotStateFromWarehouse_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DeleteRobotStateFromWarehouse_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<DeleteRobotStateFromWarehouse_Request>) -> bool;
}

// Corresponds to moveit_msgs__srv__DeleteRobotStateFromWarehouse_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DeleteRobotStateFromWarehouse_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub name: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub robot: rosidl_runtime_rs::String,

}



impl Default for DeleteRobotStateFromWarehouse_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__srv__DeleteRobotStateFromWarehouse_Request__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__srv__DeleteRobotStateFromWarehouse_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DeleteRobotStateFromWarehouse_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__DeleteRobotStateFromWarehouse_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__DeleteRobotStateFromWarehouse_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__DeleteRobotStateFromWarehouse_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DeleteRobotStateFromWarehouse_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DeleteRobotStateFromWarehouse_Request where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/srv/DeleteRobotStateFromWarehouse_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__DeleteRobotStateFromWarehouse_Request() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__DeleteRobotStateFromWarehouse_Response() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__srv__DeleteRobotStateFromWarehouse_Response__init(msg: *mut DeleteRobotStateFromWarehouse_Response) -> bool;
    fn moveit_msgs__srv__DeleteRobotStateFromWarehouse_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DeleteRobotStateFromWarehouse_Response>, size: usize) -> bool;
    fn moveit_msgs__srv__DeleteRobotStateFromWarehouse_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DeleteRobotStateFromWarehouse_Response>);
    fn moveit_msgs__srv__DeleteRobotStateFromWarehouse_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DeleteRobotStateFromWarehouse_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<DeleteRobotStateFromWarehouse_Response>) -> bool;
}

// Corresponds to moveit_msgs__srv__DeleteRobotStateFromWarehouse_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DeleteRobotStateFromWarehouse_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for DeleteRobotStateFromWarehouse_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__srv__DeleteRobotStateFromWarehouse_Response__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__srv__DeleteRobotStateFromWarehouse_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DeleteRobotStateFromWarehouse_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__DeleteRobotStateFromWarehouse_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__DeleteRobotStateFromWarehouse_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__DeleteRobotStateFromWarehouse_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DeleteRobotStateFromWarehouse_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DeleteRobotStateFromWarehouse_Response where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/srv/DeleteRobotStateFromWarehouse_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__DeleteRobotStateFromWarehouse_Response() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__ServoCommandType_Request() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__srv__ServoCommandType_Request__init(msg: *mut ServoCommandType_Request) -> bool;
    fn moveit_msgs__srv__ServoCommandType_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ServoCommandType_Request>, size: usize) -> bool;
    fn moveit_msgs__srv__ServoCommandType_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ServoCommandType_Request>);
    fn moveit_msgs__srv__ServoCommandType_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ServoCommandType_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<ServoCommandType_Request>) -> bool;
}

// Corresponds to moveit_msgs__srv__ServoCommandType_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ServoCommandType_Request {
    /// Request Field
    pub command_type: i8,

}

impl ServoCommandType_Request {

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const JOINT_JOG: i8 = 0;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const TWIST: i8 = 1;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const POSE: i8 = 2;

}


impl Default for ServoCommandType_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__srv__ServoCommandType_Request__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__srv__ServoCommandType_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ServoCommandType_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__ServoCommandType_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__ServoCommandType_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__ServoCommandType_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ServoCommandType_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ServoCommandType_Request where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/srv/ServoCommandType_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__ServoCommandType_Request() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__ServoCommandType_Response() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__srv__ServoCommandType_Response__init(msg: *mut ServoCommandType_Response) -> bool;
    fn moveit_msgs__srv__ServoCommandType_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ServoCommandType_Response>, size: usize) -> bool;
    fn moveit_msgs__srv__ServoCommandType_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ServoCommandType_Response>);
    fn moveit_msgs__srv__ServoCommandType_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ServoCommandType_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<ServoCommandType_Response>) -> bool;
}

// Corresponds to moveit_msgs__srv__ServoCommandType_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ServoCommandType_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,

}



impl Default for ServoCommandType_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__srv__ServoCommandType_Response__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__srv__ServoCommandType_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ServoCommandType_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__ServoCommandType_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__ServoCommandType_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__ServoCommandType_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ServoCommandType_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ServoCommandType_Response where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/srv/ServoCommandType_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__ServoCommandType_Response() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__GetGroupUrdf_Request() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__srv__GetGroupUrdf_Request__init(msg: *mut GetGroupUrdf_Request) -> bool;
    fn moveit_msgs__srv__GetGroupUrdf_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetGroupUrdf_Request>, size: usize) -> bool;
    fn moveit_msgs__srv__GetGroupUrdf_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetGroupUrdf_Request>);
    fn moveit_msgs__srv__GetGroupUrdf_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetGroupUrdf_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GetGroupUrdf_Request>) -> bool;
}

// Corresponds to moveit_msgs__srv__GetGroupUrdf_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetGroupUrdf_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub group_name: rosidl_runtime_rs::String,

}



impl Default for GetGroupUrdf_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__srv__GetGroupUrdf_Request__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__srv__GetGroupUrdf_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetGroupUrdf_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__GetGroupUrdf_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__GetGroupUrdf_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__GetGroupUrdf_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetGroupUrdf_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetGroupUrdf_Request where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/srv/GetGroupUrdf_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__GetGroupUrdf_Request() }
  }
}


#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__GetGroupUrdf_Response() -> *const std::ffi::c_void;
}

#[link(name = "moveit_msgs__rosidl_generator_c")]
extern "C" {
    fn moveit_msgs__srv__GetGroupUrdf_Response__init(msg: *mut GetGroupUrdf_Response) -> bool;
    fn moveit_msgs__srv__GetGroupUrdf_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetGroupUrdf_Response>, size: usize) -> bool;
    fn moveit_msgs__srv__GetGroupUrdf_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetGroupUrdf_Response>);
    fn moveit_msgs__srv__GetGroupUrdf_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetGroupUrdf_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GetGroupUrdf_Response>) -> bool;
}

// Corresponds to moveit_msgs__srv__GetGroupUrdf_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetGroupUrdf_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub error_code: super::super::msg::rmw::MoveItErrorCodes,

    /// URDF of the requested group. Empty if request was not successful e.g. because the group does not exist
    pub urdf_string: rosidl_runtime_rs::String,

}



impl Default for GetGroupUrdf_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !moveit_msgs__srv__GetGroupUrdf_Response__init(&mut msg as *mut _) {
        panic!("Call to moveit_msgs__srv__GetGroupUrdf_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetGroupUrdf_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__GetGroupUrdf_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__GetGroupUrdf_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { moveit_msgs__srv__GetGroupUrdf_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetGroupUrdf_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetGroupUrdf_Response where Self: Sized {
  const TYPE_NAME: &'static str = "moveit_msgs/srv/GetGroupUrdf_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__moveit_msgs__srv__GetGroupUrdf_Response() }
  }
}






#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__moveit_msgs__srv__GetMotionPlan() -> *const std::ffi::c_void;
}

// Corresponds to moveit_msgs__srv__GetMotionPlan
#[allow(missing_docs, non_camel_case_types)]
pub struct GetMotionPlan;

impl rosidl_runtime_rs::Service for GetMotionPlan {
    type Request = GetMotionPlan_Request;
    type Response = GetMotionPlan_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__moveit_msgs__srv__GetMotionPlan() }
    }
}




#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__moveit_msgs__srv__GetMultiStateValidity() -> *const std::ffi::c_void;
}

// Corresponds to moveit_msgs__srv__GetMultiStateValidity
#[allow(missing_docs, non_camel_case_types)]
pub struct GetMultiStateValidity;

impl rosidl_runtime_rs::Service for GetMultiStateValidity {
    type Request = GetMultiStateValidity_Request;
    type Response = GetMultiStateValidity_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__moveit_msgs__srv__GetMultiStateValidity() }
    }
}




#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__moveit_msgs__srv__GetStateValidity() -> *const std::ffi::c_void;
}

// Corresponds to moveit_msgs__srv__GetStateValidity
#[allow(missing_docs, non_camel_case_types)]
pub struct GetStateValidity;

impl rosidl_runtime_rs::Service for GetStateValidity {
    type Request = GetStateValidity_Request;
    type Response = GetStateValidity_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__moveit_msgs__srv__GetStateValidity() }
    }
}




#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__moveit_msgs__srv__GetCartesianPath() -> *const std::ffi::c_void;
}

// Corresponds to moveit_msgs__srv__GetCartesianPath
#[allow(missing_docs, non_camel_case_types)]
pub struct GetCartesianPath;

impl rosidl_runtime_rs::Service for GetCartesianPath {
    type Request = GetCartesianPath_Request;
    type Response = GetCartesianPath_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__moveit_msgs__srv__GetCartesianPath() }
    }
}




#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__moveit_msgs__srv__GetPlanningScene() -> *const std::ffi::c_void;
}

// Corresponds to moveit_msgs__srv__GetPlanningScene
#[allow(missing_docs, non_camel_case_types)]
pub struct GetPlanningScene;

impl rosidl_runtime_rs::Service for GetPlanningScene {
    type Request = GetPlanningScene_Request;
    type Response = GetPlanningScene_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__moveit_msgs__srv__GetPlanningScene() }
    }
}




#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__moveit_msgs__srv__GraspPlanning() -> *const std::ffi::c_void;
}

// Corresponds to moveit_msgs__srv__GraspPlanning
#[allow(missing_docs, non_camel_case_types)]
pub struct GraspPlanning;

impl rosidl_runtime_rs::Service for GraspPlanning {
    type Request = GraspPlanning_Request;
    type Response = GraspPlanning_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__moveit_msgs__srv__GraspPlanning() }
    }
}




#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__moveit_msgs__srv__ApplyPlanningScene() -> *const std::ffi::c_void;
}

// Corresponds to moveit_msgs__srv__ApplyPlanningScene
#[allow(missing_docs, non_camel_case_types)]
pub struct ApplyPlanningScene;

impl rosidl_runtime_rs::Service for ApplyPlanningScene {
    type Request = ApplyPlanningScene_Request;
    type Response = ApplyPlanningScene_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__moveit_msgs__srv__ApplyPlanningScene() }
    }
}




#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__moveit_msgs__srv__QueryPlannerInterfaces() -> *const std::ffi::c_void;
}

// Corresponds to moveit_msgs__srv__QueryPlannerInterfaces
#[allow(missing_docs, non_camel_case_types)]
pub struct QueryPlannerInterfaces;

impl rosidl_runtime_rs::Service for QueryPlannerInterfaces {
    type Request = QueryPlannerInterfaces_Request;
    type Response = QueryPlannerInterfaces_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__moveit_msgs__srv__QueryPlannerInterfaces() }
    }
}




#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__moveit_msgs__srv__GetMotionSequence() -> *const std::ffi::c_void;
}

// Corresponds to moveit_msgs__srv__GetMotionSequence
#[allow(missing_docs, non_camel_case_types)]
pub struct GetMotionSequence;

impl rosidl_runtime_rs::Service for GetMotionSequence {
    type Request = GetMotionSequence_Request;
    type Response = GetMotionSequence_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__moveit_msgs__srv__GetMotionSequence() }
    }
}




#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__moveit_msgs__srv__GetPositionFK() -> *const std::ffi::c_void;
}

// Corresponds to moveit_msgs__srv__GetPositionFK
#[allow(missing_docs, non_camel_case_types)]
pub struct GetPositionFK;

impl rosidl_runtime_rs::Service for GetPositionFK {
    type Request = GetPositionFK_Request;
    type Response = GetPositionFK_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__moveit_msgs__srv__GetPositionFK() }
    }
}




#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__moveit_msgs__srv__GetPositionIK() -> *const std::ffi::c_void;
}

// Corresponds to moveit_msgs__srv__GetPositionIK
#[allow(missing_docs, non_camel_case_types)]
pub struct GetPositionIK;

impl rosidl_runtime_rs::Service for GetPositionIK {
    type Request = GetPositionIK_Request;
    type Response = GetPositionIK_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__moveit_msgs__srv__GetPositionIK() }
    }
}




#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__moveit_msgs__srv__GetPlannerParams() -> *const std::ffi::c_void;
}

// Corresponds to moveit_msgs__srv__GetPlannerParams
#[allow(missing_docs, non_camel_case_types)]
pub struct GetPlannerParams;

impl rosidl_runtime_rs::Service for GetPlannerParams {
    type Request = GetPlannerParams_Request;
    type Response = GetPlannerParams_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__moveit_msgs__srv__GetPlannerParams() }
    }
}




#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__moveit_msgs__srv__SetPlannerParams() -> *const std::ffi::c_void;
}

// Corresponds to moveit_msgs__srv__SetPlannerParams
#[allow(missing_docs, non_camel_case_types)]
pub struct SetPlannerParams;

impl rosidl_runtime_rs::Service for SetPlannerParams {
    type Request = SetPlannerParams_Request;
    type Response = SetPlannerParams_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__moveit_msgs__srv__SetPlannerParams() }
    }
}




#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__moveit_msgs__srv__UpdatePointcloudOctomap() -> *const std::ffi::c_void;
}

// Corresponds to moveit_msgs__srv__UpdatePointcloudOctomap
#[allow(missing_docs, non_camel_case_types)]
pub struct UpdatePointcloudOctomap;

impl rosidl_runtime_rs::Service for UpdatePointcloudOctomap {
    type Request = UpdatePointcloudOctomap_Request;
    type Response = UpdatePointcloudOctomap_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__moveit_msgs__srv__UpdatePointcloudOctomap() }
    }
}




#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__moveit_msgs__srv__SaveGeometryToFile() -> *const std::ffi::c_void;
}

// Corresponds to moveit_msgs__srv__SaveGeometryToFile
#[allow(missing_docs, non_camel_case_types)]
pub struct SaveGeometryToFile;

impl rosidl_runtime_rs::Service for SaveGeometryToFile {
    type Request = SaveGeometryToFile_Request;
    type Response = SaveGeometryToFile_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__moveit_msgs__srv__SaveGeometryToFile() }
    }
}




#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__moveit_msgs__srv__SaveMap() -> *const std::ffi::c_void;
}

// Corresponds to moveit_msgs__srv__SaveMap
#[allow(missing_docs, non_camel_case_types)]
pub struct SaveMap;

impl rosidl_runtime_rs::Service for SaveMap {
    type Request = SaveMap_Request;
    type Response = SaveMap_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__moveit_msgs__srv__SaveMap() }
    }
}




#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__moveit_msgs__srv__LoadGeometryFromFile() -> *const std::ffi::c_void;
}

// Corresponds to moveit_msgs__srv__LoadGeometryFromFile
#[allow(missing_docs, non_camel_case_types)]
pub struct LoadGeometryFromFile;

impl rosidl_runtime_rs::Service for LoadGeometryFromFile {
    type Request = LoadGeometryFromFile_Request;
    type Response = LoadGeometryFromFile_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__moveit_msgs__srv__LoadGeometryFromFile() }
    }
}




#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__moveit_msgs__srv__LoadMap() -> *const std::ffi::c_void;
}

// Corresponds to moveit_msgs__srv__LoadMap
#[allow(missing_docs, non_camel_case_types)]
pub struct LoadMap;

impl rosidl_runtime_rs::Service for LoadMap {
    type Request = LoadMap_Request;
    type Response = LoadMap_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__moveit_msgs__srv__LoadMap() }
    }
}




#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__moveit_msgs__srv__SaveRobotStateToWarehouse() -> *const std::ffi::c_void;
}

// Corresponds to moveit_msgs__srv__SaveRobotStateToWarehouse
#[allow(missing_docs, non_camel_case_types)]
pub struct SaveRobotStateToWarehouse;

impl rosidl_runtime_rs::Service for SaveRobotStateToWarehouse {
    type Request = SaveRobotStateToWarehouse_Request;
    type Response = SaveRobotStateToWarehouse_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__moveit_msgs__srv__SaveRobotStateToWarehouse() }
    }
}




#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__moveit_msgs__srv__ListRobotStatesInWarehouse() -> *const std::ffi::c_void;
}

// Corresponds to moveit_msgs__srv__ListRobotStatesInWarehouse
#[allow(missing_docs, non_camel_case_types)]
pub struct ListRobotStatesInWarehouse;

impl rosidl_runtime_rs::Service for ListRobotStatesInWarehouse {
    type Request = ListRobotStatesInWarehouse_Request;
    type Response = ListRobotStatesInWarehouse_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__moveit_msgs__srv__ListRobotStatesInWarehouse() }
    }
}




#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__moveit_msgs__srv__GetRobotStateFromWarehouse() -> *const std::ffi::c_void;
}

// Corresponds to moveit_msgs__srv__GetRobotStateFromWarehouse
#[allow(missing_docs, non_camel_case_types)]
pub struct GetRobotStateFromWarehouse;

impl rosidl_runtime_rs::Service for GetRobotStateFromWarehouse {
    type Request = GetRobotStateFromWarehouse_Request;
    type Response = GetRobotStateFromWarehouse_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__moveit_msgs__srv__GetRobotStateFromWarehouse() }
    }
}




#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__moveit_msgs__srv__CheckIfRobotStateExistsInWarehouse() -> *const std::ffi::c_void;
}

// Corresponds to moveit_msgs__srv__CheckIfRobotStateExistsInWarehouse
#[allow(missing_docs, non_camel_case_types)]
pub struct CheckIfRobotStateExistsInWarehouse;

impl rosidl_runtime_rs::Service for CheckIfRobotStateExistsInWarehouse {
    type Request = CheckIfRobotStateExistsInWarehouse_Request;
    type Response = CheckIfRobotStateExistsInWarehouse_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__moveit_msgs__srv__CheckIfRobotStateExistsInWarehouse() }
    }
}




#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__moveit_msgs__srv__RenameRobotStateInWarehouse() -> *const std::ffi::c_void;
}

// Corresponds to moveit_msgs__srv__RenameRobotStateInWarehouse
#[allow(missing_docs, non_camel_case_types)]
pub struct RenameRobotStateInWarehouse;

impl rosidl_runtime_rs::Service for RenameRobotStateInWarehouse {
    type Request = RenameRobotStateInWarehouse_Request;
    type Response = RenameRobotStateInWarehouse_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__moveit_msgs__srv__RenameRobotStateInWarehouse() }
    }
}




#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__moveit_msgs__srv__DeleteRobotStateFromWarehouse() -> *const std::ffi::c_void;
}

// Corresponds to moveit_msgs__srv__DeleteRobotStateFromWarehouse
#[allow(missing_docs, non_camel_case_types)]
pub struct DeleteRobotStateFromWarehouse;

impl rosidl_runtime_rs::Service for DeleteRobotStateFromWarehouse {
    type Request = DeleteRobotStateFromWarehouse_Request;
    type Response = DeleteRobotStateFromWarehouse_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__moveit_msgs__srv__DeleteRobotStateFromWarehouse() }
    }
}




#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__moveit_msgs__srv__ServoCommandType() -> *const std::ffi::c_void;
}

// Corresponds to moveit_msgs__srv__ServoCommandType
#[allow(missing_docs, non_camel_case_types)]
pub struct ServoCommandType;

impl rosidl_runtime_rs::Service for ServoCommandType {
    type Request = ServoCommandType_Request;
    type Response = ServoCommandType_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__moveit_msgs__srv__ServoCommandType() }
    }
}




#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__moveit_msgs__srv__GetGroupUrdf() -> *const std::ffi::c_void;
}

// Corresponds to moveit_msgs__srv__GetGroupUrdf
#[allow(missing_docs, non_camel_case_types)]
pub struct GetGroupUrdf;

impl rosidl_runtime_rs::Service for GetGroupUrdf {
    type Request = GetGroupUrdf_Request;
    type Response = GetGroupUrdf_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__moveit_msgs__srv__GetGroupUrdf() }
    }
}


