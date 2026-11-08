#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};




// Corresponds to moveit_msgs__srv__GetMotionPlan_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetMotionPlan_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub motion_plan_request: super::msg::MotionPlanRequest,

}



impl Default for GetMotionPlan_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetMotionPlan_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GetMotionPlan_Request {
  type RmwMsg = super::srv::rmw::GetMotionPlan_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        motion_plan_request: super::msg::MotionPlanRequest::into_rmw_message(std::borrow::Cow::Owned(msg.motion_plan_request)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        motion_plan_request: super::msg::MotionPlanRequest::into_rmw_message(std::borrow::Cow::Borrowed(&msg.motion_plan_request)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      motion_plan_request: super::msg::MotionPlanRequest::from_rmw_message(msg.motion_plan_request),
    }
  }
}


// Corresponds to moveit_msgs__srv__GetMotionPlan_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetMotionPlan_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub motion_plan_response: super::msg::MotionPlanResponse,

}



impl Default for GetMotionPlan_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetMotionPlan_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GetMotionPlan_Response {
  type RmwMsg = super::srv::rmw::GetMotionPlan_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        motion_plan_response: super::msg::MotionPlanResponse::into_rmw_message(std::borrow::Cow::Owned(msg.motion_plan_response)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        motion_plan_response: super::msg::MotionPlanResponse::into_rmw_message(std::borrow::Cow::Borrowed(&msg.motion_plan_response)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      motion_plan_response: super::msg::MotionPlanResponse::from_rmw_message(msg.motion_plan_response),
    }
  }
}


// Corresponds to moveit_msgs__srv__GetMultiStateValidity_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetMultiStateValidity_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub robot_state: super::msg::RobotState,


    // This member is not documented.
    #[allow(missing_docs)]
    pub joint_states: Vec<sensor_msgs::msg::JointState>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub group_name: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub constraints: super::msg::Constraints,

}



