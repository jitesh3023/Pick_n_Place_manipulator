
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



// Corresponds to moveit_msgs__action__ExecuteTrajectory_Goal

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ExecuteTrajectory_Goal {

    // This member is not documented.
    #[allow(missing_docs)]
    pub trajectory: super::msg::RobotTrajectory,

    /// The controller names to use for execution
    pub controller_names: Vec<std::string::String>,

}



impl Default for ExecuteTrajectory_Goal {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::ExecuteTrajectory_Goal::default())
  }
}

impl rosidl_runtime_rs::Message for ExecuteTrajectory_Goal {
  type RmwMsg = super::action::rmw::ExecuteTrajectory_Goal;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        trajectory: super::msg::RobotTrajectory::into_rmw_message(std::borrow::Cow::Owned(msg.trajectory)).into_owned(),
        controller_names: msg.controller_names
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        trajectory: super::msg::RobotTrajectory::into_rmw_message(std::borrow::Cow::Borrowed(&msg.trajectory)).into_owned(),
        controller_names: msg.controller_names
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      trajectory: super::msg::RobotTrajectory::from_rmw_message(msg.trajectory),
      controller_names: msg.controller_names
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
    }
  }
}


// Corresponds to moveit_msgs__action__ExecuteTrajectory_Result

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ExecuteTrajectory_Result {
    /// Error code - encodes the overall reason for failure
    pub error_code: super::msg::MoveItErrorCodes,

}



impl Default for ExecuteTrajectory_Result {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::ExecuteTrajectory_Result::default())
  }
}

impl rosidl_runtime_rs::Message for ExecuteTrajectory_Result {
  type RmwMsg = super::action::rmw::ExecuteTrajectory_Result;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        error_code: super::msg::MoveItErrorCodes::into_rmw_message(std::borrow::Cow::Owned(msg.error_code)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        error_code: super::msg::MoveItErrorCodes::into_rmw_message(std::borrow::Cow::Borrowed(&msg.error_code)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      error_code: super::msg::MoveItErrorCodes::from_rmw_message(msg.error_code),
    }
  }
}


// Corresponds to moveit_msgs__action__ExecuteTrajectory_Feedback

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ExecuteTrajectory_Feedback {
    /// The internal state that the move group action currently is in
    pub state: std::string::String,

}



impl Default for ExecuteTrajectory_Feedback {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::ExecuteTrajectory_Feedback::default())
  }
}

impl rosidl_runtime_rs::Message for ExecuteTrajectory_Feedback {
  type RmwMsg = super::action::rmw::ExecuteTrajectory_Feedback;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        state: msg.state.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        state: msg.state.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      state: msg.state.to_string(),
    }
  }
}


// Corresponds to moveit_msgs__action__ExecuteTrajectory_FeedbackMessage

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ExecuteTrajectory_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::action::ExecuteTrajectory_Feedback,

}



impl Default for ExecuteTrajectory_FeedbackMessage {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::ExecuteTrajectory_FeedbackMessage::default())
  }
}

impl rosidl_runtime_rs::Message for ExecuteTrajectory_FeedbackMessage {
  type RmwMsg = super::action::rmw::ExecuteTrajectory_FeedbackMessage;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
        feedback: super::action::ExecuteTrajectory_Feedback::into_rmw_message(std::borrow::Cow::Owned(msg.feedback)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
        feedback: super::action::ExecuteTrajectory_Feedback::into_rmw_message(std::borrow::Cow::Borrowed(&msg.feedback)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
      feedback: super::action::ExecuteTrajectory_Feedback::from_rmw_message(msg.feedback),
    }
  }
}


// Corresponds to moveit_msgs__action__MoveGroup_Goal

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MoveGroup_Goal {

    // This member is not documented.
    #[allow(missing_docs)]
    pub request: super::msg::MotionPlanRequest,

    /// Planning options
    pub planning_options: super::msg::PlanningOptions,

}



impl Default for MoveGroup_Goal {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::MoveGroup_Goal::default())
  }
}

impl rosidl_runtime_rs::Message for MoveGroup_Goal {
  type RmwMsg = super::action::rmw::MoveGroup_Goal;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        request: super::msg::MotionPlanRequest::into_rmw_message(std::borrow::Cow::Owned(msg.request)).into_owned(),
        planning_options: super::msg::PlanningOptions::into_rmw_message(std::borrow::Cow::Owned(msg.planning_options)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        request: super::msg::MotionPlanRequest::into_rmw_message(std::borrow::Cow::Borrowed(&msg.request)).into_owned(),
        planning_options: super::msg::PlanningOptions::into_rmw_message(std::borrow::Cow::Borrowed(&msg.planning_options)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      request: super::msg::MotionPlanRequest::from_rmw_message(msg.request),
      planning_options: super::msg::PlanningOptions::from_rmw_message(msg.planning_options),
    }
  }
}


// Corresponds to moveit_msgs__action__MoveGroup_Result

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MoveGroup_Result {
    /// An error code reflecting what went wrong
    pub error_code: super::msg::MoveItErrorCodes,

    /// The full starting state of the robot at the start of the trajectory
    pub trajectory_start: super::msg::RobotState,

    /// The trajectory that moved group produced for execution
    pub planned_trajectory: super::msg::RobotTrajectory,

    /// The trace of the trajectory recorded during execution
    pub executed_trajectory: super::msg::RobotTrajectory,

    /// The amount of time it took to complete the motion plan
    pub planning_time: f64,

}



impl Default for MoveGroup_Result {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::MoveGroup_Result::default())
  }
}

impl rosidl_runtime_rs::Message for MoveGroup_Result {
  type RmwMsg = super::action::rmw::MoveGroup_Result;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        error_code: super::msg::MoveItErrorCodes::into_rmw_message(std::borrow::Cow::Owned(msg.error_code)).into_owned(),
        trajectory_start: super::msg::RobotState::into_rmw_message(std::borrow::Cow::Owned(msg.trajectory_start)).into_owned(),
        planned_trajectory: super::msg::RobotTrajectory::into_rmw_message(std::borrow::Cow::Owned(msg.planned_trajectory)).into_owned(),
        executed_trajectory: super::msg::RobotTrajectory::into_rmw_message(std::borrow::Cow::Owned(msg.executed_trajectory)).into_owned(),
        planning_time: msg.planning_time,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        error_code: super::msg::MoveItErrorCodes::into_rmw_message(std::borrow::Cow::Borrowed(&msg.error_code)).into_owned(),
        trajectory_start: super::msg::RobotState::into_rmw_message(std::borrow::Cow::Borrowed(&msg.trajectory_start)).into_owned(),
        planned_trajectory: super::msg::RobotTrajectory::into_rmw_message(std::borrow::Cow::Borrowed(&msg.planned_trajectory)).into_owned(),
        executed_trajectory: super::msg::RobotTrajectory::into_rmw_message(std::borrow::Cow::Borrowed(&msg.executed_trajectory)).into_owned(),
      planning_time: msg.planning_time,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      error_code: super::msg::MoveItErrorCodes::from_rmw_message(msg.error_code),
      trajectory_start: super::msg::RobotState::from_rmw_message(msg.trajectory_start),
      planned_trajectory: super::msg::RobotTrajectory::from_rmw_message(msg.planned_trajectory),
      executed_trajectory: super::msg::RobotTrajectory::from_rmw_message(msg.executed_trajectory),
      planning_time: msg.planning_time,
    }
  }
}


// Corresponds to moveit_msgs__action__MoveGroup_Feedback

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MoveGroup_Feedback {
    /// The internal state that the move group action currently is in
    pub state: std::string::String,

}



impl Default for MoveGroup_Feedback {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::MoveGroup_Feedback::default())
  }
}

impl rosidl_runtime_rs::Message for MoveGroup_Feedback {
  type RmwMsg = super::action::rmw::MoveGroup_Feedback;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        state: msg.state.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        state: msg.state.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      state: msg.state.to_string(),
    }
  }
}


// Corresponds to moveit_msgs__action__MoveGroup_FeedbackMessage

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MoveGroup_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::action::MoveGroup_Feedback,

}



impl Default for MoveGroup_FeedbackMessage {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::MoveGroup_FeedbackMessage::default())
  }
}

impl rosidl_runtime_rs::Message for MoveGroup_FeedbackMessage {
  type RmwMsg = super::action::rmw::MoveGroup_FeedbackMessage;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
        feedback: super::action::MoveGroup_Feedback::into_rmw_message(std::borrow::Cow::Owned(msg.feedback)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
        feedback: super::action::MoveGroup_Feedback::into_rmw_message(std::borrow::Cow::Borrowed(&msg.feedback)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
      feedback: super::action::MoveGroup_Feedback::from_rmw_message(msg.feedback),
    }
  }
}


// Corresponds to moveit_msgs__action__MoveGroupSequence_Goal

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MoveGroupSequence_Goal {

    // This member is not documented.
    #[allow(missing_docs)]
    pub request: super::msg::MotionSequenceRequest,

    /// Planning options
    pub planning_options: super::msg::PlanningOptions,

}



impl Default for MoveGroupSequence_Goal {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::MoveGroupSequence_Goal::default())
  }
}

impl rosidl_runtime_rs::Message for MoveGroupSequence_Goal {
  type RmwMsg = super::action::rmw::MoveGroupSequence_Goal;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        request: super::msg::MotionSequenceRequest::into_rmw_message(std::borrow::Cow::Owned(msg.request)).into_owned(),
        planning_options: super::msg::PlanningOptions::into_rmw_message(std::borrow::Cow::Owned(msg.planning_options)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        request: super::msg::MotionSequenceRequest::into_rmw_message(std::borrow::Cow::Borrowed(&msg.request)).into_owned(),
        planning_options: super::msg::PlanningOptions::into_rmw_message(std::borrow::Cow::Borrowed(&msg.planning_options)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      request: super::msg::MotionSequenceRequest::from_rmw_message(msg.request),
      planning_options: super::msg::PlanningOptions::from_rmw_message(msg.planning_options),
    }
  }
}


// Corresponds to moveit_msgs__action__MoveGroupSequence_Result

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MoveGroupSequence_Result {
    /// Response comprising information on all sections of the sequence
    pub response: super::msg::MotionSequenceResponse,

}



impl Default for MoveGroupSequence_Result {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::MoveGroupSequence_Result::default())
  }
}

impl rosidl_runtime_rs::Message for MoveGroupSequence_Result {
  type RmwMsg = super::action::rmw::MoveGroupSequence_Result;

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


// Corresponds to moveit_msgs__action__MoveGroupSequence_Feedback

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MoveGroupSequence_Feedback {
    /// The internal state that the move group action currently is in
    pub state: std::string::String,

}



impl Default for MoveGroupSequence_Feedback {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::MoveGroupSequence_Feedback::default())
  }
}

impl rosidl_runtime_rs::Message for MoveGroupSequence_Feedback {
  type RmwMsg = super::action::rmw::MoveGroupSequence_Feedback;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        state: msg.state.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        state: msg.state.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      state: msg.state.to_string(),
    }
  }
}


// Corresponds to moveit_msgs__action__MoveGroupSequence_FeedbackMessage

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MoveGroupSequence_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::action::MoveGroupSequence_Feedback,

}



impl Default for MoveGroupSequence_FeedbackMessage {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::MoveGroupSequence_FeedbackMessage::default())
  }
}

impl rosidl_runtime_rs::Message for MoveGroupSequence_FeedbackMessage {
  type RmwMsg = super::action::rmw::MoveGroupSequence_FeedbackMessage;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
        feedback: super::action::MoveGroupSequence_Feedback::into_rmw_message(std::borrow::Cow::Owned(msg.feedback)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
        feedback: super::action::MoveGroupSequence_Feedback::into_rmw_message(std::borrow::Cow::Borrowed(&msg.feedback)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
      feedback: super::action::MoveGroupSequence_Feedback::from_rmw_message(msg.feedback),
    }
  }
}


// Corresponds to moveit_msgs__action__Pickup_Goal

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Pickup_Goal {
    /// The name of the object to pick up (as known in the planning scene)
    pub target_name: std::string::String,

    /// which group should be used to plan for pickup
    pub group_name: std::string::String,

    /// which end-effector to be used for pickup (ideally descending from the group above)
    pub end_effector: std::string::String,

    /// a list of possible grasps to be used. At least one grasp must be filled in
    pub possible_grasps: Vec<super::msg::Grasp>,

    /// the name that the support surface (e.g. table) has in the collision map
    /// can be left empty if no name is available
    pub support_surface_name: std::string::String,

    /// whether collisions between the gripper and the support surface should be acceptable
    /// during move from pre-grasp to grasp and during lift. Collisions when moving to the
    /// pre-grasp location are still not allowed even if this is set to true.
    pub allow_gripper_support_collision: bool,

    /// The names of the links the object to be attached is allowed to touch;
    /// If this is left empty, it defaults to the links in the used end-effector
    pub attached_object_touch_links: Vec<std::string::String>,

    /// Optionally notify the pick action that it should approach the object further,
    /// as much as possible (this minimizing the distance to the object before the grasp)
    /// along the approach direction; Note: this option changes the grasping poses
    /// supplied in possible_grasps[] such that they are closer to the object when possible.
    pub minimize_object_distance: bool,

    /// Optional constraints to be imposed on every point in the motion plan
    pub path_constraints: super::msg::Constraints,

    /// The name of the motion planner to use. If no name is specified,
    /// a default motion planner will be used
    pub planner_id: std::string::String,

    /// an optional list of obstacles that we have semantic information about
    /// and that can be touched/pushed/moved in the course of grasping
    pub allowed_touch_objects: Vec<std::string::String>,

    /// The maximum amount of time the motion planner is allowed to plan for
    pub allowed_planning_time: f64,

    /// Planning options
    pub planning_options: super::msg::PlanningOptions,

}



impl Default for Pickup_Goal {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::Pickup_Goal::default())
  }
}

impl rosidl_runtime_rs::Message for Pickup_Goal {
  type RmwMsg = super::action::rmw::Pickup_Goal;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        target_name: msg.target_name.as_str().into(),
        group_name: msg.group_name.as_str().into(),
        end_effector: msg.end_effector.as_str().into(),
        possible_grasps: msg.possible_grasps
          .into_iter()
          .map(|elem| super::msg::Grasp::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
        support_surface_name: msg.support_surface_name.as_str().into(),
        allow_gripper_support_collision: msg.allow_gripper_support_collision,
        attached_object_touch_links: msg.attached_object_touch_links
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        minimize_object_distance: msg.minimize_object_distance,
        path_constraints: super::msg::Constraints::into_rmw_message(std::borrow::Cow::Owned(msg.path_constraints)).into_owned(),
        planner_id: msg.planner_id.as_str().into(),
        allowed_touch_objects: msg.allowed_touch_objects
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        allowed_planning_time: msg.allowed_planning_time,
        planning_options: super::msg::PlanningOptions::into_rmw_message(std::borrow::Cow::Owned(msg.planning_options)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        target_name: msg.target_name.as_str().into(),
        group_name: msg.group_name.as_str().into(),
        end_effector: msg.end_effector.as_str().into(),
        possible_grasps: msg.possible_grasps
          .iter()
          .map(|elem| super::msg::Grasp::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
        support_surface_name: msg.support_surface_name.as_str().into(),
      allow_gripper_support_collision: msg.allow_gripper_support_collision,
        attached_object_touch_links: msg.attached_object_touch_links
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
      minimize_object_distance: msg.minimize_object_distance,
        path_constraints: super::msg::Constraints::into_rmw_message(std::borrow::Cow::Borrowed(&msg.path_constraints)).into_owned(),
        planner_id: msg.planner_id.as_str().into(),
        allowed_touch_objects: msg.allowed_touch_objects
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
      allowed_planning_time: msg.allowed_planning_time,
        planning_options: super::msg::PlanningOptions::into_rmw_message(std::borrow::Cow::Borrowed(&msg.planning_options)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      target_name: msg.target_name.to_string(),
      group_name: msg.group_name.to_string(),
      end_effector: msg.end_effector.to_string(),
      possible_grasps: msg.possible_grasps
          .into_iter()
          .map(super::msg::Grasp::from_rmw_message)
          .collect(),
      support_surface_name: msg.support_surface_name.to_string(),
      allow_gripper_support_collision: msg.allow_gripper_support_collision,
      attached_object_touch_links: msg.attached_object_touch_links
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
      minimize_object_distance: msg.minimize_object_distance,
      path_constraints: super::msg::Constraints::from_rmw_message(msg.path_constraints),
      planner_id: msg.planner_id.to_string(),
      allowed_touch_objects: msg.allowed_touch_objects
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
      allowed_planning_time: msg.allowed_planning_time,
      planning_options: super::msg::PlanningOptions::from_rmw_message(msg.planning_options),
    }
  }
}


// Corresponds to moveit_msgs__action__Pickup_Result

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Pickup_Result {
    /// The overall result of the pickup attempt
    pub error_code: super::msg::MoveItErrorCodes,

    /// The full starting state of the robot at the start of the trajectory
    pub trajectory_start: super::msg::RobotState,

    /// The trajectory that moved group produced for execution
    pub trajectory_stages: Vec<super::msg::RobotTrajectory>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub trajectory_descriptions: Vec<std::string::String>,

    /// The performed grasp, if attempt was successful
    pub grasp: super::msg::Grasp,

    /// The amount of time in seconds it took to complete the plan
    pub planning_time: f64,

}



impl Default for Pickup_Result {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::Pickup_Result::default())
  }
}

impl rosidl_runtime_rs::Message for Pickup_Result {
  type RmwMsg = super::action::rmw::Pickup_Result;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        error_code: super::msg::MoveItErrorCodes::into_rmw_message(std::borrow::Cow::Owned(msg.error_code)).into_owned(),
        trajectory_start: super::msg::RobotState::into_rmw_message(std::borrow::Cow::Owned(msg.trajectory_start)).into_owned(),
        trajectory_stages: msg.trajectory_stages
          .into_iter()
          .map(|elem| super::msg::RobotTrajectory::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
        trajectory_descriptions: msg.trajectory_descriptions
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        grasp: super::msg::Grasp::into_rmw_message(std::borrow::Cow::Owned(msg.grasp)).into_owned(),
        planning_time: msg.planning_time,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        error_code: super::msg::MoveItErrorCodes::into_rmw_message(std::borrow::Cow::Borrowed(&msg.error_code)).into_owned(),
        trajectory_start: super::msg::RobotState::into_rmw_message(std::borrow::Cow::Borrowed(&msg.trajectory_start)).into_owned(),
        trajectory_stages: msg.trajectory_stages
          .iter()
          .map(|elem| super::msg::RobotTrajectory::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
        trajectory_descriptions: msg.trajectory_descriptions
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        grasp: super::msg::Grasp::into_rmw_message(std::borrow::Cow::Borrowed(&msg.grasp)).into_owned(),
      planning_time: msg.planning_time,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      error_code: super::msg::MoveItErrorCodes::from_rmw_message(msg.error_code),
      trajectory_start: super::msg::RobotState::from_rmw_message(msg.trajectory_start),
      trajectory_stages: msg.trajectory_stages
          .into_iter()
          .map(super::msg::RobotTrajectory::from_rmw_message)
          .collect(),
      trajectory_descriptions: msg.trajectory_descriptions
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
      grasp: super::msg::Grasp::from_rmw_message(msg.grasp),
      planning_time: msg.planning_time,
    }
  }
}


// Corresponds to moveit_msgs__action__Pickup_Feedback

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Pickup_Feedback {
    /// The internal state that the pickup action currently is in
    pub state: std::string::String,

}



impl Default for Pickup_Feedback {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::Pickup_Feedback::default())
  }
}

impl rosidl_runtime_rs::Message for Pickup_Feedback {
  type RmwMsg = super::action::rmw::Pickup_Feedback;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        state: msg.state.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        state: msg.state.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      state: msg.state.to_string(),
    }
  }
}


// Corresponds to moveit_msgs__action__Pickup_FeedbackMessage

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Pickup_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::action::Pickup_Feedback,

}



impl Default for Pickup_FeedbackMessage {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::Pickup_FeedbackMessage::default())
  }
}

impl rosidl_runtime_rs::Message for Pickup_FeedbackMessage {
  type RmwMsg = super::action::rmw::Pickup_FeedbackMessage;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
        feedback: super::action::Pickup_Feedback::into_rmw_message(std::borrow::Cow::Owned(msg.feedback)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
        feedback: super::action::Pickup_Feedback::into_rmw_message(std::borrow::Cow::Borrowed(&msg.feedback)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
      feedback: super::action::Pickup_Feedback::from_rmw_message(msg.feedback),
    }
  }
}


// Corresponds to moveit_msgs__action__Place_Goal

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Place_Goal {
    /// which group to be used to plan for grasping
    pub group_name: std::string::String,

    /// the name of the attached object to place
    pub attached_object_name: std::string::String,

    /// a list of possible transformations for placing the object
    pub place_locations: Vec<super::msg::PlaceLocation>,

    /// if the user prefers setting the eef pose (same as in pick) rather than
    /// the location of the object, this flag should be set to true
    pub place_eef: bool,

    /// the name that the support surface (e.g. table) has in the collision world
    /// can be left empty if no name is available
    pub support_surface_name: std::string::String,

    /// whether collisions between the gripper and the support surface should be acceptable
    /// during move from pre-place to place and during retreat. Collisions when moving to the
    /// pre-place location are still not allowed even if this is set to true.
    pub allow_gripper_support_collision: bool,

    /// Optional constraints to be imposed on every point in the motion plan
    pub path_constraints: super::msg::Constraints,

    /// The name of the motion planner to use. If no name is specified,
    /// a default motion planner will be used
    pub planner_id: std::string::String,

    /// an optional list of obstacles that we have semantic information about
    /// and that can be touched/pushed/moved in the course of placing
    pub allowed_touch_objects: Vec<std::string::String>,

    /// The maximum amount of time the motion planner is allowed to plan for
    pub allowed_planning_time: f64,

    /// Planning options
    pub planning_options: super::msg::PlanningOptions,

}



impl Default for Place_Goal {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::Place_Goal::default())
  }
}