impl Default for GetMultiStateValidity_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetMultiStateValidity_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GetMultiStateValidity_Request {
  type RmwMsg = super::srv::rmw::GetMultiStateValidity_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        robot_state: super::msg::RobotState::into_rmw_message(std::borrow::Cow::Owned(msg.robot_state)).into_owned(),
        joint_states: msg.joint_states
          .into_iter()
          .map(|elem| sensor_msgs::msg::JointState::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
        group_name: msg.group_name.as_str().into(),
        constraints: super::msg::Constraints::into_rmw_message(std::borrow::Cow::Owned(msg.constraints)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        robot_state: super::msg::RobotState::into_rmw_message(std::borrow::Cow::Borrowed(&msg.robot_state)).into_owned(),
        joint_states: msg.joint_states
          .iter()
          .map(|elem| sensor_msgs::msg::JointState::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
        group_name: msg.group_name.as_str().into(),
        constraints: super::msg::Constraints::into_rmw_message(std::borrow::Cow::Borrowed(&msg.constraints)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      robot_state: super::msg::RobotState::from_rmw_message(msg.robot_state),
      joint_states: msg.joint_states
          .into_iter()
          .map(sensor_msgs::msg::JointState::from_rmw_message)
          .collect(),
      group_name: msg.group_name.to_string(),
      constraints: super::msg::Constraints::from_rmw_message(msg.constraints),
    }
  }
}


// Corresponds to moveit_msgs__srv__GetMultiStateValidity_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetMultiStateValidity_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub valid: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub contacts: Vec<super::msg::ContactInformation>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub cost_sources: Vec<super::msg::CostSource>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub constraint_result: Vec<super::msg::ConstraintEvalResult>,

}



impl Default for GetMultiStateValidity_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetMultiStateValidity_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GetMultiStateValidity_Response {
  type RmwMsg = super::srv::rmw::GetMultiStateValidity_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        valid: msg.valid,
        contacts: msg.contacts
          .into_iter()
          .map(|elem| super::msg::ContactInformation::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
        cost_sources: msg.cost_sources
          .into_iter()
          .map(|elem| super::msg::CostSource::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
        constraint_result: msg.constraint_result
          .into_iter()
          .map(|elem| super::msg::ConstraintEvalResult::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      valid: msg.valid,
        contacts: msg.contacts
          .iter()
          .map(|elem| super::msg::ContactInformation::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
        cost_sources: msg.cost_sources
          .iter()
          .map(|elem| super::msg::CostSource::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
        constraint_result: msg.constraint_result
          .iter()
          .map(|elem| super::msg::ConstraintEvalResult::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      valid: msg.valid,
      contacts: msg.contacts
          .into_iter()
          .map(super::msg::ContactInformation::from_rmw_message)
          .collect(),
      cost_sources: msg.cost_sources
          .into_iter()
          .map(super::msg::CostSource::from_rmw_message)
          .collect(),
      constraint_result: msg.constraint_result
          .into_iter()
          .map(super::msg::ConstraintEvalResult::from_rmw_message)
          .collect(),
    }
  }
}


// Corresponds to moveit_msgs__srv__GetStateValidity_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetStateValidity_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub robot_state: super::msg::RobotState,


    // This member is not documented.
    #[allow(missing_docs)]
    pub group_name: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub constraints: super::msg::Constraints,

}



impl Default for GetStateValidity_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetStateValidity_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GetStateValidity_Request {
  type RmwMsg = super::srv::rmw::GetStateValidity_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        robot_state: super::msg::RobotState::into_rmw_message(std::borrow::Cow::Owned(msg.robot_state)).into_owned(),
        group_name: msg.group_name.as_str().into(),
        constraints: super::msg::Constraints::into_rmw_message(std::borrow::Cow::Owned(msg.constraints)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        robot_state: super::msg::RobotState::into_rmw_message(std::borrow::Cow::Borrowed(&msg.robot_state)).into_owned(),
        group_name: msg.group_name.as_str().into(),
        constraints: super::msg::Constraints::into_rmw_message(std::borrow::Cow::Borrowed(&msg.constraints)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      robot_state: super::msg::RobotState::from_rmw_message(msg.robot_state),
      group_name: msg.group_name.to_string(),
      constraints: super::msg::Constraints::from_rmw_message(msg.constraints),
    }
  }
}


// Corresponds to moveit_msgs__srv__GetStateValidity_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetStateValidity_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub valid: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub contacts: Vec<super::msg::ContactInformation>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub cost_sources: Vec<super::msg::CostSource>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub constraint_result: Vec<super::msg::ConstraintEvalResult>,

}



impl Default for GetStateValidity_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetStateValidity_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GetStateValidity_Response {
  type RmwMsg = super::srv::rmw::GetStateValidity_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        valid: msg.valid,
        contacts: msg.contacts
          .into_iter()
          .map(|elem| super::msg::ContactInformation::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
        cost_sources: msg.cost_sources
          .into_iter()
          .map(|elem| super::msg::CostSource::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
        constraint_result: msg.constraint_result
          .into_iter()
          .map(|elem| super::msg::ConstraintEvalResult::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      valid: msg.valid,
        contacts: msg.contacts
          .iter()
          .map(|elem| super::msg::ContactInformation::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
        cost_sources: msg.cost_sources
          .iter()
          .map(|elem| super::msg::CostSource::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
        constraint_result: msg.constraint_result
          .iter()
          .map(|elem| super::msg::ConstraintEvalResult::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      valid: msg.valid,
      contacts: msg.contacts
          .into_iter()
          .map(super::msg::ContactInformation::from_rmw_message)
          .collect(),
      cost_sources: msg.cost_sources
          .into_iter()
          .map(super::msg::CostSource::from_rmw_message)
          .collect(),
      constraint_result: msg.constraint_result
          .into_iter()
          .map(super::msg::ConstraintEvalResult::from_rmw_message)
          .collect(),
    }
  }
}


// Corresponds to moveit_msgs__srv__GetCartesianPath_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetCartesianPath_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,

    /// The start at which to start the Cartesian path
    pub start_state: super::msg::RobotState,

    /// Mandatory name of group to compute the path for
    pub group_name: std::string::String,

    /// Optional name of IK link for which waypoints are specified.
    /// If not specified, the tip of the group (which is assumed to be a chain)
    /// is assumed to be the link
    pub link_name: std::string::String,

    /// A sequence of waypoints to be followed by the specified link,
    /// while moving the specified group, such that the group moves only
    /// in a straight line between waypoints
    pub waypoints: Vec<geometry_msgs::msg::Pose>,

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
    pub path_constraints: super::msg::Constraints,

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
    pub cartesian_speed_limited_link: std::string::String,

    /// m/s
    pub max_cartesian_speed: f64,

}



impl Default for GetCartesianPath_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetCartesianPath_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GetCartesianPath_Request {
  type RmwMsg = super::srv::rmw::GetCartesianPath_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        start_state: super::msg::RobotState::into_rmw_message(std::borrow::Cow::Owned(msg.start_state)).into_owned(),
        group_name: msg.group_name.as_str().into(),
        link_name: msg.link_name.as_str().into(),
        waypoints: msg.waypoints
          .into_iter()
          .map(|elem| geometry_msgs::msg::Pose::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
        max_step: msg.max_step,
        jump_threshold: msg.jump_threshold,
        prismatic_jump_threshold: msg.prismatic_jump_threshold,
        revolute_jump_threshold: msg.revolute_jump_threshold,
        avoid_collisions: msg.avoid_collisions,
        path_constraints: super::msg::Constraints::into_rmw_message(std::borrow::Cow::Owned(msg.path_constraints)).into_owned(),
        max_velocity_scaling_factor: msg.max_velocity_scaling_factor,
        max_acceleration_scaling_factor: msg.max_acceleration_scaling_factor,
        cartesian_speed_limited_link: msg.cartesian_speed_limited_link.as_str().into(),
        max_cartesian_speed: msg.max_cartesian_speed,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        start_state: super::msg::RobotState::into_rmw_message(std::borrow::Cow::Borrowed(&msg.start_state)).into_owned(),
        group_name: msg.group_name.as_str().into(),
        link_name: msg.link_name.as_str().into(),
        waypoints: msg.waypoints
          .iter()
          .map(|elem| geometry_msgs::msg::Pose::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      max_step: msg.max_step,
      jump_threshold: msg.jump_threshold,
      prismatic_jump_threshold: msg.prismatic_jump_threshold,
      revolute_jump_threshold: msg.revolute_jump_threshold,
      avoid_collisions: msg.avoid_collisions,
        path_constraints: super::msg::Constraints::into_rmw_message(std::borrow::Cow::Borrowed(&msg.path_constraints)).into_owned(),
      max_velocity_scaling_factor: msg.max_velocity_scaling_factor,
      max_acceleration_scaling_factor: msg.max_acceleration_scaling_factor,
        cartesian_speed_limited_link: msg.cartesian_speed_limited_link.as_str().into(),
      max_cartesian_speed: msg.max_cartesian_speed,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      start_state: super::msg::RobotState::from_rmw_message(msg.start_state),
      group_name: msg.group_name.to_string(),
      link_name: msg.link_name.to_string(),
      waypoints: msg.waypoints
          .into_iter()
          .map(geometry_msgs::msg::Pose::from_rmw_message)
          .collect(),
      max_step: msg.max_step,
      jump_threshold: msg.jump_threshold,
      prismatic_jump_threshold: msg.prismatic_jump_threshold,
      revolute_jump_threshold: msg.revolute_jump_threshold,
      avoid_collisions: msg.avoid_collisions,
      path_constraints: super::msg::Constraints::from_rmw_message(msg.path_constraints),
      max_velocity_scaling_factor: msg.max_velocity_scaling_factor,
      max_acceleration_scaling_factor: msg.max_acceleration_scaling_factor,
      cartesian_speed_limited_link: msg.cartesian_speed_limited_link.to_string(),
      max_cartesian_speed: msg.max_cartesian_speed,
    }
  }
}


// Corresponds to moveit_msgs__srv__GetCartesianPath_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetCartesianPath_Response {
    /// The state at which the computed path starts
    pub start_state: super::msg::RobotState,

    /// The computed solution trajectory, for the desired group, in configuration space
    pub solution: super::msg::RobotTrajectory,

    /// If the computation was incomplete, this value indicates the fraction of the path
    /// that was in fact computed (number of waypoints traveled through)
    pub fraction: f64,

    /// The error code of the computation
    pub error_code: super::msg::MoveItErrorCodes,

}



impl Default for GetCartesianPath_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetCartesianPath_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GetCartesianPath_Response {
  type RmwMsg = super::srv::rmw::GetCartesianPath_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        start_state: super::msg::RobotState::into_rmw_message(std::borrow::Cow::Owned(msg.start_state)).into_owned(),
        solution: super::msg::RobotTrajectory::into_rmw_message(std::borrow::Cow::Owned(msg.solution)).into_owned(),
        fraction: msg.fraction,
        error_code: super::msg::MoveItErrorCodes::into_rmw_message(std::borrow::Cow::Owned(msg.error_code)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        start_state: super::msg::RobotState::into_rmw_message(std::borrow::Cow::Borrowed(&msg.start_state)).into_owned(),
        solution: super::msg::RobotTrajectory::into_rmw_message(std::borrow::Cow::Borrowed(&msg.solution)).into_owned(),
      fraction: msg.fraction,
        error_code: super::msg::MoveItErrorCodes::into_rmw_message(std::borrow::Cow::Borrowed(&msg.error_code)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      start_state: super::msg::RobotState::from_rmw_message(msg.start_state),
      solution: super::msg::RobotTrajectory::from_rmw_message(msg.solution),
      fraction: msg.fraction,
      error_code: super::msg::MoveItErrorCodes::from_rmw_message(msg.error_code),
    }
  }
}


// Corresponds to moveit_msgs__srv__GetPlanningScene_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetPlanningScene_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub components: super::msg::PlanningSceneComponents,

}



impl Default for GetPlanningScene_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetPlanningScene_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GetPlanningScene_Request {
  type RmwMsg = super::srv::rmw::GetPlanningScene_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        components: super::msg::PlanningSceneComponents::into_rmw_message(std::borrow::Cow::Owned(msg.components)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        components: super::msg::PlanningSceneComponents::into_rmw_message(std::borrow::Cow::Borrowed(&msg.components)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      components: super::msg::PlanningSceneComponents::from_rmw_message(msg.components),
    }
  }
}


// Corresponds to moveit_msgs__srv__GetPlanningScene_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetPlanningScene_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub scene: super::msg::PlanningScene,

}



impl Default for GetPlanningScene_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetPlanningScene_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GetPlanningScene_Response {
  type RmwMsg = super::srv::rmw::GetPlanningScene_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        scene: super::msg::PlanningScene::into_rmw_message(std::borrow::Cow::Owned(msg.scene)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        scene: super::msg::PlanningScene::into_rmw_message(std::borrow::Cow::Borrowed(&msg.scene)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      scene: super::msg::PlanningScene::from_rmw_message(msg.scene),
    }
  }
}


// Corresponds to moveit_msgs__srv__GraspPlanning_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GraspPlanning_Request {
    /// the planning group used
    pub group_name: std::string::String,

    /// the object to be grasped
    pub target: super::msg::CollisionObject,

    /// the names of the relevant support surfaces (e.g. tables) in the collision map
    /// can be left empty if no names are available
    pub support_surfaces: Vec<std::string::String>,

    /// an optional list of grasps to be evaluated by the planner
    pub candidate_grasps: Vec<super::msg::Grasp>,

    /// an optional list of obstacles that we have semantic information about
    /// and that can be moved in the course of grasping
    pub movable_obstacles: Vec<super::msg::CollisionObject>,

}



impl Default for GraspPlanning_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GraspPlanning_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GraspPlanning_Request {
  type RmwMsg = super::srv::rmw::GraspPlanning_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        group_name: msg.group_name.as_str().into(),
        target: super::msg::CollisionObject::into_rmw_message(std::borrow::Cow::Owned(msg.target)).into_owned(),
        support_surfaces: msg.support_surfaces
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        candidate_grasps: msg.candidate_grasps
          .into_iter()
          .map(|elem| super::msg::Grasp::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
        movable_obstacles: msg.movable_obstacles
          .into_iter()
          .map(|elem| super::msg::CollisionObject::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        group_name: msg.group_name.as_str().into(),
        target: super::msg::CollisionObject::into_rmw_message(std::borrow::Cow::Borrowed(&msg.target)).into_owned(),
        support_surfaces: msg.support_surfaces
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        candidate_grasps: msg.candidate_grasps
          .iter()
          .map(|elem| super::msg::Grasp::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
        movable_obstacles: msg.movable_obstacles
          .iter()
          .map(|elem| super::msg::CollisionObject::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      group_name: msg.group_name.to_string(),
      target: super::msg::CollisionObject::from_rmw_message(msg.target),
      support_surfaces: msg.support_surfaces
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
      candidate_grasps: msg.candidate_grasps
          .into_iter()
          .map(super::msg::Grasp::from_rmw_message)
          .collect(),
      movable_obstacles: msg.movable_obstacles
          .into_iter()
          .map(super::msg::CollisionObject::from_rmw_message)
          .collect(),
    }
  }
}


// Corresponds to moveit_msgs__srv__GraspPlanning_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GraspPlanning_Response {
    /// the list of planned grasps
    pub grasps: Vec<super::msg::Grasp>,

    /// whether an error occurred
    pub error_code: super::msg::MoveItErrorCodes,

}



impl Default for GraspPlanning_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GraspPlanning_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GraspPlanning_Response {
  type RmwMsg = super::srv::rmw::GraspPlanning_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        grasps: msg.grasps
          .into_iter()
          .map(|elem| super::msg::Grasp::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
        error_code: super::msg::MoveItErrorCodes::into_rmw_message(std::borrow::Cow::Owned(msg.error_code)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        grasps: msg.grasps
          .iter()
          .map(|elem| super::msg::Grasp::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
        error_code: super::msg::MoveItErrorCodes::into_rmw_message(std::borrow::Cow::Borrowed(&msg.error_code)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      grasps: msg.grasps
          .into_iter()
          .map(super::msg::Grasp::from_rmw_message)
          .collect(),
      error_code: super::msg::MoveItErrorCodes::from_rmw_message(msg.error_code),
    }
  }
}


// Corresponds to moveit_msgs__srv__ApplyPlanningScene_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ApplyPlanningScene_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub scene: super::msg::PlanningScene,

}



impl Default for ApplyPlanningScene_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ApplyPlanningScene_Request::default())
  }
}

impl rosidl_runtime_rs::Message for ApplyPlanningScene_Request {
  type RmwMsg = super::srv::rmw::ApplyPlanningScene_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        scene: super::msg::PlanningScene::into_rmw_message(std::borrow::Cow::Owned(msg.scene)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        scene: super::msg::PlanningScene::into_rmw_message(std::borrow::Cow::Borrowed(&msg.scene)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      scene: super::msg::PlanningScene::from_rmw_message(msg.scene),
    }
  }
}


// Corresponds to moveit_msgs__srv__ApplyPlanningScene_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ApplyPlanningScene_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,

}