impl rosidl_runtime_rs::Message for Place_Goal {
  type RmwMsg = super::action::rmw::Place_Goal;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        group_name: msg.group_name.as_str().into(),
        attached_object_name: msg.attached_object_name.as_str().into(),
        place_locations: msg.place_locations
          .into_iter()
          .map(|elem| super::msg::PlaceLocation::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
        place_eef: msg.place_eef,
        support_surface_name: msg.support_surface_name.as_str().into(),
        allow_gripper_support_collision: msg.allow_gripper_support_collision,
        path_constraints: super::msg::Constraints::into_rmw_message(std::borrow::Cow::Owned(msg.path_constraints)).into_owned(),
        planner_id: msg.planner_id.as_str().into(),
        allowed_touch_objects: msg.allowed_touch_objects
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        allowed_planning_time: msg.allowed_planning_time,
        planning_options: super::msg::PlanningOptions::into_rmw_message(std::borrow::Cow::Owned(msg.planning_options)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        group_name: msg.group_name.as_str().into(),
        attached_object_name: msg.attached_object_name.as_str().into(),
        place_locations: msg.place_locations
          .iter()
          .map(|elem| super::msg::PlaceLocation::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      place_eef: msg.place_eef,
        support_surface_name: msg.support_surface_name.as_str().into(),
      allow_gripper_support_collision: msg.allow_gripper_support_collision,
        path_constraints: super::msg::Constraints::into_rmw_message(std::borrow::Cow::Borrowed(&msg.path_constraints)).into_owned(),
        planner_id: msg.planner_id.as_str().into(),
        allowed_touch_objects: msg.allowed_touch_objects
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
      allowed_planning_time: msg.allowed_planning_time,
        planning_options: super::msg::PlanningOptions::into_rmw_message(std::borrow::Cow::Borrowed(&msg.planning_options)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      group_name: msg.group_name.to_string(),
      attached_object_name: msg.attached_object_name.to_string(),
      place_locations: msg.place_locations
          .into_iter()
          .map(super::msg::PlaceLocation::from_rmw_message)
          .collect(),
      place_eef: msg.place_eef,
      support_surface_name: msg.support_surface_name.to_string(),
      allow_gripper_support_collision: msg.allow_gripper_support_collision,
      path_constraints: super::msg::Constraints::from_rmw_message(msg.path_constraints),
      planner_id: msg.planner_id.to_string(),
      allowed_touch_objects: msg.allowed_touch_objects
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
      allowed_planning_time: msg.allowed_planning_time,
      planning_options: super::msg::PlanningOptions::from_rmw_message(msg.planning_options),
    }
  }
}


// Corresponds to moveit_msgs__action__Place_Result

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Place_Result {
    /// The result of the place attempt
    pub error_code: super::msg::MoveItErrorCodes,

    /// The full starting state of the robot at the start of the trajectory
    pub trajectory_start: super::msg::RobotState,

    /// The trajectory that moved group produced for execution
    pub trajectory_stages: Vec<super::msg::RobotTrajectory>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub trajectory_descriptions: Vec<std::string::String>,

    /// The successful place location, if any
    pub place_location: super::msg::PlaceLocation,

    /// The amount of time in seconds it took to complete the plan
    pub planning_time: f64,

}



impl Default for Place_Result {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::Place_Result::default())
  }
}

impl rosidl_runtime_rs::Message for Place_Result {
  type RmwMsg = super::action::rmw::Place_Result;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        error_code: super::msg::MoveItErrorCodes::into_rmw_message(std::borrow::Cow::Owned(msg.error_code)).into_owned(),
        trajectory_start: super::msg::RobotState::into_rmw_message(std::borrow::Cow::Owned(msg.trajectory_start)).into_owned(),
        trajectory_stages: msg.trajectory_stages
          .into_iter()
          .map(|elem| super::msg::RobotTrajectory::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
        trajectory_descriptions: msg.trajectory_descriptions
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        place_location: super::msg::PlaceLocation::into_rmw_message(std::borrow::Cow::Owned(msg.place_location)).into_owned(),
        planning_time: msg.planning_time,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        error_code: super::msg::MoveItErrorCodes::into_rmw_message(std::borrow::Cow::Borrowed(&msg.error_code)).into_owned(),
        trajectory_start: super::msg::RobotState::into_rmw_message(std::borrow::Cow::Borrowed(&msg.trajectory_start)).into_owned(),
        trajectory_stages: msg.trajectory_stages
          .iter()
          .map(|elem| super::msg::RobotTrajectory::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
        trajectory_descriptions: msg.trajectory_descriptions
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        place_location: super::msg::PlaceLocation::into_rmw_message(std::borrow::Cow::Borrowed(&msg.place_location)).into_owned(),
      planning_time: msg.planning_time,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      error_code: super::msg::MoveItErrorCodes::from_rmw_message(msg.error_code),
      trajectory_start: super::msg::RobotState::from_rmw_message(msg.trajectory_start),
      trajectory_stages: msg.trajectory_stages
          .into_iter()
          .map(super::msg::RobotTrajectory::from_rmw_message)
          .collect(),
      trajectory_descriptions: msg.trajectory_descriptions
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
      place_location: super::msg::PlaceLocation::from_rmw_message(msg.place_location),
      planning_time: msg.planning_time,
    }
  }
}


// Corresponds to moveit_msgs__action__Place_Feedback

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Place_Feedback {
    /// The internal state that the place action currently is in
    pub state: std::string::String,

}



impl Default for Place_Feedback {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::Place_Feedback::default())
  }
}

impl rosidl_runtime_rs::Message for Place_Feedback {
  type RmwMsg = super::action::rmw::Place_Feedback;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        state: msg.state.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        state: msg.state.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      state: msg.state.to_string(),
    }
  }
}


// Corresponds to moveit_msgs__action__Place_FeedbackMessage

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Place_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::action::Place_Feedback,

}



impl Default for Place_FeedbackMessage {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::Place_FeedbackMessage::default())
  }
}

impl rosidl_runtime_rs::Message for Place_FeedbackMessage {
  type RmwMsg = super::action::rmw::Place_FeedbackMessage;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
        feedback: super::action::Place_Feedback::into_rmw_message(std::borrow::Cow::Owned(msg.feedback)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
        feedback: super::action::Place_Feedback::into_rmw_message(std::borrow::Cow::Borrowed(&msg.feedback)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
      feedback: super::action::Place_Feedback::from_rmw_message(msg.feedback),
    }
  }
}


// Corresponds to moveit_msgs__action__LocalPlanner_Goal

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LocalPlanner_Goal {

    // This member is not documented.
    #[allow(missing_docs)]
    pub local_constraints: Vec<super::msg::Constraints>,

}



impl Default for LocalPlanner_Goal {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::LocalPlanner_Goal::default())
  }
}

impl rosidl_runtime_rs::Message for LocalPlanner_Goal {
  type RmwMsg = super::action::rmw::LocalPlanner_Goal;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        local_constraints: msg.local_constraints
          .into_iter()
          .map(|elem| super::msg::Constraints::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        local_constraints: msg.local_constraints
          .iter()
          .map(|elem| super::msg::Constraints::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      local_constraints: msg.local_constraints
          .into_iter()
          .map(super::msg::Constraints::from_rmw_message)
          .collect(),
    }
  }
}


// Corresponds to moveit_msgs__action__LocalPlanner_Result

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LocalPlanner_Result {

    // This member is not documented.
    #[allow(missing_docs)]
    pub error_code: super::msg::MoveItErrorCodes,

    /// Error message with more detailed information about the local planning result
    pub error_message: std::string::String,

}



impl Default for LocalPlanner_Result {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::LocalPlanner_Result::default())
  }
}

impl rosidl_runtime_rs::Message for LocalPlanner_Result {
  type RmwMsg = super::action::rmw::LocalPlanner_Result;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        error_code: super::msg::MoveItErrorCodes::into_rmw_message(std::borrow::Cow::Owned(msg.error_code)).into_owned(),
        error_message: msg.error_message.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        error_code: super::msg::MoveItErrorCodes::into_rmw_message(std::borrow::Cow::Borrowed(&msg.error_code)).into_owned(),
        error_message: msg.error_message.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      error_code: super::msg::MoveItErrorCodes::from_rmw_message(msg.error_code),
      error_message: msg.error_message.to_string(),
    }
  }
}


// Corresponds to moveit_msgs__action__LocalPlanner_Feedback

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LocalPlanner_Feedback {

    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: std::string::String,

}



impl Default for LocalPlanner_Feedback {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::LocalPlanner_Feedback::default())
  }
}

impl rosidl_runtime_rs::Message for LocalPlanner_Feedback {
  type RmwMsg = super::action::rmw::LocalPlanner_Feedback;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        feedback: msg.feedback.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        feedback: msg.feedback.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      feedback: msg.feedback.to_string(),
    }
  }
}


// Corresponds to moveit_msgs__action__LocalPlanner_FeedbackMessage

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LocalPlanner_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::action::LocalPlanner_Feedback,

}



impl Default for LocalPlanner_FeedbackMessage {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::LocalPlanner_FeedbackMessage::default())
  }
}

impl rosidl_runtime_rs::Message for LocalPlanner_FeedbackMessage {
  type RmwMsg = super::action::rmw::LocalPlanner_FeedbackMessage;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
        feedback: super::action::LocalPlanner_Feedback::into_rmw_message(std::borrow::Cow::Owned(msg.feedback)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
        feedback: super::action::LocalPlanner_Feedback::into_rmw_message(std::borrow::Cow::Borrowed(&msg.feedback)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
      feedback: super::action::LocalPlanner_Feedback::from_rmw_message(msg.feedback),
    }
  }
}


// Corresponds to moveit_msgs__action__GlobalPlanner_Goal

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GlobalPlanner_Goal {

    // This member is not documented.
    #[allow(missing_docs)]
    pub planning_group: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub motion_sequence: super::msg::MotionSequenceRequest,

}



impl Default for GlobalPlanner_Goal {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::GlobalPlanner_Goal::default())
  }
}

impl rosidl_runtime_rs::Message for GlobalPlanner_Goal {
  type RmwMsg = super::action::rmw::GlobalPlanner_Goal;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        planning_group: msg.planning_group.as_str().into(),
        motion_sequence: super::msg::MotionSequenceRequest::into_rmw_message(std::borrow::Cow::Owned(msg.motion_sequence)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        planning_group: msg.planning_group.as_str().into(),
        motion_sequence: super::msg::MotionSequenceRequest::into_rmw_message(std::borrow::Cow::Borrowed(&msg.motion_sequence)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      planning_group: msg.planning_group.to_string(),
      motion_sequence: super::msg::MotionSequenceRequest::from_rmw_message(msg.motion_sequence),
    }
  }
}


// Corresponds to moveit_msgs__action__GlobalPlanner_Result

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GlobalPlanner_Result {

    // This member is not documented.
    #[allow(missing_docs)]
    pub response: super::msg::MotionPlanResponse,

}



impl Default for GlobalPlanner_Result {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::GlobalPlanner_Result::default())
  }
}

impl rosidl_runtime_rs::Message for GlobalPlanner_Result {
  type RmwMsg = super::action::rmw::GlobalPlanner_Result;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        response: super::msg::MotionPlanResponse::into_rmw_message(std::borrow::Cow::Owned(msg.response)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        response: super::msg::MotionPlanResponse::into_rmw_message(std::borrow::Cow::Borrowed(&msg.response)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      response: super::msg::MotionPlanResponse::from_rmw_message(msg.response),
    }
  }
}


// Corresponds to moveit_msgs__action__GlobalPlanner_Feedback

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GlobalPlanner_Feedback {

    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: std::string::String,

}



impl Default for GlobalPlanner_Feedback {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::GlobalPlanner_Feedback::default())
  }
}

impl rosidl_runtime_rs::Message for GlobalPlanner_Feedback {
  type RmwMsg = super::action::rmw::GlobalPlanner_Feedback;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        feedback: msg.feedback.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        feedback: msg.feedback.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      feedback: msg.feedback.to_string(),
    }
  }
}


// Corresponds to moveit_msgs__action__GlobalPlanner_FeedbackMessage

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GlobalPlanner_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::action::GlobalPlanner_Feedback,

}



impl Default for GlobalPlanner_FeedbackMessage {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::GlobalPlanner_FeedbackMessage::default())
  }
}

impl rosidl_runtime_rs::Message for GlobalPlanner_FeedbackMessage {
  type RmwMsg = super::action::rmw::GlobalPlanner_FeedbackMessage;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
        feedback: super::action::GlobalPlanner_Feedback::into_rmw_message(std::borrow::Cow::Owned(msg.feedback)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
        feedback: super::action::GlobalPlanner_Feedback::into_rmw_message(std::borrow::Cow::Borrowed(&msg.feedback)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
      feedback: super::action::GlobalPlanner_Feedback::from_rmw_message(msg.feedback),
    }
  }
}


// Corresponds to moveit_msgs__action__HybridPlanner_Goal

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct HybridPlanner_Goal {

    // This member is not documented.
    #[allow(missing_docs)]
    pub planning_group: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub motion_sequence: super::msg::MotionSequenceRequest,

}



impl Default for HybridPlanner_Goal {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::HybridPlanner_Goal::default())
  }
}

impl rosidl_runtime_rs::Message for HybridPlanner_Goal {
  type RmwMsg = super::action::rmw::HybridPlanner_Goal;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        planning_group: msg.planning_group.as_str().into(),
        motion_sequence: super::msg::MotionSequenceRequest::into_rmw_message(std::borrow::Cow::Owned(msg.motion_sequence)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        planning_group: msg.planning_group.as_str().into(),
        motion_sequence: super::msg::MotionSequenceRequest::into_rmw_message(std::borrow::Cow::Borrowed(&msg.motion_sequence)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      planning_group: msg.planning_group.to_string(),
      motion_sequence: super::msg::MotionSequenceRequest::from_rmw_message(msg.motion_sequence),
    }
  }
}


// Corresponds to moveit_msgs__action__HybridPlanner_Result

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct HybridPlanner_Result {

    // This member is not documented.
    #[allow(missing_docs)]
    pub error_code: super::msg::MoveItErrorCodes,

    /// Error message with more detailed information about the hybrid planning result
    pub error_message: std::string::String,

}



impl Default for HybridPlanner_Result {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::HybridPlanner_Result::default())
  }
}

impl rosidl_runtime_rs::Message for HybridPlanner_Result {
  type RmwMsg = super::action::rmw::HybridPlanner_Result;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        error_code: super::msg::MoveItErrorCodes::into_rmw_message(std::borrow::Cow::Owned(msg.error_code)).into_owned(),
        error_message: msg.error_message.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        error_code: super::msg::MoveItErrorCodes::into_rmw_message(std::borrow::Cow::Borrowed(&msg.error_code)).into_owned(),
        error_message: msg.error_message.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      error_code: super::msg::MoveItErrorCodes::from_rmw_message(msg.error_code),
      error_message: msg.error_message.to_string(),
    }
  }
}


// Corresponds to moveit_msgs__action__HybridPlanner_Feedback

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct HybridPlanner_Feedback {

    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: std::string::String,

}



impl Default for HybridPlanner_Feedback {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::HybridPlanner_Feedback::default())
  }
}

impl rosidl_runtime_rs::Message for HybridPlanner_Feedback {
  type RmwMsg = super::action::rmw::HybridPlanner_Feedback;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        feedback: msg.feedback.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        feedback: msg.feedback.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      feedback: msg.feedback.to_string(),
    }
  }
}


// Corresponds to moveit_msgs__action__HybridPlanner_FeedbackMessage

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct HybridPlanner_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::action::HybridPlanner_Feedback,

}



impl Default for HybridPlanner_FeedbackMessage {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::HybridPlanner_FeedbackMessage::default())
  }
}

impl rosidl_runtime_rs::Message for HybridPlanner_FeedbackMessage {
  type RmwMsg = super::action::rmw::HybridPlanner_FeedbackMessage;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
        feedback: super::action::HybridPlanner_Feedback::into_rmw_message(std::borrow::Cow::Owned(msg.feedback)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
        feedback: super::action::HybridPlanner_Feedback::into_rmw_message(std::borrow::Cow::Borrowed(&msg.feedback)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
      feedback: super::action::HybridPlanner_Feedback::from_rmw_message(msg.feedback),
    }
  }
}






// Corresponds to moveit_msgs__action__ExecuteTrajectory_SendGoal_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ExecuteTrajectory_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::action::ExecuteTrajectory_Goal,

}



impl Default for ExecuteTrajectory_SendGoal_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::ExecuteTrajectory_SendGoal_Request::default())
  }
}

impl rosidl_runtime_rs::Message for ExecuteTrajectory_SendGoal_Request {
  type RmwMsg = super::action::rmw::ExecuteTrajectory_SendGoal_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
        goal: super::action::ExecuteTrajectory_Goal::into_rmw_message(std::borrow::Cow::Owned(msg.goal)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
        goal: super::action::ExecuteTrajectory_Goal::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
      goal: super::action::ExecuteTrajectory_Goal::from_rmw_message(msg.goal),
    }
  }
}


// Corresponds to moveit_msgs__action__ExecuteTrajectory_SendGoal_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ExecuteTrajectory_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::Time,

}



impl Default for ExecuteTrajectory_SendGoal_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::ExecuteTrajectory_SendGoal_Response::default())
  }
}

impl rosidl_runtime_rs::Message for ExecuteTrajectory_SendGoal_Response {
  type RmwMsg = super::action::rmw::ExecuteTrajectory_SendGoal_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        accepted: msg.accepted,
        stamp: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Owned(msg.stamp)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      accepted: msg.accepted,
        stamp: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Borrowed(&msg.stamp)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      accepted: msg.accepted,
      stamp: builtin_interfaces::msg::Time::from_rmw_message(msg.stamp),
    }
  }
}


// Corresponds to moveit_msgs__action__ExecuteTrajectory_GetResult_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ExecuteTrajectory_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,

}



impl Default for ExecuteTrajectory_GetResult_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::ExecuteTrajectory_GetResult_Request::default())
  }
}

impl rosidl_runtime_rs::Message for ExecuteTrajectory_GetResult_Request {
  type RmwMsg = super::action::rmw::ExecuteTrajectory_GetResult_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
    }
  }
}


// Corresponds to moveit_msgs__action__ExecuteTrajectory_GetResult_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ExecuteTrajectory_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::action::ExecuteTrajectory_Result,

}



impl Default for ExecuteTrajectory_GetResult_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::ExecuteTrajectory_GetResult_Response::default())
  }
}

impl rosidl_runtime_rs::Message for ExecuteTrajectory_GetResult_Response {
  type RmwMsg = super::action::rmw::ExecuteTrajectory_GetResult_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        status: msg.status,
        result: super::action::ExecuteTrajectory_Result::into_rmw_message(std::borrow::Cow::Owned(msg.result)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      status: msg.status,
        result: super::action::ExecuteTrajectory_Result::into_rmw_message(std::borrow::Cow::Borrowed(&msg.result)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      status: msg.status,
      result: super::action::ExecuteTrajectory_Result::from_rmw_message(msg.result),
    }
  }
}


// Corresponds to moveit_msgs__action__MoveGroup_SendGoal_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MoveGroup_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::action::MoveGroup_Goal,

}



impl Default for MoveGroup_SendGoal_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::MoveGroup_SendGoal_Request::default())
  }
}

impl rosidl_runtime_rs::Message for MoveGroup_SendGoal_Request {
  type RmwMsg = super::action::rmw::MoveGroup_SendGoal_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
        goal: super::action::MoveGroup_Goal::into_rmw_message(std::borrow::Cow::Owned(msg.goal)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
        goal: super::action::MoveGroup_Goal::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
      goal: super::action::MoveGroup_Goal::from_rmw_message(msg.goal),
    }
  }
}


// Corresponds to moveit_msgs__action__MoveGroup_SendGoal_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MoveGroup_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::Time,

}



impl Default for MoveGroup_SendGoal_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::MoveGroup_SendGoal_Response::default())
  }
}

impl rosidl_runtime_rs::Message for MoveGroup_SendGoal_Response {
  type RmwMsg = super::action::rmw::MoveGroup_SendGoal_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        accepted: msg.accepted,
        stamp: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Owned(msg.stamp)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      accepted: msg.accepted,
        stamp: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Borrowed(&msg.stamp)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      accepted: msg.accepted,
      stamp: builtin_interfaces::msg::Time::from_rmw_message(msg.stamp),
    }
  }
}


// Corresponds to moveit_msgs__action__MoveGroup_GetResult_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MoveGroup_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,

}



impl Default for MoveGroup_GetResult_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::MoveGroup_GetResult_Request::default())
  }
}

impl rosidl_runtime_rs::Message for MoveGroup_GetResult_Request {
  type RmwMsg = super::action::rmw::MoveGroup_GetResult_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
    }
  }
}


// Corresponds to moveit_msgs__action__MoveGroup_GetResult_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MoveGroup_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::action::MoveGroup_Result,

}



impl Default for MoveGroup_GetResult_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::MoveGroup_GetResult_Response::default())
  }
}

impl rosidl_runtime_rs::Message for MoveGroup_GetResult_Response {
  type RmwMsg = super::action::rmw::MoveGroup_GetResult_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        status: msg.status,
        result: super::action::MoveGroup_Result::into_rmw_message(std::borrow::Cow::Owned(msg.result)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      status: msg.status,
        result: super::action::MoveGroup_Result::into_rmw_message(std::borrow::Cow::Borrowed(&msg.result)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      status: msg.status,
      result: super::action::MoveGroup_Result::from_rmw_message(msg.result),
    }
  }
}


// Corresponds to moveit_msgs__action__MoveGroupSequence_SendGoal_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MoveGroupSequence_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::action::MoveGroupSequence_Goal,

}



impl Default for MoveGroupSequence_SendGoal_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::MoveGroupSequence_SendGoal_Request::default())
  }
}

impl rosidl_runtime_rs::Message for MoveGroupSequence_SendGoal_Request {
  type RmwMsg = super::action::rmw::MoveGroupSequence_SendGoal_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
        goal: super::action::MoveGroupSequence_Goal::into_rmw_message(std::borrow::Cow::Owned(msg.goal)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
        goal: super::action::MoveGroupSequence_Goal::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
      goal: super::action::MoveGroupSequence_Goal::from_rmw_message(msg.goal),
    }
  }
}


// Corresponds to moveit_msgs__action__MoveGroupSequence_SendGoal_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MoveGroupSequence_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::Time,

}



impl Default for MoveGroupSequence_SendGoal_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::MoveGroupSequence_SendGoal_Response::default())
  }
}

impl rosidl_runtime_rs::Message for MoveGroupSequence_SendGoal_Response {
  type RmwMsg = super::action::rmw::MoveGroupSequence_SendGoal_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        accepted: msg.accepted,
        stamp: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Owned(msg.stamp)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      accepted: msg.accepted,
        stamp: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Borrowed(&msg.stamp)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      accepted: msg.accepted,
      stamp: builtin_interfaces::msg::Time::from_rmw_message(msg.stamp),
    }
  }
}


// Corresponds to moveit_msgs__action__MoveGroupSequence_GetResult_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MoveGroupSequence_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,

}



impl Default for MoveGroupSequence_GetResult_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::MoveGroupSequence_GetResult_Request::default())
  }
}

impl rosidl_runtime_rs::Message for MoveGroupSequence_GetResult_Request {
  type RmwMsg = super::action::rmw::MoveGroupSequence_GetResult_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
    }
  }
}


// Corresponds to moveit_msgs__action__MoveGroupSequence_GetResult_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MoveGroupSequence_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::action::MoveGroupSequence_Result,

}



impl Default for MoveGroupSequence_GetResult_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::MoveGroupSequence_GetResult_Response::default())
  }
}

impl rosidl_runtime_rs::Message for MoveGroupSequence_GetResult_Response {
  type RmwMsg = super::action::rmw::MoveGroupSequence_GetResult_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        status: msg.status,
        result: super::action::MoveGroupSequence_Result::into_rmw_message(std::borrow::Cow::Owned(msg.result)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      status: msg.status,
        result: super::action::MoveGroupSequence_Result::into_rmw_message(std::borrow::Cow::Borrowed(&msg.result)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      status: msg.status,
      result: super::action::MoveGroupSequence_Result::from_rmw_message(msg.result),
    }
  }
}