impl Default for ApplyPlanningScene_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ApplyPlanningScene_Response::default())
  }
}

impl rosidl_runtime_rs::Message for ApplyPlanningScene_Response {
  type RmwMsg = super::srv::rmw::ApplyPlanningScene_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
    }
  }
}


// Corresponds to moveit_msgs__srv__QueryPlannerInterfaces_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct QueryPlannerInterfaces_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for QueryPlannerInterfaces_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::QueryPlannerInterfaces_Request::default())
  }
}

impl rosidl_runtime_rs::Message for QueryPlannerInterfaces_Request {
  type RmwMsg = super::srv::rmw::QueryPlannerInterfaces_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to moveit_msgs__srv__QueryPlannerInterfaces_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct QueryPlannerInterfaces_Response {
    /// The planning instances that could be used in the benchmark
    pub planner_interfaces: Vec<super::msg::PlannerInterfaceDescription>,

}



impl Default for QueryPlannerInterfaces_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::QueryPlannerInterfaces_Response::default())
  }
}

impl rosidl_runtime_rs::Message for QueryPlannerInterfaces_Response {
  type RmwMsg = super::srv::rmw::QueryPlannerInterfaces_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        planner_interfaces: msg.planner_interfaces
          .into_iter()
          .map(|elem| super::msg::PlannerInterfaceDescription::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        planner_interfaces: msg.planner_interfaces
          .iter()
          .map(|elem| super::msg::PlannerInterfaceDescription::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      planner_interfaces: msg.planner_interfaces
          .into_iter()
          .map(super::msg::PlannerInterfaceDescription::from_rmw_message)
          .collect(),
    }
  }
}