// Corresponds to moveit_msgs__action__Pickup_SendGoal_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Pickup_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::action::Pickup_Goal,

}



impl Default for Pickup_SendGoal_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::Pickup_SendGoal_Request::default())
  }
}

impl rosidl_runtime_rs::Message for Pickup_SendGoal_Request {
  type RmwMsg = super::action::rmw::Pickup_SendGoal_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
        goal: super::action::Pickup_Goal::into_rmw_message(std::borrow::Cow::Owned(msg.goal)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
        goal: super::action::Pickup_Goal::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
      goal: super::action::Pickup_Goal::from_rmw_message(msg.goal),
    }
  }
}


// Corresponds to moveit_msgs__action__Pickup_SendGoal_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Pickup_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::Time,

}



impl Default for Pickup_SendGoal_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::Pickup_SendGoal_Response::default())
  }
}

impl rosidl_runtime_rs::Message for Pickup_SendGoal_Response {
  type RmwMsg = super::action::rmw::Pickup_SendGoal_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        accepted: msg.accepted,
        stamp: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Owned(msg.stamp)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      accepted: msg.accepted,
        stamp: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Borrowed(&msg.stamp)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      accepted: msg.accepted,
      stamp: builtin_interfaces::msg::Time::from_rmw_message(msg.stamp),
    }
  }
}


// Corresponds to moveit_msgs__action__Pickup_GetResult_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Pickup_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,

}



impl Default for Pickup_GetResult_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::Pickup_GetResult_Request::default())
  }
}

impl rosidl_runtime_rs::Message for Pickup_GetResult_Request {
  type RmwMsg = super::action::rmw::Pickup_GetResult_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
    }
  }
}


// Corresponds to moveit_msgs__action__Pickup_GetResult_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Pickup_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::action::Pickup_Result,

}



impl Default for Pickup_GetResult_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::Pickup_GetResult_Response::default())
  }
}

impl rosidl_runtime_rs::Message for Pickup_GetResult_Response {
  type RmwMsg = super::action::rmw::Pickup_GetResult_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        status: msg.status,
        result: super::action::Pickup_Result::into_rmw_message(std::borrow::Cow::Owned(msg.result)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      status: msg.status,
        result: super::action::Pickup_Result::into_rmw_message(std::borrow::Cow::Borrowed(&msg.result)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      status: msg.status,
      result: super::action::Pickup_Result::from_rmw_message(msg.result),
    }
  }
}


// Corresponds to moveit_msgs__action__Place_SendGoal_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Place_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::action::Place_Goal,

}



impl Default for Place_SendGoal_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::Place_SendGoal_Request::default())
  }
}

impl rosidl_runtime_rs::Message for Place_SendGoal_Request {
  type RmwMsg = super::action::rmw::Place_SendGoal_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
        goal: super::action::Place_Goal::into_rmw_message(std::borrow::Cow::Owned(msg.goal)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
        goal: super::action::Place_Goal::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
      goal: super::action::Place_Goal::from_rmw_message(msg.goal),
    }
  }
}


// Corresponds to moveit_msgs__action__Place_SendGoal_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Place_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::Time,

}



impl Default for Place_SendGoal_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::Place_SendGoal_Response::default())
  }
}

impl rosidl_runtime_rs::Message for Place_SendGoal_Response {
  type RmwMsg = super::action::rmw::Place_SendGoal_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        accepted: msg.accepted,
        stamp: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Owned(msg.stamp)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      accepted: msg.accepted,
        stamp: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Borrowed(&msg.stamp)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      accepted: msg.accepted,
      stamp: builtin_interfaces::msg::Time::from_rmw_message(msg.stamp),
    }
  }
}


// Corresponds to moveit_msgs__action__Place_GetResult_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Place_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,

}



impl Default for Place_GetResult_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::Place_GetResult_Request::default())
  }
}

impl rosidl_runtime_rs::Message for Place_GetResult_Request {
  type RmwMsg = super::action::rmw::Place_GetResult_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
    }
  }
}


// Corresponds to moveit_msgs__action__Place_GetResult_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Place_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::action::Place_Result,

}



impl Default for Place_GetResult_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::Place_GetResult_Response::default())
  }
}

impl rosidl_runtime_rs::Message for Place_GetResult_Response {
  type RmwMsg = super::action::rmw::Place_GetResult_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        status: msg.status,
        result: super::action::Place_Result::into_rmw_message(std::borrow::Cow::Owned(msg.result)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      status: msg.status,
        result: super::action::Place_Result::into_rmw_message(std::borrow::Cow::Borrowed(&msg.result)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      status: msg.status,
      result: super::action::Place_Result::from_rmw_message(msg.result),
    }
  }
}


// Corresponds to moveit_msgs__action__LocalPlanner_SendGoal_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LocalPlanner_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::action::LocalPlanner_Goal,

}



impl Default for LocalPlanner_SendGoal_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::LocalPlanner_SendGoal_Request::default())
  }
}

impl rosidl_runtime_rs::Message for LocalPlanner_SendGoal_Request {
  type RmwMsg = super::action::rmw::LocalPlanner_SendGoal_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
        goal: super::action::LocalPlanner_Goal::into_rmw_message(std::borrow::Cow::Owned(msg.goal)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
        goal: super::action::LocalPlanner_Goal::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
      goal: super::action::LocalPlanner_Goal::from_rmw_message(msg.goal),
    }
  }
}


// Corresponds to moveit_msgs__action__LocalPlanner_SendGoal_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LocalPlanner_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::Time,

}



impl Default for LocalPlanner_SendGoal_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::LocalPlanner_SendGoal_Response::default())
  }
}

impl rosidl_runtime_rs::Message for LocalPlanner_SendGoal_Response {
  type RmwMsg = super::action::rmw::LocalPlanner_SendGoal_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        accepted: msg.accepted,
        stamp: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Owned(msg.stamp)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      accepted: msg.accepted,
        stamp: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Borrowed(&msg.stamp)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      accepted: msg.accepted,
      stamp: builtin_interfaces::msg::Time::from_rmw_message(msg.stamp),
    }
  }
}


// Corresponds to moveit_msgs__action__LocalPlanner_GetResult_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LocalPlanner_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,

}



impl Default for LocalPlanner_GetResult_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::LocalPlanner_GetResult_Request::default())
  }
}

impl rosidl_runtime_rs::Message for LocalPlanner_GetResult_Request {
  type RmwMsg = super::action::rmw::LocalPlanner_GetResult_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
    }
  }
}


// Corresponds to moveit_msgs__action__LocalPlanner_GetResult_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LocalPlanner_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::action::LocalPlanner_Result,

}



impl Default for LocalPlanner_GetResult_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::LocalPlanner_GetResult_Response::default())
  }
}

impl rosidl_runtime_rs::Message for LocalPlanner_GetResult_Response {
  type RmwMsg = super::action::rmw::LocalPlanner_GetResult_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        status: msg.status,
        result: super::action::LocalPlanner_Result::into_rmw_message(std::borrow::Cow::Owned(msg.result)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      status: msg.status,
        result: super::action::LocalPlanner_Result::into_rmw_message(std::borrow::Cow::Borrowed(&msg.result)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      status: msg.status,
      result: super::action::LocalPlanner_Result::from_rmw_message(msg.result),
    }
  }
}


// Corresponds to moveit_msgs__action__GlobalPlanner_SendGoal_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GlobalPlanner_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::action::GlobalPlanner_Goal,

}



impl Default for GlobalPlanner_SendGoal_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::GlobalPlanner_SendGoal_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GlobalPlanner_SendGoal_Request {
  type RmwMsg = super::action::rmw::GlobalPlanner_SendGoal_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
        goal: super::action::GlobalPlanner_Goal::into_rmw_message(std::borrow::Cow::Owned(msg.goal)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
        goal: super::action::GlobalPlanner_Goal::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
      goal: super::action::GlobalPlanner_Goal::from_rmw_message(msg.goal),
    }
  }
}


// Corresponds to moveit_msgs__action__GlobalPlanner_SendGoal_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GlobalPlanner_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::Time,

}



impl Default for GlobalPlanner_SendGoal_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::GlobalPlanner_SendGoal_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GlobalPlanner_SendGoal_Response {
  type RmwMsg = super::action::rmw::GlobalPlanner_SendGoal_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        accepted: msg.accepted,
        stamp: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Owned(msg.stamp)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      accepted: msg.accepted,
        stamp: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Borrowed(&msg.stamp)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      accepted: msg.accepted,
      stamp: builtin_interfaces::msg::Time::from_rmw_message(msg.stamp),
    }
  }
}


// Corresponds to moveit_msgs__action__GlobalPlanner_GetResult_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GlobalPlanner_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,

}



impl Default for GlobalPlanner_GetResult_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::GlobalPlanner_GetResult_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GlobalPlanner_GetResult_Request {
  type RmwMsg = super::action::rmw::GlobalPlanner_GetResult_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
    }
  }
}


// Corresponds to moveit_msgs__action__GlobalPlanner_GetResult_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GlobalPlanner_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::action::GlobalPlanner_Result,

}



impl Default for GlobalPlanner_GetResult_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::GlobalPlanner_GetResult_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GlobalPlanner_GetResult_Response {
  type RmwMsg = super::action::rmw::GlobalPlanner_GetResult_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        status: msg.status,
        result: super::action::GlobalPlanner_Result::into_rmw_message(std::borrow::Cow::Owned(msg.result)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      status: msg.status,
        result: super::action::GlobalPlanner_Result::into_rmw_message(std::borrow::Cow::Borrowed(&msg.result)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      status: msg.status,
      result: super::action::GlobalPlanner_Result::from_rmw_message(msg.result),
    }
  }
}


// Corresponds to moveit_msgs__action__HybridPlanner_SendGoal_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct HybridPlanner_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::action::HybridPlanner_Goal,

}



impl Default for HybridPlanner_SendGoal_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::HybridPlanner_SendGoal_Request::default())
  }
}

impl rosidl_runtime_rs::Message for HybridPlanner_SendGoal_Request {
  type RmwMsg = super::action::rmw::HybridPlanner_SendGoal_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
        goal: super::action::HybridPlanner_Goal::into_rmw_message(std::borrow::Cow::Owned(msg.goal)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
        goal: super::action::HybridPlanner_Goal::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
      goal: super::action::HybridPlanner_Goal::from_rmw_message(msg.goal),
    }
  }
}


// Corresponds to moveit_msgs__action__HybridPlanner_SendGoal_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct HybridPlanner_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::Time,

}



impl Default for HybridPlanner_SendGoal_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::HybridPlanner_SendGoal_Response::default())
  }
}

impl rosidl_runtime_rs::Message for HybridPlanner_SendGoal_Response {
  type RmwMsg = super::action::rmw::HybridPlanner_SendGoal_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        accepted: msg.accepted,
        stamp: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Owned(msg.stamp)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      accepted: msg.accepted,
        stamp: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Borrowed(&msg.stamp)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      accepted: msg.accepted,
      stamp: builtin_interfaces::msg::Time::from_rmw_message(msg.stamp),
    }
  }
}


// Corresponds to moveit_msgs__action__HybridPlanner_GetResult_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct HybridPlanner_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,

}



impl Default for HybridPlanner_GetResult_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::HybridPlanner_GetResult_Request::default())
  }
}

impl rosidl_runtime_rs::Message for HybridPlanner_GetResult_Request {
  type RmwMsg = super::action::rmw::HybridPlanner_GetResult_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
    }
  }
}


// Corresponds to moveit_msgs__action__HybridPlanner_GetResult_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct HybridPlanner_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::action::HybridPlanner_Result,

}



impl Default for HybridPlanner_GetResult_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::HybridPlanner_GetResult_Response::default())
  }
}