// Corresponds to moveit_msgs__srv__GetMotionSequence_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetMotionSequence_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub request: super::msg::MotionSequenceRequest,

}



impl Default for GetMotionSequence_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetMotionSequence_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GetMotionSequence_Request {
  type RmwMsg = super::srv::rmw::GetMotionSequence_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        request: super::msg::MotionSequenceRequest::into_rmw_message(std::borrow::Cow::Owned(msg.request)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        request: super::msg::MotionSequenceRequest::into_rmw_message(std::borrow::Cow::Borrowed(&msg.request)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      request: super::msg::MotionSequenceRequest::from_rmw_message(msg.request),
    }
  }
}


// Corresponds to moveit_msgs__srv__GetMotionSequence_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetMotionSequence_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub response: super::msg::MotionSequenceResponse,

}



impl Default for GetMotionSequence_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetMotionSequence_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GetMotionSequence_Response {
  type RmwMsg = super::srv::rmw::GetMotionSequence_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        response: super::msg::MotionSequenceResponse::into_rmw_message(std::borrow::Cow::Owned(msg.response)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        response: super::msg::MotionSequenceResponse::into_rmw_message(std::borrow::Cow::Borrowed(&msg.response)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      response: super::msg::MotionSequenceResponse::from_rmw_message(msg.response),
    }
  }
}


// Corresponds to moveit_msgs__srv__GetPositionFK_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetPositionFK_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,

    /// A vector of link name for which forward kinematics must be computed
    pub fk_link_names: Vec<std::string::String>,

    /// A robot state consisting of joint names and joint positions to be used for forward kinematics
    pub robot_state: super::msg::RobotState,

}



impl Default for GetPositionFK_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetPositionFK_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GetPositionFK_Request {
  type RmwMsg = super::srv::rmw::GetPositionFK_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        fk_link_names: msg.fk_link_names
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        robot_state: super::msg::RobotState::into_rmw_message(std::borrow::Cow::Owned(msg.robot_state)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        fk_link_names: msg.fk_link_names
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        robot_state: super::msg::RobotState::into_rmw_message(std::borrow::Cow::Borrowed(&msg.robot_state)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      fk_link_names: msg.fk_link_names
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
      robot_state: super::msg::RobotState::from_rmw_message(msg.robot_state),
    }
  }
}


// Corresponds to moveit_msgs__srv__GetPositionFK_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetPositionFK_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub pose_stamped: Vec<geometry_msgs::msg::PoseStamped>,

    /// The list of link names corresponding to the poses
    pub fk_link_names: Vec<std::string::String>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub error_code: super::msg::MoveItErrorCodes,

}



impl Default for GetPositionFK_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetPositionFK_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GetPositionFK_Response {
  type RmwMsg = super::srv::rmw::GetPositionFK_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        pose_stamped: msg.pose_stamped
          .into_iter()
          .map(|elem| geometry_msgs::msg::PoseStamped::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
        fk_link_names: msg.fk_link_names
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        error_code: super::msg::MoveItErrorCodes::into_rmw_message(std::borrow::Cow::Owned(msg.error_code)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        pose_stamped: msg.pose_stamped
          .iter()
          .map(|elem| geometry_msgs::msg::PoseStamped::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
        fk_link_names: msg.fk_link_names
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        error_code: super::msg::MoveItErrorCodes::into_rmw_message(std::borrow::Cow::Borrowed(&msg.error_code)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      pose_stamped: msg.pose_stamped
          .into_iter()
          .map(geometry_msgs::msg::PoseStamped::from_rmw_message)
          .collect(),
      fk_link_names: msg.fk_link_names
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
      error_code: super::msg::MoveItErrorCodes::from_rmw_message(msg.error_code),
    }
  }
}


// Corresponds to moveit_msgs__srv__GetPositionIK_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetPositionIK_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub ik_request: super::msg::PositionIKRequest,

}



impl Default for GetPositionIK_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetPositionIK_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GetPositionIK_Request {
  type RmwMsg = super::srv::rmw::GetPositionIK_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        ik_request: super::msg::PositionIKRequest::into_rmw_message(std::borrow::Cow::Owned(msg.ik_request)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        ik_request: super::msg::PositionIKRequest::into_rmw_message(std::borrow::Cow::Borrowed(&msg.ik_request)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      ik_request: super::msg::PositionIKRequest::from_rmw_message(msg.ik_request),
    }
  }
}


// Corresponds to moveit_msgs__srv__GetPositionIK_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetPositionIK_Response {
    /// The returned solution
    /// (in the same order as the list of joints specified in the IKRequest message)
    pub solution: super::msg::RobotState,


    // This member is not documented.
    #[allow(missing_docs)]
    pub error_code: super::msg::MoveItErrorCodes,

}



impl Default for GetPositionIK_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetPositionIK_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GetPositionIK_Response {
  type RmwMsg = super::srv::rmw::GetPositionIK_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        solution: super::msg::RobotState::into_rmw_message(std::borrow::Cow::Owned(msg.solution)).into_owned(),
        error_code: super::msg::MoveItErrorCodes::into_rmw_message(std::borrow::Cow::Owned(msg.error_code)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        solution: super::msg::RobotState::into_rmw_message(std::borrow::Cow::Borrowed(&msg.solution)).into_owned(),
        error_code: super::msg::MoveItErrorCodes::into_rmw_message(std::borrow::Cow::Borrowed(&msg.error_code)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      solution: super::msg::RobotState::from_rmw_message(msg.solution),
      error_code: super::msg::MoveItErrorCodes::from_rmw_message(msg.error_code),
    }
  }
}


// Corresponds to moveit_msgs__srv__GetPlannerParams_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetPlannerParams_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub pipeline_id: std::string::String,

    /// Name of planning config
    pub planner_config: std::string::String,

    /// Optional name of planning group (return global defaults if empty)
    pub group: std::string::String,

}



impl Default for GetPlannerParams_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetPlannerParams_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GetPlannerParams_Request {
  type RmwMsg = super::srv::rmw::GetPlannerParams_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        pipeline_id: msg.pipeline_id.as_str().into(),
        planner_config: msg.planner_config.as_str().into(),
        group: msg.group.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        pipeline_id: msg.pipeline_id.as_str().into(),
        planner_config: msg.planner_config.as_str().into(),
        group: msg.group.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      pipeline_id: msg.pipeline_id.to_string(),
      planner_config: msg.planner_config.to_string(),
      group: msg.group.to_string(),
    }
  }
}


// Corresponds to moveit_msgs__srv__GetPlannerParams_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetPlannerParams_Response {
    /// parameters as key-value pairs
    pub params: super::msg::PlannerParams,

}



impl Default for GetPlannerParams_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetPlannerParams_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GetPlannerParams_Response {
  type RmwMsg = super::srv::rmw::GetPlannerParams_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        params: super::msg::PlannerParams::into_rmw_message(std::borrow::Cow::Owned(msg.params)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        params: super::msg::PlannerParams::into_rmw_message(std::borrow::Cow::Borrowed(&msg.params)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      params: super::msg::PlannerParams::from_rmw_message(msg.params),
    }
  }
}