impl rosidl_runtime_rs::Message for HybridPlanner_GetResult_Response {
  type RmwMsg = super::action::rmw::HybridPlanner_GetResult_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        status: msg.status,
        result: super::action::HybridPlanner_Result::into_rmw_message(std::borrow::Cow::Owned(msg.result)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      status: msg.status,
        result: super::action::HybridPlanner_Result::into_rmw_message(std::borrow::Cow::Borrowed(&msg.result)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      status: msg.status,
      result: super::action::HybridPlanner_Result::from_rmw_message(msg.result),
    }
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






#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_action_type_support_handle__moveit_msgs__action__ExecuteTrajectory() -> *const std::ffi::c_void;
}

// Corresponds to moveit_msgs__action__ExecuteTrajectory
#[allow(missing_docs, non_camel_case_types)]
pub struct ExecuteTrajectory;

impl rosidl_runtime_rs::Action for ExecuteTrajectory {
  // --- Associated types for client library users ---
  /// The goal message defined in the action definition.
  type Goal = ExecuteTrajectory_Goal;

  /// The result message defined in the action definition.
  type Result = ExecuteTrajectory_Result;

  /// The feedback message defined in the action definition.
  type Feedback = ExecuteTrajectory_Feedback;

  // --- Associated types for client library implementation ---
  /// The feedback message with generic fields which wraps the feedback message.
  type FeedbackMessage = super::action::ExecuteTrajectory_FeedbackMessage;

  /// The send_goal service using a wrapped version of the goal message as a request.
  type SendGoalService = super::action::ExecuteTrajectory_SendGoal;

  /// The generic service to cancel a goal.
  type CancelGoalService = action_msgs::srv::rmw::CancelGoal;

  /// The get_result service using a wrapped version of the result message as a response.
  type GetResultService = super::action::ExecuteTrajectory_GetResult;

  // --- Methods for client library implementation ---
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_action_type_support_handle__moveit_msgs__action__ExecuteTrajectory() }
  }

  fn create_goal_request(
    goal_id: &[u8; 16],
    goal: super::action::rmw::ExecuteTrajectory_Goal,
  ) -> super::action::rmw::ExecuteTrajectory_SendGoal_Request {
   super::action::rmw::ExecuteTrajectory_SendGoal_Request {
      goal_id: unique_identifier_msgs::msg::rmw::UUID { uuid: *goal_id },
      goal,
    }
  }

  fn split_goal_request(
    request: super::action::rmw::ExecuteTrajectory_SendGoal_Request,
  ) -> (
    [u8; 16],
   super::action::rmw::ExecuteTrajectory_Goal,
  ) {
    (request.goal_id.uuid, request.goal)
  }

  fn create_goal_response(
    accepted: bool,
    stamp: (i32, u32),
  ) -> super::action::rmw::ExecuteTrajectory_SendGoal_Response {
   super::action::rmw::ExecuteTrajectory_SendGoal_Response {
      accepted,
      stamp: builtin_interfaces::msg::rmw::Time {
        sec: stamp.0,
        nanosec: stamp.1,
      },
    }
  }

  fn get_goal_response_accepted(
    response: &super::action::rmw::ExecuteTrajectory_SendGoal_Response,
  ) -> bool {
    response.accepted
  }

  fn get_goal_response_stamp(
    response: &super::action::rmw::ExecuteTrajectory_SendGoal_Response,
  ) -> (i32, u32) {
    (response.stamp.sec, response.stamp.nanosec)
  }

  fn create_feedback_message(
    goal_id: &[u8; 16],
    feedback: super::action::rmw::ExecuteTrajectory_Feedback,
  ) -> super::action::rmw::ExecuteTrajectory_FeedbackMessage {
    let mut message = super::action::rmw::ExecuteTrajectory_FeedbackMessage::default();
    message.goal_id.uuid = *goal_id;
    message.feedback = feedback;
    message
  }

  fn split_feedback_message(
    feedback: super::action::rmw::ExecuteTrajectory_FeedbackMessage,
  ) -> (
    [u8; 16],
   super::action::rmw::ExecuteTrajectory_Feedback,
  ) {
    (feedback.goal_id.uuid, feedback.feedback)
  }

  fn create_result_request(
    goal_id: &[u8; 16],
  ) -> super::action::rmw::ExecuteTrajectory_GetResult_Request {
   super::action::rmw::ExecuteTrajectory_GetResult_Request {
      goal_id: unique_identifier_msgs::msg::rmw::UUID { uuid: *goal_id },
    }
  }

  fn get_result_request_uuid(
    request: &super::action::rmw::ExecuteTrajectory_GetResult_Request,
  ) -> &[u8; 16] {
    &request.goal_id.uuid
  }

  fn create_result_response(
    status: i8,
    result: super::action::rmw::ExecuteTrajectory_Result,
  ) -> super::action::rmw::ExecuteTrajectory_GetResult_Response {
   super::action::rmw::ExecuteTrajectory_GetResult_Response {
      status,
      result,
    }
  }

  fn split_result_response(
    response: super::action::rmw::ExecuteTrajectory_GetResult_Response
  ) -> (
    i8,
   super::action::rmw::ExecuteTrajectory_Result,
  ) {
    (response.status, response.result)
  }
}




#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_action_type_support_handle__moveit_msgs__action__MoveGroup() -> *const std::ffi::c_void;
}

// Corresponds to moveit_msgs__action__MoveGroup
#[allow(missing_docs, non_camel_case_types)]
pub struct MoveGroup;

impl rosidl_runtime_rs::Action for MoveGroup {
  // --- Associated types for client library users ---
  /// The goal message defined in the action definition.
  type Goal = MoveGroup_Goal;

  /// The result message defined in the action definition.
  type Result = MoveGroup_Result;

  /// The feedback message defined in the action definition.
  type Feedback = MoveGroup_Feedback;

  // --- Associated types for client library implementation ---
  /// The feedback message with generic fields which wraps the feedback message.
  type FeedbackMessage = super::action::MoveGroup_FeedbackMessage;

  /// The send_goal service using a wrapped version of the goal message as a request.
  type SendGoalService = super::action::MoveGroup_SendGoal;

  /// The generic service to cancel a goal.
  type CancelGoalService = action_msgs::srv::rmw::CancelGoal;

  /// The get_result service using a wrapped version of the result message as a response.
  type GetResultService = super::action::MoveGroup_GetResult;

  // --- Methods for client library implementation ---
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_action_type_support_handle__moveit_msgs__action__MoveGroup() }
  }

  fn create_goal_request(
    goal_id: &[u8; 16],
    goal: super::action::rmw::MoveGroup_Goal,
  ) -> super::action::rmw::MoveGroup_SendGoal_Request {
   super::action::rmw::MoveGroup_SendGoal_Request {
      goal_id: unique_identifier_msgs::msg::rmw::UUID { uuid: *goal_id },
      goal,
    }
  }

  fn split_goal_request(
    request: super::action::rmw::MoveGroup_SendGoal_Request,
  ) -> (
    [u8; 16],
   super::action::rmw::MoveGroup_Goal,
  ) {
    (request.goal_id.uuid, request.goal)
  }

  fn create_goal_response(
    accepted: bool,
    stamp: (i32, u32),
  ) -> super::action::rmw::MoveGroup_SendGoal_Response {
   super::action::rmw::MoveGroup_SendGoal_Response {
      accepted,
      stamp: builtin_interfaces::msg::rmw::Time {
        sec: stamp.0,
        nanosec: stamp.1,
      },
    }
  }

  fn get_goal_response_accepted(
    response: &super::action::rmw::MoveGroup_SendGoal_Response,
  ) -> bool {
    response.accepted
  }

  fn get_goal_response_stamp(
    response: &super::action::rmw::MoveGroup_SendGoal_Response,
  ) -> (i32, u32) {
    (response.stamp.sec, response.stamp.nanosec)
  }

  fn create_feedback_message(
    goal_id: &[u8; 16],
    feedback: super::action::rmw::MoveGroup_Feedback,
  ) -> super::action::rmw::MoveGroup_FeedbackMessage {
    let mut message = super::action::rmw::MoveGroup_FeedbackMessage::default();
    message.goal_id.uuid = *goal_id;
    message.feedback = feedback;
    message
  }

  fn split_feedback_message(
    feedback: super::action::rmw::MoveGroup_FeedbackMessage,
  ) -> (
    [u8; 16],
   super::action::rmw::MoveGroup_Feedback,
  ) {
    (feedback.goal_id.uuid, feedback.feedback)
  }

  fn create_result_request(
    goal_id: &[u8; 16],
  ) -> super::action::rmw::MoveGroup_GetResult_Request {
   super::action::rmw::MoveGroup_GetResult_Request {
      goal_id: unique_identifier_msgs::msg::rmw::UUID { uuid: *goal_id },
    }
  }

  fn get_result_request_uuid(
    request: &super::action::rmw::MoveGroup_GetResult_Request,
  ) -> &[u8; 16] {
    &request.goal_id.uuid
  }

  fn create_result_response(
    status: i8,
    result: super::action::rmw::MoveGroup_Result,
  ) -> super::action::rmw::MoveGroup_GetResult_Response {
   super::action::rmw::MoveGroup_GetResult_Response {
      status,
      result,
    }
  }

  fn split_result_response(
    response: super::action::rmw::MoveGroup_GetResult_Response
  ) -> (
    i8,
   super::action::rmw::MoveGroup_Result,
  ) {
    (response.status, response.result)
  }
}




#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_action_type_support_handle__moveit_msgs__action__MoveGroupSequence() -> *const std::ffi::c_void;
}

// Corresponds to moveit_msgs__action__MoveGroupSequence
#[allow(missing_docs, non_camel_case_types)]
pub struct MoveGroupSequence;

impl rosidl_runtime_rs::Action for MoveGroupSequence {
  // --- Associated types for client library users ---
  /// The goal message defined in the action definition.
  type Goal = MoveGroupSequence_Goal;

  /// The result message defined in the action definition.
  type Result = MoveGroupSequence_Result;

  /// The feedback message defined in the action definition.
  type Feedback = MoveGroupSequence_Feedback;

  // --- Associated types for client library implementation ---
  /// The feedback message with generic fields which wraps the feedback message.
  type FeedbackMessage = super::action::MoveGroupSequence_FeedbackMessage;

  /// The send_goal service using a wrapped version of the goal message as a request.
  type SendGoalService = super::action::MoveGroupSequence_SendGoal;

  /// The generic service to cancel a goal.
  type CancelGoalService = action_msgs::srv::rmw::CancelGoal;

  /// The get_result service using a wrapped version of the result message as a response.
  type GetResultService = super::action::MoveGroupSequence_GetResult;

  // --- Methods for client library implementation ---
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_action_type_support_handle__moveit_msgs__action__MoveGroupSequence() }
  }

  fn create_goal_request(
    goal_id: &[u8; 16],
    goal: super::action::rmw::MoveGroupSequence_Goal,
  ) -> super::action::rmw::MoveGroupSequence_SendGoal_Request {
   super::action::rmw::MoveGroupSequence_SendGoal_Request {
      goal_id: unique_identifier_msgs::msg::rmw::UUID { uuid: *goal_id },
      goal,
    }
  }

  fn split_goal_request(
    request: super::action::rmw::MoveGroupSequence_SendGoal_Request,
  ) -> (
    [u8; 16],
   super::action::rmw::MoveGroupSequence_Goal,
  ) {
    (request.goal_id.uuid, request.goal)
  }

  fn create_goal_response(
    accepted: bool,
    stamp: (i32, u32),
  ) -> super::action::rmw::MoveGroupSequence_SendGoal_Response {
   super::action::rmw::MoveGroupSequence_SendGoal_Response {
      accepted,
      stamp: builtin_interfaces::msg::rmw::Time {
        sec: stamp.0,
        nanosec: stamp.1,
      },
    }
  }

  fn get_goal_response_accepted(
    response: &super::action::rmw::MoveGroupSequence_SendGoal_Response,
  ) -> bool {
    response.accepted
  }

  fn get_goal_response_stamp(
    response: &super::action::rmw::MoveGroupSequence_SendGoal_Response,
  ) -> (i32, u32) {
    (response.stamp.sec, response.stamp.nanosec)
  }

  fn create_feedback_message(
    goal_id: &[u8; 16],
    feedback: super::action::rmw::MoveGroupSequence_Feedback,
  ) -> super::action::rmw::MoveGroupSequence_FeedbackMessage {
    let mut message = super::action::rmw::MoveGroupSequence_FeedbackMessage::default();
    message.goal_id.uuid = *goal_id;
    message.feedback = feedback;
    message
  }

  fn split_feedback_message(
    feedback: super::action::rmw::MoveGroupSequence_FeedbackMessage,
  ) -> (
    [u8; 16],
   super::action::rmw::MoveGroupSequence_Feedback,
  ) {
    (feedback.goal_id.uuid, feedback.feedback)
  }

  fn create_result_request(
    goal_id: &[u8; 16],
  ) -> super::action::rmw::MoveGroupSequence_GetResult_Request {
   super::action::rmw::MoveGroupSequence_GetResult_Request {
      goal_id: unique_identifier_msgs::msg::rmw::UUID { uuid: *goal_id },
    }
  }

  fn get_result_request_uuid(
    request: &super::action::rmw::MoveGroupSequence_GetResult_Request,
  ) -> &[u8; 16] {
    &request.goal_id.uuid
  }

  fn create_result_response(
    status: i8,
    result: super::action::rmw::MoveGroupSequence_Result,
  ) -> super::action::rmw::MoveGroupSequence_GetResult_Response {
   super::action::rmw::MoveGroupSequence_GetResult_Response {
      status,
      result,
    }
  }

  fn split_result_response(
    response: super::action::rmw::MoveGroupSequence_GetResult_Response
  ) -> (
    i8,
   super::action::rmw::MoveGroupSequence_Result,
  ) {
    (response.status, response.result)
  }
}




#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_action_type_support_handle__moveit_msgs__action__Pickup() -> *const std::ffi::c_void;
}

// Corresponds to moveit_msgs__action__Pickup
#[allow(missing_docs, non_camel_case_types)]
pub struct Pickup;

impl rosidl_runtime_rs::Action for Pickup {
  // --- Associated types for client library users ---
  /// The goal message defined in the action definition.
  type Goal = Pickup_Goal;

  /// The result message defined in the action definition.
  type Result = Pickup_Result;

  /// The feedback message defined in the action definition.
  type Feedback = Pickup_Feedback;

  // --- Associated types for client library implementation ---
  /// The feedback message with generic fields which wraps the feedback message.
  type FeedbackMessage = super::action::Pickup_FeedbackMessage;

  /// The send_goal service using a wrapped version of the goal message as a request.
  type SendGoalService = super::action::Pickup_SendGoal;

  /// The generic service to cancel a goal.
  type CancelGoalService = action_msgs::srv::rmw::CancelGoal;

  /// The get_result service using a wrapped version of the result message as a response.
  type GetResultService = super::action::Pickup_GetResult;

  // --- Methods for client library implementation ---
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_action_type_support_handle__moveit_msgs__action__Pickup() }
  }

  fn create_goal_request(
    goal_id: &[u8; 16],
    goal: super::action::rmw::Pickup_Goal,
  ) -> super::action::rmw::Pickup_SendGoal_Request {
   super::action::rmw::Pickup_SendGoal_Request {
      goal_id: unique_identifier_msgs::msg::rmw::UUID { uuid: *goal_id },
      goal,
    }
  }

  fn split_goal_request(
    request: super::action::rmw::Pickup_SendGoal_Request,
  ) -> (
    [u8; 16],
   super::action::rmw::Pickup_Goal,
  ) {
    (request.goal_id.uuid, request.goal)
  }

  fn create_goal_response(
    accepted: bool,
    stamp: (i32, u32),
  ) -> super::action::rmw::Pickup_SendGoal_Response {
   super::action::rmw::Pickup_SendGoal_Response {
      accepted,
      stamp: builtin_interfaces::msg::rmw::Time {
        sec: stamp.0,
        nanosec: stamp.1,
      },
    }
  }

  fn get_goal_response_accepted(
    response: &super::action::rmw::Pickup_SendGoal_Response,
  ) -> bool {
    response.accepted
  }

  fn get_goal_response_stamp(
    response: &super::action::rmw::Pickup_SendGoal_Response,
  ) -> (i32, u32) {
    (response.stamp.sec, response.stamp.nanosec)
  }

  fn create_feedback_message(
    goal_id: &[u8; 16],
    feedback: super::action::rmw::Pickup_Feedback,
  ) -> super::action::rmw::Pickup_FeedbackMessage {
    let mut message = super::action::rmw::Pickup_FeedbackMessage::default();
    message.goal_id.uuid = *goal_id;
    message.feedback = feedback;
    message
  }

  fn split_feedback_message(
    feedback: super::action::rmw::Pickup_FeedbackMessage,
  ) -> (
    [u8; 16],
   super::action::rmw::Pickup_Feedback,
  ) {
    (feedback.goal_id.uuid, feedback.feedback)
  }

  fn create_result_request(
    goal_id: &[u8; 16],
  ) -> super::action::rmw::Pickup_GetResult_Request {
   super::action::rmw::Pickup_GetResult_Request {
      goal_id: unique_identifier_msgs::msg::rmw::UUID { uuid: *goal_id },
    }
  }

  fn get_result_request_uuid(
    request: &super::action::rmw::Pickup_GetResult_Request,
  ) -> &[u8; 16] {
    &request.goal_id.uuid
  }

  fn create_result_response(
    status: i8,
    result: super::action::rmw::Pickup_Result,
  ) -> super::action::rmw::Pickup_GetResult_Response {
   super::action::rmw::Pickup_GetResult_Response {
      status,
      result,
    }
  }

  fn split_result_response(
    response: super::action::rmw::Pickup_GetResult_Response
  ) -> (
    i8,
   super::action::rmw::Pickup_Result,
  ) {
    (response.status, response.result)
  }
}




#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_action_type_support_handle__moveit_msgs__action__Place() -> *const std::ffi::c_void;
}

// Corresponds to moveit_msgs__action__Place
#[allow(missing_docs, non_camel_case_types)]
pub struct Place;

impl rosidl_runtime_rs::Action for Place {
  // --- Associated types for client library users ---
  /// The goal message defined in the action definition.
  type Goal = Place_Goal;

  /// The result message defined in the action definition.
  type Result = Place_Result;

  /// The feedback message defined in the action definition.
  type Feedback = Place_Feedback;

  // --- Associated types for client library implementation ---
  /// The feedback message with generic fields which wraps the feedback message.
  type FeedbackMessage = super::action::Place_FeedbackMessage;

  /// The send_goal service using a wrapped version of the goal message as a request.
  type SendGoalService = super::action::Place_SendGoal;

  /// The generic service to cancel a goal.
  type CancelGoalService = action_msgs::srv::rmw::CancelGoal;

  /// The get_result service using a wrapped version of the result message as a response.
  type GetResultService = super::action::Place_GetResult;

  // --- Methods for client library implementation ---
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_action_type_support_handle__moveit_msgs__action__Place() }
  }

  fn create_goal_request(
    goal_id: &[u8; 16],
    goal: super::action::rmw::Place_Goal,
  ) -> super::action::rmw::Place_SendGoal_Request {
   super::action::rmw::Place_SendGoal_Request {
      goal_id: unique_identifier_msgs::msg::rmw::UUID { uuid: *goal_id },
      goal,
    }
  }

  fn split_goal_request(
    request: super::action::rmw::Place_SendGoal_Request,
  ) -> (
    [u8; 16],
   super::action::rmw::Place_Goal,
  ) {
    (request.goal_id.uuid, request.goal)
  }

  fn create_goal_response(
    accepted: bool,
    stamp: (i32, u32),
  ) -> super::action::rmw::Place_SendGoal_Response {
   super::action::rmw::Place_SendGoal_Response {
      accepted,
      stamp: builtin_interfaces::msg::rmw::Time {
        sec: stamp.0,
        nanosec: stamp.1,
      },
    }
  }

  fn get_goal_response_accepted(
    response: &super::action::rmw::Place_SendGoal_Response,
  ) -> bool {
    response.accepted
  }

  fn get_goal_response_stamp(
    response: &super::action::rmw::Place_SendGoal_Response,
  ) -> (i32, u32) {
    (response.stamp.sec, response.stamp.nanosec)
  }

  fn create_feedback_message(
    goal_id: &[u8; 16],
    feedback: super::action::rmw::Place_Feedback,
  ) -> super::action::rmw::Place_FeedbackMessage {
    let mut message = super::action::rmw::Place_FeedbackMessage::default();
    message.goal_id.uuid = *goal_id;
    message.feedback = feedback;
    message
  }

  fn split_feedback_message(
    feedback: super::action::rmw::Place_FeedbackMessage,
  ) -> (
    [u8; 16],
   super::action::rmw::Place_Feedback,
  ) {
    (feedback.goal_id.uuid, feedback.feedback)
  }

  fn create_result_request(
    goal_id: &[u8; 16],
  ) -> super::action::rmw::Place_GetResult_Request {
   super::action::rmw::Place_GetResult_Request {
      goal_id: unique_identifier_msgs::msg::rmw::UUID { uuid: *goal_id },
    }
  }

  fn get_result_request_uuid(
    request: &super::action::rmw::Place_GetResult_Request,
  ) -> &[u8; 16] {
    &request.goal_id.uuid
  }

  fn create_result_response(
    status: i8,
    result: super::action::rmw::Place_Result,
  ) -> super::action::rmw::Place_GetResult_Response {
   super::action::rmw::Place_GetResult_Response {
      status,
      result,
    }
  }

  fn split_result_response(
    response: super::action::rmw::Place_GetResult_Response
  ) -> (
    i8,
   super::action::rmw::Place_Result,
  ) {
    (response.status, response.result)
  }
}




#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_action_type_support_handle__moveit_msgs__action__LocalPlanner() -> *const std::ffi::c_void;
}

// Corresponds to moveit_msgs__action__LocalPlanner
#[allow(missing_docs, non_camel_case_types)]
pub struct LocalPlanner;

impl rosidl_runtime_rs::Action for LocalPlanner {
  // --- Associated types for client library users ---
  /// The goal message defined in the action definition.
  type Goal = LocalPlanner_Goal;

  /// The result message defined in the action definition.
  type Result = LocalPlanner_Result;

  /// The feedback message defined in the action definition.
  type Feedback = LocalPlanner_Feedback;

  // --- Associated types for client library implementation ---
  /// The feedback message with generic fields which wraps the feedback message.
  type FeedbackMessage = super::action::LocalPlanner_FeedbackMessage;

  /// The send_goal service using a wrapped version of the goal message as a request.
  type SendGoalService = super::action::LocalPlanner_SendGoal;

  /// The generic service to cancel a goal.
  type CancelGoalService = action_msgs::srv::rmw::CancelGoal;

  /// The get_result service using a wrapped version of the result message as a response.
  type GetResultService = super::action::LocalPlanner_GetResult;