// Corresponds to moveit_msgs__srv__SetPlannerParams_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetPlannerParams_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub pipeline_id: std::string::String,

    /// Name of planning config
    pub planner_config: std::string::String,

    /// Optional name of planning group (set global defaults if empty)
    pub group: std::string::String,

    /// parameters as key-value pairs
    pub params: super::msg::PlannerParams,

    /// replace params or augment existing ones?
    pub replace: bool,

}



impl Default for SetPlannerParams_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetPlannerParams_Request::default())
  }
}

impl rosidl_runtime_rs::Message for SetPlannerParams_Request {
  type RmwMsg = super::srv::rmw::SetPlannerParams_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        pipeline_id: msg.pipeline_id.as_str().into(),
        planner_config: msg.planner_config.as_str().into(),
        group: msg.group.as_str().into(),
        params: super::msg::PlannerParams::into_rmw_message(std::borrow::Cow::Owned(msg.params)).into_owned(),
        replace: msg.replace,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        pipeline_id: msg.pipeline_id.as_str().into(),
        planner_config: msg.planner_config.as_str().into(),
        group: msg.group.as_str().into(),
        params: super::msg::PlannerParams::into_rmw_message(std::borrow::Cow::Borrowed(&msg.params)).into_owned(),
      replace: msg.replace,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      pipeline_id: msg.pipeline_id.to_string(),
      planner_config: msg.planner_config.to_string(),
      group: msg.group.to_string(),
      params: super::msg::PlannerParams::from_rmw_message(msg.params),
      replace: msg.replace,
    }
  }
}


// Corresponds to moveit_msgs__srv__SetPlannerParams_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetPlannerParams_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for SetPlannerParams_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetPlannerParams_Response::default())
  }
}

impl rosidl_runtime_rs::Message for SetPlannerParams_Response {
  type RmwMsg = super::srv::rmw::SetPlannerParams_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to moveit_msgs__srv__UpdatePointcloudOctomap_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct UpdatePointcloudOctomap_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub cloud: sensor_msgs::msg::PointCloud2,

}



impl Default for UpdatePointcloudOctomap_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::UpdatePointcloudOctomap_Request::default())
  }
}

impl rosidl_runtime_rs::Message for UpdatePointcloudOctomap_Request {
  type RmwMsg = super::srv::rmw::UpdatePointcloudOctomap_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        cloud: sensor_msgs::msg::PointCloud2::into_rmw_message(std::borrow::Cow::Owned(msg.cloud)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        cloud: sensor_msgs::msg::PointCloud2::into_rmw_message(std::borrow::Cow::Borrowed(&msg.cloud)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      cloud: sensor_msgs::msg::PointCloud2::from_rmw_message(msg.cloud),
    }
  }
}


// Corresponds to moveit_msgs__srv__UpdatePointcloudOctomap_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct UpdatePointcloudOctomap_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,

}



impl Default for UpdatePointcloudOctomap_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::UpdatePointcloudOctomap_Response::default())
  }
}

impl rosidl_runtime_rs::Message for UpdatePointcloudOctomap_Response {
  type RmwMsg = super::srv::rmw::UpdatePointcloudOctomap_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
    }
  }
}


// Corresponds to moveit_msgs__srv__SaveGeometryToFile_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SaveGeometryToFile_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub file_path_and_name: std::string::String,

}



impl Default for SaveGeometryToFile_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SaveGeometryToFile_Request::default())
  }
}

impl rosidl_runtime_rs::Message for SaveGeometryToFile_Request {
  type RmwMsg = super::srv::rmw::SaveGeometryToFile_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        file_path_and_name: msg.file_path_and_name.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        file_path_and_name: msg.file_path_and_name.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      file_path_and_name: msg.file_path_and_name.to_string(),
    }
  }
}


// Corresponds to moveit_msgs__srv__SaveGeometryToFile_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SaveGeometryToFile_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,

}



impl Default for SaveGeometryToFile_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SaveGeometryToFile_Response::default())
  }
}

impl rosidl_runtime_rs::Message for SaveGeometryToFile_Response {
  type RmwMsg = super::srv::rmw::SaveGeometryToFile_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
    }
  }
}


// Corresponds to moveit_msgs__srv__SaveMap_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SaveMap_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub filename: std::string::String,

}



impl Default for SaveMap_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SaveMap_Request::default())
  }
}

impl rosidl_runtime_rs::Message for SaveMap_Request {
  type RmwMsg = super::srv::rmw::SaveMap_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        filename: msg.filename.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        filename: msg.filename.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      filename: msg.filename.to_string(),
    }
  }
}


// Corresponds to moveit_msgs__srv__SaveMap_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SaveMap_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,

}



impl Default for SaveMap_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SaveMap_Response::default())
  }
}

impl rosidl_runtime_rs::Message for SaveMap_Response {
  type RmwMsg = super::srv::rmw::SaveMap_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
    }
  }
}


// Corresponds to moveit_msgs__srv__LoadGeometryFromFile_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LoadGeometryFromFile_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub file_path_and_name: std::string::String,

}



impl Default for LoadGeometryFromFile_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::LoadGeometryFromFile_Request::default())
  }
}

impl rosidl_runtime_rs::Message for LoadGeometryFromFile_Request {
  type RmwMsg = super::srv::rmw::LoadGeometryFromFile_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        file_path_and_name: msg.file_path_and_name.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        file_path_and_name: msg.file_path_and_name.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      file_path_and_name: msg.file_path_and_name.to_string(),
    }
  }
}