  // --- Methods for client library implementation ---
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_action_type_support_handle__moveit_msgs__action__LocalPlanner() }
  }

  fn create_goal_request(
    goal_id: &[u8; 16],
    goal: super::action::rmw::LocalPlanner_Goal,
  ) -> super::action::rmw::LocalPlanner_SendGoal_Request {
   super::action::rmw::LocalPlanner_SendGoal_Request {
      goal_id: unique_identifier_msgs::msg::rmw::UUID { uuid: *goal_id },
      goal,
    }
  }

  fn split_goal_request(
    request: super::action::rmw::LocalPlanner_SendGoal_Request,
  ) -> (
    [u8; 16],
   super::action::rmw::LocalPlanner_Goal,
  ) {
    (request.goal_id.uuid, request.goal)
  }

  fn create_goal_response(
    accepted: bool,
    stamp: (i32, u32),
  ) -> super::action::rmw::LocalPlanner_SendGoal_Response {
   super::action::rmw::LocalPlanner_SendGoal_Response {
      accepted,
      stamp: builtin_interfaces::msg::rmw::Time {
        sec: stamp.0,
        nanosec: stamp.1,
      },
    }
  }

  fn get_goal_response_accepted(
    response: &super::action::rmw::LocalPlanner_SendGoal_Response,
  ) -> bool {
    response.accepted
  }

  fn get_goal_response_stamp(
    response: &super::action::rmw::LocalPlanner_SendGoal_Response,
  ) -> (i32, u32) {
    (response.stamp.sec, response.stamp.nanosec)
  }

  fn create_feedback_message(
    goal_id: &[u8; 16],
    feedback: super::action::rmw::LocalPlanner_Feedback,
  ) -> super::action::rmw::LocalPlanner_FeedbackMessage {
    let mut message = super::action::rmw::LocalPlanner_FeedbackMessage::default();
    message.goal_id.uuid = *goal_id;
    message.feedback = feedback;
    message
  }

  fn split_feedback_message(
    feedback: super::action::rmw::LocalPlanner_FeedbackMessage,
  ) -> (
    [u8; 16],
   super::action::rmw::LocalPlanner_Feedback,
  ) {
    (feedback.goal_id.uuid, feedback.feedback)
  }

  fn create_result_request(
    goal_id: &[u8; 16],
  ) -> super::action::rmw::LocalPlanner_GetResult_Request {
   super::action::rmw::LocalPlanner_GetResult_Request {
      goal_id: unique_identifier_msgs::msg::rmw::UUID { uuid: *goal_id },
    }
  }

  fn get_result_request_uuid(
    request: &super::action::rmw::LocalPlanner_GetResult_Request,
  ) -> &[u8; 16] {
    &request.goal_id.uuid
  }

  fn create_result_response(
    status: i8,
    result: super::action::rmw::LocalPlanner_Result,
  ) -> super::action::rmw::LocalPlanner_GetResult_Response {
   super::action::rmw::LocalPlanner_GetResult_Response {
      status,
      result,
    }
  }

  fn split_result_response(
    response: super::action::rmw::LocalPlanner_GetResult_Response
  ) -> (
    i8,
   super::action::rmw::LocalPlanner_Result,
  ) {
    (response.status, response.result)
  }
}




#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_action_type_support_handle__moveit_msgs__action__GlobalPlanner() -> *const std::ffi::c_void;
}

// Corresponds to moveit_msgs__action__GlobalPlanner
#[allow(missing_docs, non_camel_case_types)]
pub struct GlobalPlanner;

impl rosidl_runtime_rs::Action for GlobalPlanner {
  // --- Associated types for client library users ---
  /// The goal message defined in the action definition.
  type Goal = GlobalPlanner_Goal;

  /// The result message defined in the action definition.
  type Result = GlobalPlanner_Result;

  /// The feedback message defined in the action definition.
  type Feedback = GlobalPlanner_Feedback;

  // --- Associated types for client library implementation ---
  /// The feedback message with generic fields which wraps the feedback message.
  type FeedbackMessage = super::action::GlobalPlanner_FeedbackMessage;

  /// The send_goal service using a wrapped version of the goal message as a request.
  type SendGoalService = super::action::GlobalPlanner_SendGoal;

  /// The generic service to cancel a goal.
  type CancelGoalService = action_msgs::srv::rmw::CancelGoal;

  /// The get_result service using a wrapped version of the result message as a response.
  type GetResultService = super::action::GlobalPlanner_GetResult;

  // --- Methods for client library implementation ---
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_action_type_support_handle__moveit_msgs__action__GlobalPlanner() }
  }

  fn create_goal_request(
    goal_id: &[u8; 16],
    goal: super::action::rmw::GlobalPlanner_Goal,
  ) -> super::action::rmw::GlobalPlanner_SendGoal_Request {
   super::action::rmw::GlobalPlanner_SendGoal_Request {
      goal_id: unique_identifier_msgs::msg::rmw::UUID { uuid: *goal_id },
      goal,
    }
  }

  fn split_goal_request(
    request: super::action::rmw::GlobalPlanner_SendGoal_Request,
  ) -> (
    [u8; 16],
   super::action::rmw::GlobalPlanner_Goal,
  ) {
    (request.goal_id.uuid, request.goal)
  }

  fn create_goal_response(
    accepted: bool,
    stamp: (i32, u32),
  ) -> super::action::rmw::GlobalPlanner_SendGoal_Response {
   super::action::rmw::GlobalPlanner_SendGoal_Response {
      accepted,
      stamp: builtin_interfaces::msg::rmw::Time {
        sec: stamp.0,
        nanosec: stamp.1,
      },
    }
  }

  fn get_goal_response_accepted(
    response: &super::action::rmw::GlobalPlanner_SendGoal_Response,
  ) -> bool {
    response.accepted
  }

  fn get_goal_response_stamp(
    response: &super::action::rmw::GlobalPlanner_SendGoal_Response,
  ) -> (i32, u32) {
    (response.stamp.sec, response.stamp.nanosec)
  }

  fn create_feedback_message(
    goal_id: &[u8; 16],
    feedback: super::action::rmw::GlobalPlanner_Feedback,
  ) -> super::action::rmw::GlobalPlanner_FeedbackMessage {
    let mut message = super::action::rmw::GlobalPlanner_FeedbackMessage::default();
    message.goal_id.uuid = *goal_id;
    message.feedback = feedback;
    message
  }

  fn split_feedback_message(
    feedback: super::action::rmw::GlobalPlanner_FeedbackMessage,
  ) -> (
    [u8; 16],
   super::action::rmw::GlobalPlanner_Feedback,
  ) {
    (feedback.goal_id.uuid, feedback.feedback)
  }

  fn create_result_request(
    goal_id: &[u8; 16],
  ) -> super::action::rmw::GlobalPlanner_GetResult_Request {
   super::action::rmw::GlobalPlanner_GetResult_Request {
      goal_id: unique_identifier_msgs::msg::rmw::UUID { uuid: *goal_id },
    }
  }

  fn get_result_request_uuid(
    request: &super::action::rmw::GlobalPlanner_GetResult_Request,
  ) -> &[u8; 16] {
    &request.goal_id.uuid
  }

  fn create_result_response(
    status: i8,
    result: super::action::rmw::GlobalPlanner_Result,
  ) -> super::action::rmw::GlobalPlanner_GetResult_Response {
   super::action::rmw::GlobalPlanner_GetResult_Response {
      status,
      result,
    }
  }

  fn split_result_response(
    response: super::action::rmw::GlobalPlanner_GetResult_Response
  ) -> (
    i8,
   super::action::rmw::GlobalPlanner_Result,
  ) {
    (response.status, response.result)
  }
}




#[link(name = "moveit_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_action_type_support_handle__moveit_msgs__action__HybridPlanner() -> *const std::ffi::c_void;
}

// Corresponds to moveit_msgs__action__HybridPlanner
#[allow(missing_docs, non_camel_case_types)]
pub struct HybridPlanner;

impl rosidl_runtime_rs::Action for HybridPlanner {
  // --- Associated types for client library users ---
  /// The goal message defined in the action definition.
  type Goal = HybridPlanner_Goal;

  /// The result message defined in the action definition.
  type Result = HybridPlanner_Result;

  /// The feedback message defined in the action definition.
  type Feedback = HybridPlanner_Feedback;

  // --- Associated types for client library implementation ---
  /// The feedback message with generic fields which wraps the feedback message.
  type FeedbackMessage = super::action::HybridPlanner_FeedbackMessage;

  /// The send_goal service using a wrapped version of the goal message as a request.
  type SendGoalService = super::action::HybridPlanner_SendGoal;

  /// The generic service to cancel a goal.
  type CancelGoalService = action_msgs::srv::rmw::CancelGoal;

  /// The get_result service using a wrapped version of the result message as a response.
  type GetResultService = super::action::HybridPlanner_GetResult;

  // --- Methods for client library implementation ---
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_action_type_support_handle__moveit_msgs__action__HybridPlanner() }
  }

  fn create_goal_request(
    goal_id: &[u8; 16],
    goal: super::action::rmw::HybridPlanner_Goal,
  ) -> super::action::rmw::HybridPlanner_SendGoal_Request {
   super::action::rmw::HybridPlanner_SendGoal_Request {
      goal_id: unique_identifier_msgs::msg::rmw::UUID { uuid: *goal_id },
      goal,
    }
  }

  fn split_goal_request(
    request: super::action::rmw::HybridPlanner_SendGoal_Request,
  ) -> (
    [u8; 16],
   super::action::rmw::HybridPlanner_Goal,
  ) {
    (request.goal_id.uuid, request.goal)
  }

  fn create_goal_response(
    accepted: bool,
    stamp: (i32, u32),
  ) -> super::action::rmw::HybridPlanner_SendGoal_Response {
   super::action::rmw::HybridPlanner_SendGoal_Response {
      accepted,
      stamp: builtin_interfaces::msg::rmw::Time {
        sec: stamp.0,
        nanosec: stamp.1,
      },
    }
  }

  fn get_goal_response_accepted(
    response: &super::action::rmw::HybridPlanner_SendGoal_Response,
  ) -> bool {
    response.accepted
  }

  fn get_goal_response_stamp(
    response: &super::action::rmw::HybridPlanner_SendGoal_Response,
  ) -> (i32, u32) {
    (response.stamp.sec, response.stamp.nanosec)
  }

  fn create_feedback_message(
    goal_id: &[u8; 16],
    feedback: super::action::rmw::HybridPlanner_Feedback,
  ) -> super::action::rmw::HybridPlanner_FeedbackMessage {
    let mut message = super::action::rmw::HybridPlanner_FeedbackMessage::default();
    message.goal_id.uuid = *goal_id;
    message.feedback = feedback;
    message
  }

  fn split_feedback_message(
    feedback: super::action::rmw::HybridPlanner_FeedbackMessage,
  ) -> (
    [u8; 16],
   super::action::rmw::HybridPlanner_Feedback,
  ) {
    (feedback.goal_id.uuid, feedback.feedback)
  }

  fn create_result_request(
    goal_id: &[u8; 16],
  ) -> super::action::rmw::HybridPlanner_GetResult_Request {
   super::action::rmw::HybridPlanner_GetResult_Request {
      goal_id: unique_identifier_msgs::msg::rmw::UUID { uuid: *goal_id },
    }
  }

  fn get_result_request_uuid(
    request: &super::action::rmw::HybridPlanner_GetResult_Request,
  ) -> &[u8; 16] {
    &request.goal_id.uuid
  }

  fn create_result_response(
    status: i8,
    result: super::action::rmw::HybridPlanner_Result,
  ) -> super::action::rmw::HybridPlanner_GetResult_Response {
   super::action::rmw::HybridPlanner_GetResult_Response {
      status,
      result,
    }
  }

  fn split_result_response(
    response: super::action::rmw::HybridPlanner_GetResult_Response
  ) -> (
    i8,
   super::action::rmw::HybridPlanner_Result,
  ) {
    (response.status, response.result)
  }
}