// Corresponds to moveit_msgs__srv__LoadGeometryFromFile_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LoadGeometryFromFile_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,

}



impl Default for LoadGeometryFromFile_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::LoadGeometryFromFile_Response::default())
  }
}

impl rosidl_runtime_rs::Message for LoadGeometryFromFile_Response {
  type RmwMsg = super::srv::rmw::LoadGeometryFromFile_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
    }
  }
}


// Corresponds to moveit_msgs__srv__LoadMap_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LoadMap_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub filename: std::string::String,

}



impl Default for LoadMap_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::LoadMap_Request::default())
  }
}

impl rosidl_runtime_rs::Message for LoadMap_Request {
  type RmwMsg = super::srv::rmw::LoadMap_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        filename: msg.filename.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        filename: msg.filename.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      filename: msg.filename.to_string(),
    }
  }
}


// Corresponds to moveit_msgs__srv__LoadMap_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LoadMap_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,

}



impl Default for LoadMap_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::LoadMap_Response::default())
  }
}

impl rosidl_runtime_rs::Message for LoadMap_Response {
  type RmwMsg = super::srv::rmw::LoadMap_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
    }
  }
}


// Corresponds to moveit_msgs__srv__SaveRobotStateToWarehouse_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SaveRobotStateToWarehouse_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub name: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub robot: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub state: super::msg::RobotState,

}



impl Default for SaveRobotStateToWarehouse_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SaveRobotStateToWarehouse_Request::default())
  }
}

impl rosidl_runtime_rs::Message for SaveRobotStateToWarehouse_Request {
  type RmwMsg = super::srv::rmw::SaveRobotStateToWarehouse_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name.as_str().into(),
        robot: msg.robot.as_str().into(),
        state: super::msg::RobotState::into_rmw_message(std::borrow::Cow::Owned(msg.state)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name.as_str().into(),
        robot: msg.robot.as_str().into(),
        state: super::msg::RobotState::into_rmw_message(std::borrow::Cow::Borrowed(&msg.state)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      name: msg.name.to_string(),
      robot: msg.robot.to_string(),
      state: super::msg::RobotState::from_rmw_message(msg.state),
    }
  }
}


// Corresponds to moveit_msgs__srv__SaveRobotStateToWarehouse_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SaveRobotStateToWarehouse_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,

}



impl Default for SaveRobotStateToWarehouse_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SaveRobotStateToWarehouse_Response::default())
  }
}

impl rosidl_runtime_rs::Message for SaveRobotStateToWarehouse_Response {
  type RmwMsg = super::srv::rmw::SaveRobotStateToWarehouse_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
    }
  }
}


// Corresponds to moveit_msgs__srv__ListRobotStatesInWarehouse_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ListRobotStatesInWarehouse_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub regex: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub robot: std::string::String,

}



impl Default for ListRobotStatesInWarehouse_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ListRobotStatesInWarehouse_Request::default())
  }
}

impl rosidl_runtime_rs::Message for ListRobotStatesInWarehouse_Request {
  type RmwMsg = super::srv::rmw::ListRobotStatesInWarehouse_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        regex: msg.regex.as_str().into(),
        robot: msg.robot.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        regex: msg.regex.as_str().into(),
        robot: msg.robot.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      regex: msg.regex.to_string(),
      robot: msg.robot.to_string(),
    }
  }
}


// Corresponds to moveit_msgs__srv__ListRobotStatesInWarehouse_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ListRobotStatesInWarehouse_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub states: Vec<std::string::String>,

}



impl Default for ListRobotStatesInWarehouse_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ListRobotStatesInWarehouse_Response::default())
  }
}

impl rosidl_runtime_rs::Message for ListRobotStatesInWarehouse_Response {
  type RmwMsg = super::srv::rmw::ListRobotStatesInWarehouse_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        states: msg.states
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        states: msg.states
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      states: msg.states
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
    }
  }
}


// Corresponds to moveit_msgs__srv__GetRobotStateFromWarehouse_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetRobotStateFromWarehouse_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub name: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub robot: std::string::String,

}



impl Default for GetRobotStateFromWarehouse_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetRobotStateFromWarehouse_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GetRobotStateFromWarehouse_Request {
  type RmwMsg = super::srv::rmw::GetRobotStateFromWarehouse_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name.as_str().into(),
        robot: msg.robot.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name.as_str().into(),
        robot: msg.robot.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      name: msg.name.to_string(),
      robot: msg.robot.to_string(),
    }
  }
}


// Corresponds to moveit_msgs__srv__GetRobotStateFromWarehouse_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetRobotStateFromWarehouse_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub state: super::msg::RobotState,

}



impl Default for GetRobotStateFromWarehouse_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetRobotStateFromWarehouse_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GetRobotStateFromWarehouse_Response {
  type RmwMsg = super::srv::rmw::GetRobotStateFromWarehouse_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        state: super::msg::RobotState::into_rmw_message(std::borrow::Cow::Owned(msg.state)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        state: super::msg::RobotState::into_rmw_message(std::borrow::Cow::Borrowed(&msg.state)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      state: super::msg::RobotState::from_rmw_message(msg.state),
    }
  }
}


// Corresponds to moveit_msgs__srv__CheckIfRobotStateExistsInWarehouse_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CheckIfRobotStateExistsInWarehouse_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub name: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub robot: std::string::String,

}



impl Default for CheckIfRobotStateExistsInWarehouse_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::CheckIfRobotStateExistsInWarehouse_Request::default())
  }
}

impl rosidl_runtime_rs::Message for CheckIfRobotStateExistsInWarehouse_Request {
  type RmwMsg = super::srv::rmw::CheckIfRobotStateExistsInWarehouse_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name.as_str().into(),
        robot: msg.robot.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name.as_str().into(),
        robot: msg.robot.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      name: msg.name.to_string(),
      robot: msg.robot.to_string(),
    }
  }
}


// Corresponds to moveit_msgs__srv__CheckIfRobotStateExistsInWarehouse_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CheckIfRobotStateExistsInWarehouse_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub exists: bool,

}



impl Default for CheckIfRobotStateExistsInWarehouse_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::CheckIfRobotStateExistsInWarehouse_Response::default())
  }
}

impl rosidl_runtime_rs::Message for CheckIfRobotStateExistsInWarehouse_Response {
  type RmwMsg = super::srv::rmw::CheckIfRobotStateExistsInWarehouse_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        exists: msg.exists,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      exists: msg.exists,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      exists: msg.exists,
    }
  }
}


// Corresponds to moveit_msgs__srv__RenameRobotStateInWarehouse_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RenameRobotStateInWarehouse_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub old_name: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub new_name: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub robot: std::string::String,

}



impl Default for RenameRobotStateInWarehouse_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::RenameRobotStateInWarehouse_Request::default())
  }
}

impl rosidl_runtime_rs::Message for RenameRobotStateInWarehouse_Request {
  type RmwMsg = super::srv::rmw::RenameRobotStateInWarehouse_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        old_name: msg.old_name.as_str().into(),
        new_name: msg.new_name.as_str().into(),
        robot: msg.robot.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        old_name: msg.old_name.as_str().into(),
        new_name: msg.new_name.as_str().into(),
        robot: msg.robot.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      old_name: msg.old_name.to_string(),
      new_name: msg.new_name.to_string(),
      robot: msg.robot.to_string(),
    }
  }
}


// Corresponds to moveit_msgs__srv__RenameRobotStateInWarehouse_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RenameRobotStateInWarehouse_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for RenameRobotStateInWarehouse_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::RenameRobotStateInWarehouse_Response::default())
  }
}

impl rosidl_runtime_rs::Message for RenameRobotStateInWarehouse_Response {
  type RmwMsg = super::srv::rmw::RenameRobotStateInWarehouse_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to moveit_msgs__srv__DeleteRobotStateFromWarehouse_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DeleteRobotStateFromWarehouse_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub name: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub robot: std::string::String,

}



impl Default for DeleteRobotStateFromWarehouse_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::DeleteRobotStateFromWarehouse_Request::default())
  }
}

impl rosidl_runtime_rs::Message for DeleteRobotStateFromWarehouse_Request {
  type RmwMsg = super::srv::rmw::DeleteRobotStateFromWarehouse_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name.as_str().into(),
        robot: msg.robot.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name.as_str().into(),
        robot: msg.robot.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      name: msg.name.to_string(),
      robot: msg.robot.to_string(),
    }
  }
}


// Corresponds to moveit_msgs__srv__DeleteRobotStateFromWarehouse_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DeleteRobotStateFromWarehouse_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for DeleteRobotStateFromWarehouse_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::DeleteRobotStateFromWarehouse_Response::default())
  }
}

impl rosidl_runtime_rs::Message for DeleteRobotStateFromWarehouse_Response {
  type RmwMsg = super::srv::rmw::DeleteRobotStateFromWarehouse_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to moveit_msgs__srv__ServoCommandType_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ServoCommandType_Request::default())
  }
}

impl rosidl_runtime_rs::Message for ServoCommandType_Request {
  type RmwMsg = super::srv::rmw::ServoCommandType_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        command_type: msg.command_type,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      command_type: msg.command_type,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      command_type: msg.command_type,
    }
  }
}


// Corresponds to moveit_msgs__srv__ServoCommandType_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ServoCommandType_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,

}



impl Default for ServoCommandType_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ServoCommandType_Response::default())
  }
}

impl rosidl_runtime_rs::Message for ServoCommandType_Response {
  type RmwMsg = super::srv::rmw::ServoCommandType_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
    }
  }
}


// Corresponds to moveit_msgs__srv__GetGroupUrdf_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetGroupUrdf_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub group_name: std::string::String,

}



impl Default for GetGroupUrdf_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetGroupUrdf_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GetGroupUrdf_Request {
  type RmwMsg = super::srv::rmw::GetGroupUrdf_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        group_name: msg.group_name.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        group_name: msg.group_name.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      group_name: msg.group_name.to_string(),
    }
  }
}


// Corresponds to moveit_msgs__srv__GetGroupUrdf_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetGroupUrdf_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub error_code: super::msg::MoveItErrorCodes,

    /// URDF of the requested group. Empty if request was not successful e.g. because the group does not exist
    pub urdf_string: std::string::String,

}



impl Default for GetGroupUrdf_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetGroupUrdf_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GetGroupUrdf_Response {
  type RmwMsg = super::srv::rmw::GetGroupUrdf_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        error_code: super::msg::MoveItErrorCodes::into_rmw_message(std::borrow::Cow::Owned(msg.error_code)).into_owned(),
        urdf_string: msg.urdf_string.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        error_code: super::msg::MoveItErrorCodes::into_rmw_message(std::borrow::Cow::Borrowed(&msg.error_code)).into_owned(),
        urdf_string: msg.urdf_string.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      error_code: super::msg::MoveItErrorCodes::from_rmw_message(msg.error_code),
      urdf_string: msg.urdf_string.to_string(),
    }
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


