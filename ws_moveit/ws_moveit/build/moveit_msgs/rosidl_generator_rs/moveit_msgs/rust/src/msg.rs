#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



// Corresponds to moveit_msgs__msg__AllowedCollisionEntry
/// whether or not collision checking is enabled

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AllowedCollisionEntry {

    // This member is not documented.
    #[allow(missing_docs)]
    pub enabled: Vec<bool>,

}



impl Default for AllowedCollisionEntry {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::AllowedCollisionEntry::default())
  }
}

impl rosidl_runtime_rs::Message for AllowedCollisionEntry {
  type RmwMsg = super::msg::rmw::AllowedCollisionEntry;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        enabled: msg.enabled.as_slice().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        enabled: msg.enabled.as_slice().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      enabled: msg.enabled.into(),
    }
  }
}


// Corresponds to moveit_msgs__msg__AllowedCollisionMatrix
/// The list of entry names in the matrix

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AllowedCollisionMatrix {

    // This member is not documented.
    #[allow(missing_docs)]
    pub entry_names: Vec<std::string::String>,

    /// The individual entries in the allowed collision matrix
    /// square, symmetric, with same order as entry_names
    pub entry_values: Vec<super::msg::AllowedCollisionEntry>,

    /// In addition to the collision matrix itself, we also have
    /// the default entry value for each entry name.
    /// If the allowed collision flag is queried for a pair of names (n1, n2)
    /// that is not found in the collision matrix itself, the value of
    /// the collision flag is considered to be that of the entry (n1 or n2)
    /// specified in the list below. If both n1 and n2 are found in the list
    /// of defaults, the result is computed with an AND operation
    pub default_entry_names: Vec<std::string::String>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub default_entry_values: Vec<bool>,

}



impl Default for AllowedCollisionMatrix {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::AllowedCollisionMatrix::default())
  }
}

impl rosidl_runtime_rs::Message for AllowedCollisionMatrix {
  type RmwMsg = super::msg::rmw::AllowedCollisionMatrix;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        entry_names: msg.entry_names
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        entry_values: msg.entry_values
          .into_iter()
          .map(|elem| super::msg::AllowedCollisionEntry::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
        default_entry_names: msg.default_entry_names
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        default_entry_values: msg.default_entry_values.as_slice().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        entry_names: msg.entry_names
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        entry_values: msg.entry_values
          .iter()
          .map(|elem| super::msg::AllowedCollisionEntry::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
        default_entry_names: msg.default_entry_names
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        default_entry_values: msg.default_entry_values.as_slice().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      entry_names: msg.entry_names
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
      entry_values: msg.entry_values
          .into_iter()
          .map(super::msg::AllowedCollisionEntry::from_rmw_message)
          .collect(),
      default_entry_names: msg.default_entry_names
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
      default_entry_values: msg.default_entry_values.into(),
    }
  }
}


// Corresponds to moveit_msgs__msg__AttachedCollisionObject
/// The CollisionObject will be attached with a fixed joint to this link

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AttachedCollisionObject {

    // This member is not documented.
    #[allow(missing_docs)]
    pub link_name: std::string::String,

    /// This contains the actual shapes and poses for the CollisionObject
    /// to be attached to the link
    /// If action is remove and no object.id is set, all objects
    /// attached to the link indicated by link_name will be removed
    pub object: super::msg::CollisionObject,

    /// The set of links that the attached objects are allowed to touch
    /// by default - the link_name is already considered by default
    pub touch_links: Vec<std::string::String>,

    /// If certain links were placed in a particular posture for this object to remain attached
    /// (e.g., an end effector closing around an object), the posture necessary for releasing
    /// the object is stored here
    pub detach_posture: trajectory_msgs::msg::JointTrajectory,

    /// The weight of the attached object, if known
    pub weight: f64,

}



impl Default for AttachedCollisionObject {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::AttachedCollisionObject::default())
  }
}

impl rosidl_runtime_rs::Message for AttachedCollisionObject {
  type RmwMsg = super::msg::rmw::AttachedCollisionObject;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        link_name: msg.link_name.as_str().into(),
        object: super::msg::CollisionObject::into_rmw_message(std::borrow::Cow::Owned(msg.object)).into_owned(),
        touch_links: msg.touch_links
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        detach_posture: trajectory_msgs::msg::JointTrajectory::into_rmw_message(std::borrow::Cow::Owned(msg.detach_posture)).into_owned(),
        weight: msg.weight,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        link_name: msg.link_name.as_str().into(),
        object: super::msg::CollisionObject::into_rmw_message(std::borrow::Cow::Borrowed(&msg.object)).into_owned(),
        touch_links: msg.touch_links
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        detach_posture: trajectory_msgs::msg::JointTrajectory::into_rmw_message(std::borrow::Cow::Borrowed(&msg.detach_posture)).into_owned(),
      weight: msg.weight,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      link_name: msg.link_name.to_string(),
      object: super::msg::CollisionObject::from_rmw_message(msg.object),
      touch_links: msg.touch_links
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
      detach_posture: trajectory_msgs::msg::JointTrajectory::from_rmw_message(msg.detach_posture),
      weight: msg.weight,
    }
  }
}


// Corresponds to moveit_msgs__msg__BoundingVolume
/// Define a volume in 3D

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BoundingVolume {
    /// A set of solid geometric primitives that make up the volume to define (as a union)
    pub primitives: Vec<shape_msgs::msg::SolidPrimitive>,

    /// The poses at which the primitives are located
    pub primitive_poses: Vec<geometry_msgs::msg::Pose>,

    /// In addition to primitives, meshes can be specified to add to the bounding volume (again, as union)
    pub meshes: Vec<shape_msgs::msg::Mesh>,

    /// The poses at which the meshes are located
    pub mesh_poses: Vec<geometry_msgs::msg::Pose>,

}



impl Default for BoundingVolume {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::BoundingVolume::default())
  }
}

impl rosidl_runtime_rs::Message for BoundingVolume {
  type RmwMsg = super::msg::rmw::BoundingVolume;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        primitives: msg.primitives
          .into_iter()
          .map(|elem| shape_msgs::msg::SolidPrimitive::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
        primitive_poses: msg.primitive_poses
          .into_iter()
          .map(|elem| geometry_msgs::msg::Pose::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
        meshes: msg.meshes
          .into_iter()
          .map(|elem| shape_msgs::msg::Mesh::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
        mesh_poses: msg.mesh_poses
          .into_iter()
          .map(|elem| geometry_msgs::msg::Pose::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        primitives: msg.primitives
          .iter()
          .map(|elem| shape_msgs::msg::SolidPrimitive::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
        primitive_poses: msg.primitive_poses
          .iter()
          .map(|elem| geometry_msgs::msg::Pose::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
        meshes: msg.meshes
          .iter()
          .map(|elem| shape_msgs::msg::Mesh::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
        mesh_poses: msg.mesh_poses
          .iter()
          .map(|elem| geometry_msgs::msg::Pose::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      primitives: msg.primitives
          .into_iter()
          .map(shape_msgs::msg::SolidPrimitive::from_rmw_message)
          .collect(),
      primitive_poses: msg.primitive_poses
          .into_iter()
          .map(geometry_msgs::msg::Pose::from_rmw_message)
          .collect(),
      meshes: msg.meshes
          .into_iter()
          .map(shape_msgs::msg::Mesh::from_rmw_message)
          .collect(),
      mesh_poses: msg.mesh_poses
          .into_iter()
          .map(geometry_msgs::msg::Pose::from_rmw_message)
          .collect(),
    }
  }
}


// Corresponds to moveit_msgs__msg__CartesianPoint
/// This message defines a point in a cartesian trajectory

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CartesianPoint {

    // This member is not documented.
    #[allow(missing_docs)]
    pub pose: geometry_msgs::msg::Pose,


    // This member is not documented.
    #[allow(missing_docs)]
    pub velocity: geometry_msgs::msg::Twist,


    // This member is not documented.
    #[allow(missing_docs)]
    pub acceleration: geometry_msgs::msg::Accel,

}



impl Default for CartesianPoint {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::CartesianPoint::default())
  }
}

impl rosidl_runtime_rs::Message for CartesianPoint {
  type RmwMsg = super::msg::rmw::CartesianPoint;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        pose: geometry_msgs::msg::Pose::into_rmw_message(std::borrow::Cow::Owned(msg.pose)).into_owned(),
        velocity: geometry_msgs::msg::Twist::into_rmw_message(std::borrow::Cow::Owned(msg.velocity)).into_owned(),
        acceleration: geometry_msgs::msg::Accel::into_rmw_message(std::borrow::Cow::Owned(msg.acceleration)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        pose: geometry_msgs::msg::Pose::into_rmw_message(std::borrow::Cow::Borrowed(&msg.pose)).into_owned(),
        velocity: geometry_msgs::msg::Twist::into_rmw_message(std::borrow::Cow::Borrowed(&msg.velocity)).into_owned(),
        acceleration: geometry_msgs::msg::Accel::into_rmw_message(std::borrow::Cow::Borrowed(&msg.acceleration)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      pose: geometry_msgs::msg::Pose::from_rmw_message(msg.pose),
      velocity: geometry_msgs::msg::Twist::from_rmw_message(msg.velocity),
      acceleration: geometry_msgs::msg::Accel::from_rmw_message(msg.acceleration),
    }
  }
}


// Corresponds to moveit_msgs__msg__CartesianTrajectory
/// This message describes the trajectory of a tracked frame in task-space

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CartesianTrajectory {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,

    /// The name of the Cartesian frame being tracked with respect to the base frame provided in header.frame_id
    pub tracked_frame: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub points: Vec<super::msg::CartesianTrajectoryPoint>,

}



impl Default for CartesianTrajectory {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::CartesianTrajectory::default())
  }
}

impl rosidl_runtime_rs::Message for CartesianTrajectory {
  type RmwMsg = super::msg::rmw::CartesianTrajectory;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        tracked_frame: msg.tracked_frame.as_str().into(),
        points: msg.points
          .into_iter()
          .map(|elem| super::msg::CartesianTrajectoryPoint::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        tracked_frame: msg.tracked_frame.as_str().into(),
        points: msg.points
          .iter()
          .map(|elem| super::msg::CartesianTrajectoryPoint::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      tracked_frame: msg.tracked_frame.to_string(),
      points: msg.points
          .into_iter()
          .map(super::msg::CartesianTrajectoryPoint::from_rmw_message)
          .collect(),
    }
  }
}


// Corresponds to moveit_msgs__msg__CartesianTrajectoryPoint
/// The definition of a cartesian point in a trajectory. Defines the cartesian state of the point and it's time,
/// following the pattern of the JointTrajectory message

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CartesianTrajectoryPoint {

    // This member is not documented.
    #[allow(missing_docs)]
    pub point: super::msg::CartesianPoint,


    // This member is not documented.
    #[allow(missing_docs)]
    pub time_from_start: builtin_interfaces::msg::Duration,

}



impl Default for CartesianTrajectoryPoint {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::CartesianTrajectoryPoint::default())
  }
}

impl rosidl_runtime_rs::Message for CartesianTrajectoryPoint {
  type RmwMsg = super::msg::rmw::CartesianTrajectoryPoint;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        point: super::msg::CartesianPoint::into_rmw_message(std::borrow::Cow::Owned(msg.point)).into_owned(),
        time_from_start: builtin_interfaces::msg::Duration::into_rmw_message(std::borrow::Cow::Owned(msg.time_from_start)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        point: super::msg::CartesianPoint::into_rmw_message(std::borrow::Cow::Borrowed(&msg.point)).into_owned(),
        time_from_start: builtin_interfaces::msg::Duration::into_rmw_message(std::borrow::Cow::Borrowed(&msg.time_from_start)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      point: super::msg::CartesianPoint::from_rmw_message(msg.point),
      time_from_start: builtin_interfaces::msg::Duration::from_rmw_message(msg.time_from_start),
    }
  }
}


// Corresponds to moveit_msgs__msg__CollisionObject
/// a header, used for interpreting the poses

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CollisionObject {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,

    /// The object's pose relative to the header frame.
    /// The shapes and subframe poses are defined relative to this pose.
    pub pose: geometry_msgs::msg::Pose,

    /// The id of the object (name used in MoveIt)
    pub id: std::string::String,

    /// The object type in a database of known objects
    pub type_: object_recognition_msgs::msg::ObjectType,

    /// The collision geometries associated with the object.
    /// Their poses are with respect to the object's pose
    /// Solid geometric primitives
    pub primitives: Vec<shape_msgs::msg::SolidPrimitive>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub primitive_poses: Vec<geometry_msgs::msg::Pose>,

    /// Meshes
    pub meshes: Vec<shape_msgs::msg::Mesh>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub mesh_poses: Vec<geometry_msgs::msg::Pose>,

    /// Bounding planes (equation is specified, but the plane can be oriented using an additional pose)
    pub planes: Vec<shape_msgs::msg::Plane>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub plane_poses: Vec<geometry_msgs::msg::Pose>,

    /// Named subframes on the object. Use these to define points of interest on the object that you want
    /// to plan with (e.g. "tip", "spout", "handle"). The id of the object will be prepended to the subframe.
    /// If an object with the id "screwdriver" and a subframe "tip" is in the scene, you can use the frame
    /// "screwdriver/tip" for planning.
    /// The length of the subframe_names and subframe_poses has to be identical.
    pub subframe_names: Vec<std::string::String>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub subframe_poses: Vec<geometry_msgs::msg::Pose>,

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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::CollisionObject::default())
  }
}

impl rosidl_runtime_rs::Message for CollisionObject {
  type RmwMsg = super::msg::rmw::CollisionObject;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        pose: geometry_msgs::msg::Pose::into_rmw_message(std::borrow::Cow::Owned(msg.pose)).into_owned(),
        id: msg.id.as_str().into(),
        type_: object_recognition_msgs::msg::ObjectType::into_rmw_message(std::borrow::Cow::Owned(msg.type_)).into_owned(),
        primitives: msg.primitives
          .into_iter()
          .map(|elem| shape_msgs::msg::SolidPrimitive::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
        primitive_poses: msg.primitive_poses
          .into_iter()
          .map(|elem| geometry_msgs::msg::Pose::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
        meshes: msg.meshes
          .into_iter()
          .map(|elem| shape_msgs::msg::Mesh::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
        mesh_poses: msg.mesh_poses
          .into_iter()
          .map(|elem| geometry_msgs::msg::Pose::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
        planes: msg.planes
          .into_iter()
          .map(|elem| shape_msgs::msg::Plane::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
        plane_poses: msg.plane_poses
          .into_iter()
          .map(|elem| geometry_msgs::msg::Pose::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
        subframe_names: msg.subframe_names
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        subframe_poses: msg.subframe_poses
          .into_iter()
          .map(|elem| geometry_msgs::msg::Pose::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
        operation: msg.operation,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        pose: geometry_msgs::msg::Pose::into_rmw_message(std::borrow::Cow::Borrowed(&msg.pose)).into_owned(),
        id: msg.id.as_str().into(),
        type_: object_recognition_msgs::msg::ObjectType::into_rmw_message(std::borrow::Cow::Borrowed(&msg.type_)).into_owned(),
        primitives: msg.primitives
          .iter()
          .map(|elem| shape_msgs::msg::SolidPrimitive::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
        primitive_poses: msg.primitive_poses
          .iter()
          .map(|elem| geometry_msgs::msg::Pose::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
        meshes: msg.meshes
          .iter()
          .map(|elem| shape_msgs::msg::Mesh::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
        mesh_poses: msg.mesh_poses
          .iter()
          .map(|elem| geometry_msgs::msg::Pose::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
        planes: msg.planes
          .iter()
          .map(|elem| shape_msgs::msg::Plane::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
        plane_poses: msg.plane_poses
          .iter()
          .map(|elem| geometry_msgs::msg::Pose::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
        subframe_names: msg.subframe_names
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        subframe_poses: msg.subframe_poses
          .iter()
          .map(|elem| geometry_msgs::msg::Pose::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      operation: msg.operation,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      pose: geometry_msgs::msg::Pose::from_rmw_message(msg.pose),
      id: msg.id.to_string(),
      type_: object_recognition_msgs::msg::ObjectType::from_rmw_message(msg.type_),
      primitives: msg.primitives
          .into_iter()
          .map(shape_msgs::msg::SolidPrimitive::from_rmw_message)
          .collect(),
      primitive_poses: msg.primitive_poses
          .into_iter()
          .map(geometry_msgs::msg::Pose::from_rmw_message)
          .collect(),
      meshes: msg.meshes
          .into_iter()
          .map(shape_msgs::msg::Mesh::from_rmw_message)
          .collect(),
      mesh_poses: msg.mesh_poses
          .into_iter()
          .map(geometry_msgs::msg::Pose::from_rmw_message)
          .collect(),
      planes: msg.planes
          .into_iter()
          .map(shape_msgs::msg::Plane::from_rmw_message)
          .collect(),
      plane_poses: msg.plane_poses
          .into_iter()
          .map(geometry_msgs::msg::Pose::from_rmw_message)
          .collect(),
      subframe_names: msg.subframe_names
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
      subframe_poses: msg.subframe_poses
          .into_iter()
          .map(geometry_msgs::msg::Pose::from_rmw_message)
          .collect(),
      operation: msg.operation,
    }
  }
}


// Corresponds to moveit_msgs__msg__ConstraintEvalResult
/// This message contains result from constraint evaluation
/// result specifies the result of constraint evaluation
/// (true indicates state satisfies constraint, false indicates state violates constraint)
/// if false, distance specifies a measure of the distance of the state from the constraint
/// if true, distance is set to zero

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::ConstraintEvalResult::default())
  }
}

impl rosidl_runtime_rs::Message for ConstraintEvalResult {
  type RmwMsg = super::msg::rmw::ConstraintEvalResult;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        result: msg.result,
        distance: msg.distance,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      result: msg.result,
      distance: msg.distance,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      result: msg.result,
      distance: msg.distance,
    }
  }
}


// Corresponds to moveit_msgs__msg__Constraints
/// This message contains a list of motion planning constraints.
/// All constraints must be satisfied for a goal to be considered valid

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Constraints {

    // This member is not documented.
    #[allow(missing_docs)]
    pub name: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub joint_constraints: Vec<super::msg::JointConstraint>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub position_constraints: Vec<super::msg::PositionConstraint>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub orientation_constraints: Vec<super::msg::OrientationConstraint>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub visibility_constraints: Vec<super::msg::VisibilityConstraint>,

}



impl Default for Constraints {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::Constraints::default())
  }
}

impl rosidl_runtime_rs::Message for Constraints {
  type RmwMsg = super::msg::rmw::Constraints;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name.as_str().into(),
        joint_constraints: msg.joint_constraints
          .into_iter()
          .map(|elem| super::msg::JointConstraint::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
        position_constraints: msg.position_constraints
          .into_iter()
          .map(|elem| super::msg::PositionConstraint::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
        orientation_constraints: msg.orientation_constraints
          .into_iter()
          .map(|elem| super::msg::OrientationConstraint::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
        visibility_constraints: msg.visibility_constraints
          .into_iter()
          .map(|elem| super::msg::VisibilityConstraint::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name.as_str().into(),
        joint_constraints: msg.joint_constraints
          .iter()
          .map(|elem| super::msg::JointConstraint::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
        position_constraints: msg.position_constraints
          .iter()
          .map(|elem| super::msg::PositionConstraint::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
        orientation_constraints: msg.orientation_constraints
          .iter()
          .map(|elem| super::msg::OrientationConstraint::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
        visibility_constraints: msg.visibility_constraints
          .iter()
          .map(|elem| super::msg::VisibilityConstraint::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      name: msg.name.to_string(),
      joint_constraints: msg.joint_constraints
          .into_iter()
          .map(super::msg::JointConstraint::from_rmw_message)
          .collect(),
      position_constraints: msg.position_constraints
          .into_iter()
          .map(super::msg::PositionConstraint::from_rmw_message)
          .collect(),
      orientation_constraints: msg.orientation_constraints
          .into_iter()
          .map(super::msg::OrientationConstraint::from_rmw_message)
          .collect(),
      visibility_constraints: msg.visibility_constraints
          .into_iter()
          .map(super::msg::VisibilityConstraint::from_rmw_message)
          .collect(),
    }
  }
}


// Corresponds to moveit_msgs__msg__CostSource
/// The density of the cost source

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CostSource {

    // This member is not documented.
    #[allow(missing_docs)]
    pub cost_density: f64,

    /// The volume of the cost source is represented as an
    /// axis-aligned bounding box (AABB)
    /// The AABB is specified by two of its opposite corners
    pub aabb_min: geometry_msgs::msg::Vector3,


    // This member is not documented.
    #[allow(missing_docs)]
    pub aabb_max: geometry_msgs::msg::Vector3,

}



impl Default for CostSource {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::CostSource::default())
  }
}

impl rosidl_runtime_rs::Message for CostSource {
  type RmwMsg = super::msg::rmw::CostSource;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        cost_density: msg.cost_density,
        aabb_min: geometry_msgs::msg::Vector3::into_rmw_message(std::borrow::Cow::Owned(msg.aabb_min)).into_owned(),
        aabb_max: geometry_msgs::msg::Vector3::into_rmw_message(std::borrow::Cow::Owned(msg.aabb_max)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      cost_density: msg.cost_density,
        aabb_min: geometry_msgs::msg::Vector3::into_rmw_message(std::borrow::Cow::Borrowed(&msg.aabb_min)).into_owned(),
        aabb_max: geometry_msgs::msg::Vector3::into_rmw_message(std::borrow::Cow::Borrowed(&msg.aabb_max)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      cost_density: msg.cost_density,
      aabb_min: geometry_msgs::msg::Vector3::from_rmw_message(msg.aabb_min),
      aabb_max: geometry_msgs::msg::Vector3::from_rmw_message(msg.aabb_max),
    }
  }
}


// Corresponds to moveit_msgs__msg__ContactInformation
/// Standard ROS header contains information
/// about the frame in which this
/// contact is specified

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ContactInformation {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,

    /// Position of the contact point
    pub position: geometry_msgs::msg::Point,

    /// Normal corresponding to the contact point
    pub normal: geometry_msgs::msg::Vector3,

    /// Depth of contact point
    pub depth: f64,

    /// Name of the first body that is in contact
    /// This could be a link or a namespace that represents a body
    pub contact_body_1: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub body_type_1: u32,

    /// Name of the second body that is in contact
    /// This could be a link or a namespace that represents a body
    pub contact_body_2: std::string::String,


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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::ContactInformation::default())
  }
}

impl rosidl_runtime_rs::Message for ContactInformation {
  type RmwMsg = super::msg::rmw::ContactInformation;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        position: geometry_msgs::msg::Point::into_rmw_message(std::borrow::Cow::Owned(msg.position)).into_owned(),
        normal: geometry_msgs::msg::Vector3::into_rmw_message(std::borrow::Cow::Owned(msg.normal)).into_owned(),
        depth: msg.depth,
        contact_body_1: msg.contact_body_1.as_str().into(),
        body_type_1: msg.body_type_1,
        contact_body_2: msg.contact_body_2.as_str().into(),
        body_type_2: msg.body_type_2,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        position: geometry_msgs::msg::Point::into_rmw_message(std::borrow::Cow::Borrowed(&msg.position)).into_owned(),
        normal: geometry_msgs::msg::Vector3::into_rmw_message(std::borrow::Cow::Borrowed(&msg.normal)).into_owned(),
      depth: msg.depth,
        contact_body_1: msg.contact_body_1.as_str().into(),
      body_type_1: msg.body_type_1,
        contact_body_2: msg.contact_body_2.as_str().into(),
      body_type_2: msg.body_type_2,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      position: geometry_msgs::msg::Point::from_rmw_message(msg.position),
      normal: geometry_msgs::msg::Vector3::from_rmw_message(msg.normal),
      depth: msg.depth,
      contact_body_1: msg.contact_body_1.to_string(),
      body_type_1: msg.body_type_1,
      contact_body_2: msg.contact_body_2.to_string(),
      body_type_2: msg.body_type_2,
    }
  }
}


// Corresponds to moveit_msgs__msg__DisplayTrajectory
/// The model id for which this path has been generated

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DisplayTrajectory {

    // This member is not documented.
    #[allow(missing_docs)]
    pub model_id: std::string::String,

    /// The representation of the path contains position values for all the joints that are moving along the path;
    /// a sequence of trajectories may be specified
    pub trajectory: Vec<super::msg::RobotTrajectory>,

    /// The robot state is used to obtain positions for all/some of the joints of the robot.
    /// It is used by the path display node to determine the positions of the joints that are not specified
    /// in the joint trajectory message above. The positions in the joint trajectory message
    /// take precedence (for joints that the message covers).
    pub trajectory_start: super::msg::RobotState,

}



impl Default for DisplayTrajectory {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::DisplayTrajectory::default())
  }
}

impl rosidl_runtime_rs::Message for DisplayTrajectory {
  type RmwMsg = super::msg::rmw::DisplayTrajectory;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        model_id: msg.model_id.as_str().into(),
        trajectory: msg.trajectory
          .into_iter()
          .map(|elem| super::msg::RobotTrajectory::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
        trajectory_start: super::msg::RobotState::into_rmw_message(std::borrow::Cow::Owned(msg.trajectory_start)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        model_id: msg.model_id.as_str().into(),
        trajectory: msg.trajectory
          .iter()
          .map(|elem| super::msg::RobotTrajectory::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
        trajectory_start: super::msg::RobotState::into_rmw_message(std::borrow::Cow::Borrowed(&msg.trajectory_start)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      model_id: msg.model_id.to_string(),
      trajectory: msg.trajectory
          .into_iter()
          .map(super::msg::RobotTrajectory::from_rmw_message)
          .collect(),
      trajectory_start: super::msg::RobotState::from_rmw_message(msg.trajectory_start),
    }
  }
}


// Corresponds to moveit_msgs__msg__DisplayRobotState
/// The robot state to display

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DisplayRobotState {

    // This member is not documented.
    #[allow(missing_docs)]
    pub state: super::msg::RobotState,

    /// Optionally, various links can be highlighted
    pub highlight_links: Vec<super::msg::ObjectColor>,

    /// If true, suppress the display in visualizations (like rviz)
    pub hide: bool,

}



impl Default for DisplayRobotState {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::DisplayRobotState::default())
  }
}

impl rosidl_runtime_rs::Message for DisplayRobotState {
  type RmwMsg = super::msg::rmw::DisplayRobotState;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        state: super::msg::RobotState::into_rmw_message(std::borrow::Cow::Owned(msg.state)).into_owned(),
        highlight_links: msg.highlight_links
          .into_iter()
          .map(|elem| super::msg::ObjectColor::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
        hide: msg.hide,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        state: super::msg::RobotState::into_rmw_message(std::borrow::Cow::Borrowed(&msg.state)).into_owned(),
        highlight_links: msg.highlight_links
          .iter()
          .map(|elem| super::msg::ObjectColor::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      hide: msg.hide,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      state: super::msg::RobotState::from_rmw_message(msg.state),
      highlight_links: msg.highlight_links
          .into_iter()
          .map(super::msg::ObjectColor::from_rmw_message)
          .collect(),
      hide: msg.hide,
    }
  }
}


// Corresponds to moveit_msgs__msg__GenericTrajectory
/// A generic trajectory message that may either encode a joint- or cartesian-space trajectory, or both
/// Trajectories encoded in this message are considered semantically equivalent

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GenericTrajectory {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub joint_trajectory: Vec<trajectory_msgs::msg::JointTrajectory>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub cartesian_trajectory: Vec<super::msg::CartesianTrajectory>,

}



impl Default for GenericTrajectory {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::GenericTrajectory::default())
  }
}

impl rosidl_runtime_rs::Message for GenericTrajectory {
  type RmwMsg = super::msg::rmw::GenericTrajectory;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        joint_trajectory: msg.joint_trajectory
          .into_iter()
          .map(|elem| trajectory_msgs::msg::JointTrajectory::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
        cartesian_trajectory: msg.cartesian_trajectory
          .into_iter()
          .map(|elem| super::msg::CartesianTrajectory::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        joint_trajectory: msg.joint_trajectory
          .iter()
          .map(|elem| trajectory_msgs::msg::JointTrajectory::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
        cartesian_trajectory: msg.cartesian_trajectory
          .iter()
          .map(|elem| super::msg::CartesianTrajectory::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      joint_trajectory: msg.joint_trajectory
          .into_iter()
          .map(trajectory_msgs::msg::JointTrajectory::from_rmw_message)
          .collect(),
      cartesian_trajectory: msg.cartesian_trajectory
          .into_iter()
          .map(super::msg::CartesianTrajectory::from_rmw_message)
          .collect(),
    }
  }
}


// Corresponds to moveit_msgs__msg__Grasp
/// This message contains a description of a grasp that would be used
/// with a particular end-effector to grasp an object, including how to
/// approach it, grip it, etc.  This message does not contain any
/// information about a "grasp point" (a position ON the object).
/// Whatever generates this message should have already combined
/// information about grasp points with information about the geometry
/// of the end-effector to compute the grasp_pose in this message.

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Grasp {
    /// A name for this grasp
    pub id: std::string::String,

    /// The internal posture of the hand for the pre-grasp
    /// only positions are used
    pub pre_grasp_posture: trajectory_msgs::msg::JointTrajectory,

    /// The internal posture of the hand for the grasp
    /// positions and efforts are used
    pub grasp_posture: trajectory_msgs::msg::JointTrajectory,

    /// The position of the end-effector for the grasp.  This is the pose of
    /// the "parent_link" of the end-effector, not actually the pose of any
    /// link *in* the end-effector.  Typically this would be the pose of the
    /// most distal wrist link before the hand (end-effector) links began.
    pub grasp_pose: geometry_msgs::msg::PoseStamped,

    /// The estimated probability of success for this grasp, or some other
    /// measure of how "good" it is.
    pub grasp_quality: f64,

    /// The approach direction to take before picking an object
    pub pre_grasp_approach: super::msg::GripperTranslation,

    /// The retreat direction to take after a grasp has been completed (object is attached)
    pub post_grasp_retreat: super::msg::GripperTranslation,

    /// The retreat motion to perform when releasing the object; this information
    /// is not necessary for the grasp itself, but when releasing the object,
    /// the information will be necessary. The grasp used to perform a pickup
    /// is returned as part of the result, so this information is available for
    /// later use.
    pub post_place_retreat: super::msg::GripperTranslation,

    /// the maximum contact force to use while grasping (<=0 to disable)
    pub max_contact_force: f32,

    /// an optional list of obstacles that we have semantic information about
    /// and that can be touched/pushed/moved in the course of grasping
    pub allowed_touch_objects: Vec<std::string::String>,

}



impl Default for Grasp {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::Grasp::default())
  }
}

impl rosidl_runtime_rs::Message for Grasp {
  type RmwMsg = super::msg::rmw::Grasp;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        id: msg.id.as_str().into(),
        pre_grasp_posture: trajectory_msgs::msg::JointTrajectory::into_rmw_message(std::borrow::Cow::Owned(msg.pre_grasp_posture)).into_owned(),
        grasp_posture: trajectory_msgs::msg::JointTrajectory::into_rmw_message(std::borrow::Cow::Owned(msg.grasp_posture)).into_owned(),
        grasp_pose: geometry_msgs::msg::PoseStamped::into_rmw_message(std::borrow::Cow::Owned(msg.grasp_pose)).into_owned(),
        grasp_quality: msg.grasp_quality,
        pre_grasp_approach: super::msg::GripperTranslation::into_rmw_message(std::borrow::Cow::Owned(msg.pre_grasp_approach)).into_owned(),
        post_grasp_retreat: super::msg::GripperTranslation::into_rmw_message(std::borrow::Cow::Owned(msg.post_grasp_retreat)).into_owned(),
        post_place_retreat: super::msg::GripperTranslation::into_rmw_message(std::borrow::Cow::Owned(msg.post_place_retreat)).into_owned(),
        max_contact_force: msg.max_contact_force,
        allowed_touch_objects: msg.allowed_touch_objects
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        id: msg.id.as_str().into(),
        pre_grasp_posture: trajectory_msgs::msg::JointTrajectory::into_rmw_message(std::borrow::Cow::Borrowed(&msg.pre_grasp_posture)).into_owned(),
        grasp_posture: trajectory_msgs::msg::JointTrajectory::into_rmw_message(std::borrow::Cow::Borrowed(&msg.grasp_posture)).into_owned(),
        grasp_pose: geometry_msgs::msg::PoseStamped::into_rmw_message(std::borrow::Cow::Borrowed(&msg.grasp_pose)).into_owned(),
      grasp_quality: msg.grasp_quality,
        pre_grasp_approach: super::msg::GripperTranslation::into_rmw_message(std::borrow::Cow::Borrowed(&msg.pre_grasp_approach)).into_owned(),
        post_grasp_retreat: super::msg::GripperTranslation::into_rmw_message(std::borrow::Cow::Borrowed(&msg.post_grasp_retreat)).into_owned(),
        post_place_retreat: super::msg::GripperTranslation::into_rmw_message(std::borrow::Cow::Borrowed(&msg.post_place_retreat)).into_owned(),
      max_contact_force: msg.max_contact_force,
        allowed_touch_objects: msg.allowed_touch_objects
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      id: msg.id.to_string(),
      pre_grasp_posture: trajectory_msgs::msg::JointTrajectory::from_rmw_message(msg.pre_grasp_posture),
      grasp_posture: trajectory_msgs::msg::JointTrajectory::from_rmw_message(msg.grasp_posture),
      grasp_pose: geometry_msgs::msg::PoseStamped::from_rmw_message(msg.grasp_pose),
      grasp_quality: msg.grasp_quality,
      pre_grasp_approach: super::msg::GripperTranslation::from_rmw_message(msg.pre_grasp_approach),
      post_grasp_retreat: super::msg::GripperTranslation::from_rmw_message(msg.post_grasp_retreat),
      post_place_retreat: super::msg::GripperTranslation::from_rmw_message(msg.post_place_retreat),
      max_contact_force: msg.max_contact_force,
      allowed_touch_objects: msg.allowed_touch_objects
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
    }
  }
}


// Corresponds to moveit_msgs__msg__GripperTranslation
/// defines a translation for the gripper, used in pickup or place tasks
/// for example for lifting an object off a table or approaching the table for placing

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GripperTranslation {
    /// the direction of the translation
    pub direction: geometry_msgs::msg::Vector3Stamped,

    /// the desired translation distance
    pub desired_distance: f32,

    /// the min distance that must be considered feasible before the
    /// grasp is even attempted
    pub min_distance: f32,

}



impl Default for GripperTranslation {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::GripperTranslation::default())
  }
}

impl rosidl_runtime_rs::Message for GripperTranslation {
  type RmwMsg = super::msg::rmw::GripperTranslation;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        direction: geometry_msgs::msg::Vector3Stamped::into_rmw_message(std::borrow::Cow::Owned(msg.direction)).into_owned(),
        desired_distance: msg.desired_distance,
        min_distance: msg.min_distance,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        direction: geometry_msgs::msg::Vector3Stamped::into_rmw_message(std::borrow::Cow::Borrowed(&msg.direction)).into_owned(),
      desired_distance: msg.desired_distance,
      min_distance: msg.min_distance,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      direction: geometry_msgs::msg::Vector3Stamped::from_rmw_message(msg.direction),
      desired_distance: msg.desired_distance,
      min_distance: msg.min_distance,
    }
  }
}


// Corresponds to moveit_msgs__msg__JointConstraint
/// Constrain the position of a joint to be within a certain bound

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct JointConstraint {

    // This member is not documented.
    #[allow(missing_docs)]
    pub joint_name: std::string::String,

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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::JointConstraint::default())
  }
}

impl rosidl_runtime_rs::Message for JointConstraint {
  type RmwMsg = super::msg::rmw::JointConstraint;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        joint_name: msg.joint_name.as_str().into(),
        position: msg.position,
        tolerance_above: msg.tolerance_above,
        tolerance_below: msg.tolerance_below,
        weight: msg.weight,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        joint_name: msg.joint_name.as_str().into(),
      position: msg.position,
      tolerance_above: msg.tolerance_above,
      tolerance_below: msg.tolerance_below,
      weight: msg.weight,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      joint_name: msg.joint_name.to_string(),
      position: msg.position,
      tolerance_above: msg.tolerance_above,
      tolerance_below: msg.tolerance_below,
      weight: msg.weight,
    }
  }
}


// Corresponds to moveit_msgs__msg__JointLimits
/// This message contains information about limits of a particular joint (or control dimension)

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct JointLimits {

    // This member is not documented.
    #[allow(missing_docs)]
    pub joint_name: std::string::String,

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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::JointLimits::default())
  }
}

impl rosidl_runtime_rs::Message for JointLimits {
  type RmwMsg = super::msg::rmw::JointLimits;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        joint_name: msg.joint_name.as_str().into(),
        has_position_limits: msg.has_position_limits,
        min_position: msg.min_position,
        max_position: msg.max_position,
        has_velocity_limits: msg.has_velocity_limits,
        max_velocity: msg.max_velocity,
        has_acceleration_limits: msg.has_acceleration_limits,
        max_acceleration: msg.max_acceleration,
        has_jerk_limits: msg.has_jerk_limits,
        max_jerk: msg.max_jerk,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        joint_name: msg.joint_name.as_str().into(),
      has_position_limits: msg.has_position_limits,
      min_position: msg.min_position,
      max_position: msg.max_position,
      has_velocity_limits: msg.has_velocity_limits,
      max_velocity: msg.max_velocity,
      has_acceleration_limits: msg.has_acceleration_limits,
      max_acceleration: msg.max_acceleration,
      has_jerk_limits: msg.has_jerk_limits,
      max_jerk: msg.max_jerk,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      joint_name: msg.joint_name.to_string(),
      has_position_limits: msg.has_position_limits,
      min_position: msg.min_position,
      max_position: msg.max_position,
      has_velocity_limits: msg.has_velocity_limits,
      max_velocity: msg.max_velocity,
      has_acceleration_limits: msg.has_acceleration_limits,
      max_acceleration: msg.max_acceleration,
      has_jerk_limits: msg.has_jerk_limits,
      max_jerk: msg.max_jerk,
    }
  }
}


// Corresponds to moveit_msgs__msg__LinkPadding
/// name for the link

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LinkPadding {

    // This member is not documented.
    #[allow(missing_docs)]
    pub link_name: std::string::String,

    /// padding to apply to the link
    pub padding: f64,

}



impl Default for LinkPadding {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::LinkPadding::default())
  }
}

impl rosidl_runtime_rs::Message for LinkPadding {
  type RmwMsg = super::msg::rmw::LinkPadding;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        link_name: msg.link_name.as_str().into(),
        padding: msg.padding,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        link_name: msg.link_name.as_str().into(),
      padding: msg.padding,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      link_name: msg.link_name.to_string(),
      padding: msg.padding,
    }
  }
}


// Corresponds to moveit_msgs__msg__LinkScale
/// name for the link

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LinkScale {

    // This member is not documented.
    #[allow(missing_docs)]
    pub link_name: std::string::String,

    /// scaling to apply to the link
    pub scale: f64,

}



impl Default for LinkScale {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::LinkScale::default())
  }
}

impl rosidl_runtime_rs::Message for LinkScale {
  type RmwMsg = super::msg::rmw::LinkScale;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        link_name: msg.link_name.as_str().into(),
        scale: msg.scale,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        link_name: msg.link_name.as_str().into(),
      scale: msg.scale,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      link_name: msg.link_name.to_string(),
      scale: msg.scale,
    }
  }
}


// Corresponds to moveit_msgs__msg__MotionPlanRequest
/// This service contains the definition for a request to the motion
/// planner and the output it provides

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MotionPlanRequest {
    /// Parameters for the workspace that the planner should work inside
    pub workspace_parameters: super::msg::WorkspaceParameters,

    /// Starting state updates. If certain joints should be considered
    /// at positions other than the current ones, these positions should
    /// be set here
    pub start_state: super::msg::RobotState,

    /// The possible goal states for the model to plan for. Each element of
    /// the array defines a goal region. The goal is achieved
    /// if the constraints for a particular region are satisfied
    pub goal_constraints: Vec<super::msg::Constraints>,

    /// No state at any point along the path in the produced motion plan will violate these constraints (this applies to all points, not just waypoints)
    pub path_constraints: super::msg::Constraints,

    /// The constraints the resulting trajectory must satisfy
    pub trajectory_constraints: super::msg::TrajectoryConstraints,

    /// A set of trajectories that may be used as reference or initial trajectories for (typically optimization-based) planners
    /// These trajectories do not override start_state or goal_constraints
    pub reference_trajectories: Vec<super::msg::GenericTrajectory>,

    /// The name of the planning pipeline to use. If no name is specified,
    /// the configured planning pipeline will be used
    pub pipeline_id: std::string::String,

    /// The name of the planning algorithm to use. If no name is specified,
    /// the default planner of the planning pipeline will be used
    pub planner_id: std::string::String,

    /// The name of the group of joints on which this planner is operating
    pub group_name: std::string::String,

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
    pub cartesian_speed_limited_link: std::string::String,

    /// m/s
    pub max_cartesian_speed: f64,

    /// smoothness of trajectory for FREE command of Pilz planner
    /// Value should be between 0.01 and 1.0
    pub smoothness_level: f64,

}



impl Default for MotionPlanRequest {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::MotionPlanRequest::default())
  }
}

impl rosidl_runtime_rs::Message for MotionPlanRequest {
  type RmwMsg = super::msg::rmw::MotionPlanRequest;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        workspace_parameters: super::msg::WorkspaceParameters::into_rmw_message(std::borrow::Cow::Owned(msg.workspace_parameters)).into_owned(),
        start_state: super::msg::RobotState::into_rmw_message(std::borrow::Cow::Owned(msg.start_state)).into_owned(),
        goal_constraints: msg.goal_constraints
          .into_iter()
          .map(|elem| super::msg::Constraints::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
        path_constraints: super::msg::Constraints::into_rmw_message(std::borrow::Cow::Owned(msg.path_constraints)).into_owned(),
        trajectory_constraints: super::msg::TrajectoryConstraints::into_rmw_message(std::borrow::Cow::Owned(msg.trajectory_constraints)).into_owned(),
        reference_trajectories: msg.reference_trajectories
          .into_iter()
          .map(|elem| super::msg::GenericTrajectory::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
        pipeline_id: msg.pipeline_id.as_str().into(),
        planner_id: msg.planner_id.as_str().into(),
        group_name: msg.group_name.as_str().into(),
        num_planning_attempts: msg.num_planning_attempts,
        allowed_planning_time: msg.allowed_planning_time,
        max_velocity_scaling_factor: msg.max_velocity_scaling_factor,
        max_acceleration_scaling_factor: msg.max_acceleration_scaling_factor,
        cartesian_speed_limited_link: msg.cartesian_speed_limited_link.as_str().into(),
        max_cartesian_speed: msg.max_cartesian_speed,
        smoothness_level: msg.smoothness_level,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        workspace_parameters: super::msg::WorkspaceParameters::into_rmw_message(std::borrow::Cow::Borrowed(&msg.workspace_parameters)).into_owned(),
        start_state: super::msg::RobotState::into_rmw_message(std::borrow::Cow::Borrowed(&msg.start_state)).into_owned(),
        goal_constraints: msg.goal_constraints
          .iter()
          .map(|elem| super::msg::Constraints::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
        path_constraints: super::msg::Constraints::into_rmw_message(std::borrow::Cow::Borrowed(&msg.path_constraints)).into_owned(),
        trajectory_constraints: super::msg::TrajectoryConstraints::into_rmw_message(std::borrow::Cow::Borrowed(&msg.trajectory_constraints)).into_owned(),
        reference_trajectories: msg.reference_trajectories
          .iter()
          .map(|elem| super::msg::GenericTrajectory::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
        pipeline_id: msg.pipeline_id.as_str().into(),
        planner_id: msg.planner_id.as_str().into(),
        group_name: msg.group_name.as_str().into(),
      num_planning_attempts: msg.num_planning_attempts,
      allowed_planning_time: msg.allowed_planning_time,
      max_velocity_scaling_factor: msg.max_velocity_scaling_factor,
      max_acceleration_scaling_factor: msg.max_acceleration_scaling_factor,
        cartesian_speed_limited_link: msg.cartesian_speed_limited_link.as_str().into(),
      max_cartesian_speed: msg.max_cartesian_speed,
      smoothness_level: msg.smoothness_level,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      workspace_parameters: super::msg::WorkspaceParameters::from_rmw_message(msg.workspace_parameters),
      start_state: super::msg::RobotState::from_rmw_message(msg.start_state),
      goal_constraints: msg.goal_constraints
          .into_iter()
          .map(super::msg::Constraints::from_rmw_message)
          .collect(),
      path_constraints: super::msg::Constraints::from_rmw_message(msg.path_constraints),
      trajectory_constraints: super::msg::TrajectoryConstraints::from_rmw_message(msg.trajectory_constraints),
      reference_trajectories: msg.reference_trajectories
          .into_iter()
          .map(super::msg::GenericTrajectory::from_rmw_message)
          .collect(),
      pipeline_id: msg.pipeline_id.to_string(),
      planner_id: msg.planner_id.to_string(),
      group_name: msg.group_name.to_string(),
      num_planning_attempts: msg.num_planning_attempts,
      allowed_planning_time: msg.allowed_planning_time,
      max_velocity_scaling_factor: msg.max_velocity_scaling_factor,
      max_acceleration_scaling_factor: msg.max_acceleration_scaling_factor,
      cartesian_speed_limited_link: msg.cartesian_speed_limited_link.to_string(),
      max_cartesian_speed: msg.max_cartesian_speed,
      smoothness_level: msg.smoothness_level,
    }
  }
}


// Corresponds to moveit_msgs__msg__MotionPlanResponse
/// The representation of a solution to a planning problem

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MotionPlanResponse {
    /// The corresponding robot state
    pub trajectory_start: super::msg::RobotState,

    /// The group used for planning (usually the same as in the request)
    pub group_name: std::string::String,

    /// A solution trajectory, if found
    pub trajectory: super::msg::RobotTrajectory,

    /// Planning time (seconds)
    pub planning_time: f64,

    /// Error code - encodes the overall reason for failure
    pub error_code: super::msg::MoveItErrorCodes,

}



impl Default for MotionPlanResponse {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::MotionPlanResponse::default())
  }
}

impl rosidl_runtime_rs::Message for MotionPlanResponse {
  type RmwMsg = super::msg::rmw::MotionPlanResponse;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        trajectory_start: super::msg::RobotState::into_rmw_message(std::borrow::Cow::Owned(msg.trajectory_start)).into_owned(),
        group_name: msg.group_name.as_str().into(),
        trajectory: super::msg::RobotTrajectory::into_rmw_message(std::borrow::Cow::Owned(msg.trajectory)).into_owned(),
        planning_time: msg.planning_time,
        error_code: super::msg::MoveItErrorCodes::into_rmw_message(std::borrow::Cow::Owned(msg.error_code)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        trajectory_start: super::msg::RobotState::into_rmw_message(std::borrow::Cow::Borrowed(&msg.trajectory_start)).into_owned(),
        group_name: msg.group_name.as_str().into(),
        trajectory: super::msg::RobotTrajectory::into_rmw_message(std::borrow::Cow::Borrowed(&msg.trajectory)).into_owned(),
      planning_time: msg.planning_time,
        error_code: super::msg::MoveItErrorCodes::into_rmw_message(std::borrow::Cow::Borrowed(&msg.error_code)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      trajectory_start: super::msg::RobotState::from_rmw_message(msg.trajectory_start),
      group_name: msg.group_name.to_string(),
      trajectory: super::msg::RobotTrajectory::from_rmw_message(msg.trajectory),
      planning_time: msg.planning_time,
      error_code: super::msg::MoveItErrorCodes::from_rmw_message(msg.error_code),
    }
  }
}


// Corresponds to moveit_msgs__msg__MotionPlanDetailedResponse
/// The representation of a solution to a planning problem, including intermediate data

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MotionPlanDetailedResponse {
    /// The starting state considered for the robot solution path
    pub trajectory_start: super::msg::RobotState,

    /// The group used for planning (usually the same as in the request)
    pub group_name: std::string::String,

    /// Multiple solution paths are reported, each reflecting intermediate steps in the trajectory processing
    /// The list of reported trajectories
    pub trajectory: Vec<super::msg::RobotTrajectory>,

    /// Description of the reported trajectories (name of processing step)
    pub description: Vec<std::string::String>,

    /// The amount of time spent computing a particular step in motion plan computation
    pub processing_time: Vec<f64>,

    /// Status at the end of this plan
    pub error_code: super::msg::MoveItErrorCodes,

}



impl Default for MotionPlanDetailedResponse {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::MotionPlanDetailedResponse::default())
  }
}

impl rosidl_runtime_rs::Message for MotionPlanDetailedResponse {
  type RmwMsg = super::msg::rmw::MotionPlanDetailedResponse;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        trajectory_start: super::msg::RobotState::into_rmw_message(std::borrow::Cow::Owned(msg.trajectory_start)).into_owned(),
        group_name: msg.group_name.as_str().into(),
        trajectory: msg.trajectory
          .into_iter()
          .map(|elem| super::msg::RobotTrajectory::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
        description: msg.description
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        processing_time: msg.processing_time.as_slice().into(),
        error_code: super::msg::MoveItErrorCodes::into_rmw_message(std::borrow::Cow::Owned(msg.error_code)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        trajectory_start: super::msg::RobotState::into_rmw_message(std::borrow::Cow::Borrowed(&msg.trajectory_start)).into_owned(),
        group_name: msg.group_name.as_str().into(),
        trajectory: msg.trajectory
          .iter()
          .map(|elem| super::msg::RobotTrajectory::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
        description: msg.description
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        processing_time: msg.processing_time.as_slice().into(),
        error_code: super::msg::MoveItErrorCodes::into_rmw_message(std::borrow::Cow::Borrowed(&msg.error_code)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      trajectory_start: super::msg::RobotState::from_rmw_message(msg.trajectory_start),
      group_name: msg.group_name.to_string(),
      trajectory: msg.trajectory
          .into_iter()
          .map(super::msg::RobotTrajectory::from_rmw_message)
          .collect(),
      description: msg.description
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
      processing_time: msg.processing_time.into(),
      error_code: super::msg::MoveItErrorCodes::from_rmw_message(msg.error_code),
    }
  }
}


// Corresponds to moveit_msgs__msg__MotionSequenceItem
/// The plan request for this item.
/// It is the planning request for this segment of the sequence, as if it were a solitary motion.

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MotionSequenceItem {

    // This member is not documented.
    #[allow(missing_docs)]
    pub req: super::msg::MotionPlanRequest,

    /// To blend between sequence items, the motion may be smoothed using a circular motion.
    /// The blend radius of the circle between this and the next command, where 0 means no blending.
    pub blend_radius: f64,

}



impl Default for MotionSequenceItem {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::MotionSequenceItem::default())
  }
}

impl rosidl_runtime_rs::Message for MotionSequenceItem {
  type RmwMsg = super::msg::rmw::MotionSequenceItem;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        req: super::msg::MotionPlanRequest::into_rmw_message(std::borrow::Cow::Owned(msg.req)).into_owned(),
        blend_radius: msg.blend_radius,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        req: super::msg::MotionPlanRequest::into_rmw_message(std::borrow::Cow::Borrowed(&msg.req)).into_owned(),
      blend_radius: msg.blend_radius,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      req: super::msg::MotionPlanRequest::from_rmw_message(msg.req),
      blend_radius: msg.blend_radius,
    }
  }
}


// Corresponds to moveit_msgs__msg__MotionSequenceRequest
/// List of motion planning request with a blend_radius for each.
/// In the response of the planner all of these will be executable as one sequence.

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MotionSequenceRequest {

    // This member is not documented.
    #[allow(missing_docs)]
    pub items: Vec<super::msg::MotionSequenceItem>,

}



impl Default for MotionSequenceRequest {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::MotionSequenceRequest::default())
  }
}

impl rosidl_runtime_rs::Message for MotionSequenceRequest {
  type RmwMsg = super::msg::rmw::MotionSequenceRequest;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        items: msg.items
          .into_iter()
          .map(|elem| super::msg::MotionSequenceItem::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        items: msg.items
          .iter()
          .map(|elem| super::msg::MotionSequenceItem::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      items: msg.items
          .into_iter()
          .map(super::msg::MotionSequenceItem::from_rmw_message)
          .collect(),
    }
  }
}


// Corresponds to moveit_msgs__msg__MotionSequenceResponse
/// An error code reflecting what went wrong

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MotionSequenceResponse {

    // This member is not documented.
    #[allow(missing_docs)]
    pub error_code: super::msg::MoveItErrorCodes,

    /// The full starting state of the robot at the start of the sequence
    pub sequence_start: super::msg::RobotState,

    /// The trajectories that the planner produced for execution
    pub planned_trajectories: Vec<super::msg::RobotTrajectory>,

    /// The amount of time it took to complete the motion plan
    pub planning_time: f64,

}



impl Default for MotionSequenceResponse {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::MotionSequenceResponse::default())
  }
}

impl rosidl_runtime_rs::Message for MotionSequenceResponse {
  type RmwMsg = super::msg::rmw::MotionSequenceResponse;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        error_code: super::msg::MoveItErrorCodes::into_rmw_message(std::borrow::Cow::Owned(msg.error_code)).into_owned(),
        sequence_start: super::msg::RobotState::into_rmw_message(std::borrow::Cow::Owned(msg.sequence_start)).into_owned(),
        planned_trajectories: msg.planned_trajectories
          .into_iter()
          .map(|elem| super::msg::RobotTrajectory::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
        planning_time: msg.planning_time,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        error_code: super::msg::MoveItErrorCodes::into_rmw_message(std::borrow::Cow::Borrowed(&msg.error_code)).into_owned(),
        sequence_start: super::msg::RobotState::into_rmw_message(std::borrow::Cow::Borrowed(&msg.sequence_start)).into_owned(),
        planned_trajectories: msg.planned_trajectories
          .iter()
          .map(|elem| super::msg::RobotTrajectory::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      planning_time: msg.planning_time,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      error_code: super::msg::MoveItErrorCodes::from_rmw_message(msg.error_code),
      sequence_start: super::msg::RobotState::from_rmw_message(msg.sequence_start),
      planned_trajectories: msg.planned_trajectories
          .into_iter()
          .map(super::msg::RobotTrajectory::from_rmw_message)
          .collect(),
      planning_time: msg.planning_time,
    }
  }
}


// Corresponds to moveit_msgs__msg__MoveItErrorCodes

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MoveItErrorCodes {

    // This member is not documented.
    #[allow(missing_docs)]
    pub val: i32,

    /// A message to provide additional information.
    pub message: std::string::String,

    /// Name of the component that created the status.
    /// This is helpful to locate error source.
    pub source: std::string::String,

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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::MoveItErrorCodes::default())
  }
}

impl rosidl_runtime_rs::Message for MoveItErrorCodes {
  type RmwMsg = super::msg::rmw::MoveItErrorCodes;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        val: msg.val,
        message: msg.message.as_str().into(),
        source: msg.source.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      val: msg.val,
        message: msg.message.as_str().into(),
        source: msg.source.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      val: msg.val,
      message: msg.message.to_string(),
      source: msg.source.to_string(),
    }
  }
}


// Corresponds to moveit_msgs__msg__TrajectoryConstraints
/// The array of constraints to consider along the trajectory

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TrajectoryConstraints {

    // This member is not documented.
    #[allow(missing_docs)]
    pub constraints: Vec<super::msg::Constraints>,

}



impl Default for TrajectoryConstraints {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::TrajectoryConstraints::default())
  }
}

impl rosidl_runtime_rs::Message for TrajectoryConstraints {
  type RmwMsg = super::msg::rmw::TrajectoryConstraints;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        constraints: msg.constraints
          .into_iter()
          .map(|elem| super::msg::Constraints::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        constraints: msg.constraints
          .iter()
          .map(|elem| super::msg::Constraints::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      constraints: msg.constraints
          .into_iter()
          .map(super::msg::Constraints::from_rmw_message)
          .collect(),
    }
  }
}


// Corresponds to moveit_msgs__msg__ObjectColor
/// The object id for which we specify color

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ObjectColor {

    // This member is not documented.
    #[allow(missing_docs)]
    pub id: std::string::String,

    /// The value of the color
    pub color: std_msgs::msg::ColorRGBA,

}



impl Default for ObjectColor {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::ObjectColor::default())
  }
}

impl rosidl_runtime_rs::Message for ObjectColor {
  type RmwMsg = super::msg::rmw::ObjectColor;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        id: msg.id.as_str().into(),
        color: std_msgs::msg::ColorRGBA::into_rmw_message(std::borrow::Cow::Owned(msg.color)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        id: msg.id.as_str().into(),
        color: std_msgs::msg::ColorRGBA::into_rmw_message(std::borrow::Cow::Borrowed(&msg.color)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      id: msg.id.to_string(),
      color: std_msgs::msg::ColorRGBA::from_rmw_message(msg.color),
    }
  }
}


// Corresponds to moveit_msgs__msg__OrientationConstraint
/// This message contains the definition of an orientation constraint.

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct OrientationConstraint {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,

    /// The desired orientation of the robot link specified as a quaternion
    pub orientation: geometry_msgs::msg::Quaternion,

    /// The robot link this constraint refers to
    pub link_name: std::string::String,

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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::OrientationConstraint::default())
  }
}

impl rosidl_runtime_rs::Message for OrientationConstraint {
  type RmwMsg = super::msg::rmw::OrientationConstraint;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        orientation: geometry_msgs::msg::Quaternion::into_rmw_message(std::borrow::Cow::Owned(msg.orientation)).into_owned(),
        link_name: msg.link_name.as_str().into(),
        absolute_x_axis_tolerance: msg.absolute_x_axis_tolerance,
        absolute_y_axis_tolerance: msg.absolute_y_axis_tolerance,
        absolute_z_axis_tolerance: msg.absolute_z_axis_tolerance,
        parameterization: msg.parameterization,
        weight: msg.weight,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        orientation: geometry_msgs::msg::Quaternion::into_rmw_message(std::borrow::Cow::Borrowed(&msg.orientation)).into_owned(),
        link_name: msg.link_name.as_str().into(),
      absolute_x_axis_tolerance: msg.absolute_x_axis_tolerance,
      absolute_y_axis_tolerance: msg.absolute_y_axis_tolerance,
      absolute_z_axis_tolerance: msg.absolute_z_axis_tolerance,
      parameterization: msg.parameterization,
      weight: msg.weight,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      orientation: geometry_msgs::msg::Quaternion::from_rmw_message(msg.orientation),
      link_name: msg.link_name.to_string(),
      absolute_x_axis_tolerance: msg.absolute_x_axis_tolerance,
      absolute_y_axis_tolerance: msg.absolute_y_axis_tolerance,
      absolute_z_axis_tolerance: msg.absolute_z_axis_tolerance,
      parameterization: msg.parameterization,
      weight: msg.weight,
    }
  }
}


// Corresponds to moveit_msgs__msg__OrientedBoundingBox
/// the pose of the box

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct OrientedBoundingBox {

    // This member is not documented.
    #[allow(missing_docs)]
    pub pose: geometry_msgs::msg::Pose,

    /// the extents of the box, assuming the center is at the origin
    pub extents: geometry_msgs::msg::Point32,

}



impl Default for OrientedBoundingBox {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::OrientedBoundingBox::default())
  }
}

impl rosidl_runtime_rs::Message for OrientedBoundingBox {
  type RmwMsg = super::msg::rmw::OrientedBoundingBox;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        pose: geometry_msgs::msg::Pose::into_rmw_message(std::borrow::Cow::Owned(msg.pose)).into_owned(),
        extents: geometry_msgs::msg::Point32::into_rmw_message(std::borrow::Cow::Owned(msg.extents)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        pose: geometry_msgs::msg::Pose::into_rmw_message(std::borrow::Cow::Borrowed(&msg.pose)).into_owned(),
        extents: geometry_msgs::msg::Point32::into_rmw_message(std::borrow::Cow::Borrowed(&msg.extents)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      pose: geometry_msgs::msg::Pose::from_rmw_message(msg.pose),
      extents: geometry_msgs::msg::Point32::from_rmw_message(msg.extents),
    }
  }
}


// Corresponds to moveit_msgs__msg__PlaceLocation
/// A name for this grasp

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PlaceLocation {

    // This member is not documented.
    #[allow(missing_docs)]
    pub id: std::string::String,

    /// The internal posture of the hand for the grasp
    /// positions and efforts are used
    pub post_place_posture: trajectory_msgs::msg::JointTrajectory,

    /// The position of the end-effector for the grasp relative to a reference frame
    /// (that is always specified elsewhere, not in this message)
    pub place_pose: geometry_msgs::msg::PoseStamped,

    /// The estimated probability of success for this place, or some other
    /// measure of how "good" it is.
    pub quality: f64,

    /// The approach motion
    pub pre_place_approach: super::msg::GripperTranslation,

    /// The retreat motion
    pub post_place_retreat: super::msg::GripperTranslation,

    /// an optional list of obstacles that we have semantic information about
    /// and that can be touched/pushed/moved in the course of grasping
    pub allowed_touch_objects: Vec<std::string::String>,

}



impl Default for PlaceLocation {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::PlaceLocation::default())
  }
}

impl rosidl_runtime_rs::Message for PlaceLocation {
  type RmwMsg = super::msg::rmw::PlaceLocation;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        id: msg.id.as_str().into(),
        post_place_posture: trajectory_msgs::msg::JointTrajectory::into_rmw_message(std::borrow::Cow::Owned(msg.post_place_posture)).into_owned(),
        place_pose: geometry_msgs::msg::PoseStamped::into_rmw_message(std::borrow::Cow::Owned(msg.place_pose)).into_owned(),
        quality: msg.quality,
        pre_place_approach: super::msg::GripperTranslation::into_rmw_message(std::borrow::Cow::Owned(msg.pre_place_approach)).into_owned(),
        post_place_retreat: super::msg::GripperTranslation::into_rmw_message(std::borrow::Cow::Owned(msg.post_place_retreat)).into_owned(),
        allowed_touch_objects: msg.allowed_touch_objects
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        id: msg.id.as_str().into(),
        post_place_posture: trajectory_msgs::msg::JointTrajectory::into_rmw_message(std::borrow::Cow::Borrowed(&msg.post_place_posture)).into_owned(),
        place_pose: geometry_msgs::msg::PoseStamped::into_rmw_message(std::borrow::Cow::Borrowed(&msg.place_pose)).into_owned(),
      quality: msg.quality,
        pre_place_approach: super::msg::GripperTranslation::into_rmw_message(std::borrow::Cow::Borrowed(&msg.pre_place_approach)).into_owned(),
        post_place_retreat: super::msg::GripperTranslation::into_rmw_message(std::borrow::Cow::Borrowed(&msg.post_place_retreat)).into_owned(),
        allowed_touch_objects: msg.allowed_touch_objects
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      id: msg.id.to_string(),
      post_place_posture: trajectory_msgs::msg::JointTrajectory::from_rmw_message(msg.post_place_posture),
      place_pose: geometry_msgs::msg::PoseStamped::from_rmw_message(msg.place_pose),
      quality: msg.quality,
      pre_place_approach: super::msg::GripperTranslation::from_rmw_message(msg.pre_place_approach),
      post_place_retreat: super::msg::GripperTranslation::from_rmw_message(msg.post_place_retreat),
      allowed_touch_objects: msg.allowed_touch_objects
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
    }
  }
}


// Corresponds to moveit_msgs__msg__PlannerInterfaceDescription
/// The name of the planner interface

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PlannerInterfaceDescription {

    // This member is not documented.
    #[allow(missing_docs)]
    pub name: std::string::String,

    /// The name of the planning pipeline
    pub pipeline_id: std::string::String,

    /// The names of the planner ids within the interface
    pub planner_ids: Vec<std::string::String>,

}



impl Default for PlannerInterfaceDescription {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::PlannerInterfaceDescription::default())
  }
}

impl rosidl_runtime_rs::Message for PlannerInterfaceDescription {
  type RmwMsg = super::msg::rmw::PlannerInterfaceDescription;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name.as_str().into(),
        pipeline_id: msg.pipeline_id.as_str().into(),
        planner_ids: msg.planner_ids
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name.as_str().into(),
        pipeline_id: msg.pipeline_id.as_str().into(),
        planner_ids: msg.planner_ids
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      name: msg.name.to_string(),
      pipeline_id: msg.pipeline_id.to_string(),
      planner_ids: msg.planner_ids
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
    }
  }
}


// Corresponds to moveit_msgs__msg__PlannerParams
/// parameter names (same size as values)

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PlannerParams {

    // This member is not documented.
    #[allow(missing_docs)]
    pub keys: Vec<std::string::String>,

    /// parameter values (same size as keys)
    pub values: Vec<std::string::String>,

    /// parameter description (can be empty)
    pub descriptions: Vec<std::string::String>,

}



impl Default for PlannerParams {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::PlannerParams::default())
  }
}

impl rosidl_runtime_rs::Message for PlannerParams {
  type RmwMsg = super::msg::rmw::PlannerParams;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        keys: msg.keys
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        values: msg.values
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        descriptions: msg.descriptions
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        keys: msg.keys
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        values: msg.values
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        descriptions: msg.descriptions
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      keys: msg.keys
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
      values: msg.values
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
      descriptions: msg.descriptions
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
    }
  }
}


// Corresponds to moveit_msgs__msg__PlanningScene
/// name of planning scene

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PlanningScene {

    // This member is not documented.
    #[allow(missing_docs)]
    pub name: std::string::String,

    /// full robot state
    pub robot_state: super::msg::RobotState,

    /// The name of the robot model this scene is for
    pub robot_model_name: std::string::String,

    /// additional frames for duplicating tf (with respect to the planning frame)
    pub fixed_frame_transforms: Vec<geometry_msgs::msg::TransformStamped>,

    /// full allowed collision matrix
    pub allowed_collision_matrix: super::msg::AllowedCollisionMatrix,

    /// all link paddings
    pub link_padding: Vec<super::msg::LinkPadding>,

    /// all link scales
    pub link_scale: Vec<super::msg::LinkScale>,

    /// Attached objects, collision objects, even the octomap or collision map can have
    /// colors associated to them. This array specifies them.
    pub object_colors: Vec<super::msg::ObjectColor>,

    /// the collision map
    pub world: super::msg::PlanningSceneWorld,

    /// Flag indicating whether this scene is to be interpreted as a diff with respect to some other scene
    pub is_diff: bool,

}



impl Default for PlanningScene {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::PlanningScene::default())
  }
}

impl rosidl_runtime_rs::Message for PlanningScene {
  type RmwMsg = super::msg::rmw::PlanningScene;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name.as_str().into(),
        robot_state: super::msg::RobotState::into_rmw_message(std::borrow::Cow::Owned(msg.robot_state)).into_owned(),
        robot_model_name: msg.robot_model_name.as_str().into(),
        fixed_frame_transforms: msg.fixed_frame_transforms
          .into_iter()
          .map(|elem| geometry_msgs::msg::TransformStamped::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
        allowed_collision_matrix: super::msg::AllowedCollisionMatrix::into_rmw_message(std::borrow::Cow::Owned(msg.allowed_collision_matrix)).into_owned(),
        link_padding: msg.link_padding
          .into_iter()
          .map(|elem| super::msg::LinkPadding::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
        link_scale: msg.link_scale
          .into_iter()
          .map(|elem| super::msg::LinkScale::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
        object_colors: msg.object_colors
          .into_iter()
          .map(|elem| super::msg::ObjectColor::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
        world: super::msg::PlanningSceneWorld::into_rmw_message(std::borrow::Cow::Owned(msg.world)).into_owned(),
        is_diff: msg.is_diff,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name.as_str().into(),
        robot_state: super::msg::RobotState::into_rmw_message(std::borrow::Cow::Borrowed(&msg.robot_state)).into_owned(),
        robot_model_name: msg.robot_model_name.as_str().into(),
        fixed_frame_transforms: msg.fixed_frame_transforms
          .iter()
          .map(|elem| geometry_msgs::msg::TransformStamped::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
        allowed_collision_matrix: super::msg::AllowedCollisionMatrix::into_rmw_message(std::borrow::Cow::Borrowed(&msg.allowed_collision_matrix)).into_owned(),
        link_padding: msg.link_padding
          .iter()
          .map(|elem| super::msg::LinkPadding::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
        link_scale: msg.link_scale
          .iter()
          .map(|elem| super::msg::LinkScale::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
        object_colors: msg.object_colors
          .iter()
          .map(|elem| super::msg::ObjectColor::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
        world: super::msg::PlanningSceneWorld::into_rmw_message(std::borrow::Cow::Borrowed(&msg.world)).into_owned(),
      is_diff: msg.is_diff,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      name: msg.name.to_string(),
      robot_state: super::msg::RobotState::from_rmw_message(msg.robot_state),
      robot_model_name: msg.robot_model_name.to_string(),
      fixed_frame_transforms: msg.fixed_frame_transforms
          .into_iter()
          .map(geometry_msgs::msg::TransformStamped::from_rmw_message)
          .collect(),
      allowed_collision_matrix: super::msg::AllowedCollisionMatrix::from_rmw_message(msg.allowed_collision_matrix),
      link_padding: msg.link_padding
          .into_iter()
          .map(super::msg::LinkPadding::from_rmw_message)
          .collect(),
      link_scale: msg.link_scale
          .into_iter()
          .map(super::msg::LinkScale::from_rmw_message)
          .collect(),
      object_colors: msg.object_colors
          .into_iter()
          .map(super::msg::ObjectColor::from_rmw_message)
          .collect(),
      world: super::msg::PlanningSceneWorld::from_rmw_message(msg.world),
      is_diff: msg.is_diff,
    }
  }
}


// Corresponds to moveit_msgs__msg__PlanningSceneComponents
/// This message defines the components that make up the PlanningScene message.
/// The values can be used as a bitfield to specify which parts of the PlanningScene message
/// are of interest

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::PlanningSceneComponents::default())
  }
}

impl rosidl_runtime_rs::Message for PlanningSceneComponents {
  type RmwMsg = super::msg::rmw::PlanningSceneComponents;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        components: msg.components,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      components: msg.components,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      components: msg.components,
    }
  }
}


// Corresponds to moveit_msgs__msg__PlanningSceneWorld
/// collision objects

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PlanningSceneWorld {

    // This member is not documented.
    #[allow(missing_docs)]
    pub collision_objects: Vec<super::msg::CollisionObject>,

    /// The octomap that represents additional collision data
    pub octomap: octomap_msgs::msg::OctomapWithPose,

}



impl Default for PlanningSceneWorld {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::PlanningSceneWorld::default())
  }
}

impl rosidl_runtime_rs::Message for PlanningSceneWorld {
  type RmwMsg = super::msg::rmw::PlanningSceneWorld;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        collision_objects: msg.collision_objects
          .into_iter()
          .map(|elem| super::msg::CollisionObject::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
        octomap: octomap_msgs::msg::OctomapWithPose::into_rmw_message(std::borrow::Cow::Owned(msg.octomap)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        collision_objects: msg.collision_objects
          .iter()
          .map(|elem| super::msg::CollisionObject::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
        octomap: octomap_msgs::msg::OctomapWithPose::into_rmw_message(std::borrow::Cow::Borrowed(&msg.octomap)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      collision_objects: msg.collision_objects
          .into_iter()
          .map(super::msg::CollisionObject::from_rmw_message)
          .collect(),
      octomap: octomap_msgs::msg::OctomapWithPose::from_rmw_message(msg.octomap),
    }
  }
}


// Corresponds to moveit_msgs__msg__PlanningOptions

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PlanningOptions {
    /// The diff to consider for the planning scene (optional)
    pub planning_scene_diff: super::msg::PlanningScene,

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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::PlanningOptions::default())
  }
}

impl rosidl_runtime_rs::Message for PlanningOptions {
  type RmwMsg = super::msg::rmw::PlanningOptions;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        planning_scene_diff: super::msg::PlanningScene::into_rmw_message(std::borrow::Cow::Owned(msg.planning_scene_diff)).into_owned(),
        plan_only: msg.plan_only,
        look_around: msg.look_around,
        look_around_attempts: msg.look_around_attempts,
        max_safe_execution_cost: msg.max_safe_execution_cost,
        replan: msg.replan,
        replan_attempts: msg.replan_attempts,
        replan_delay: msg.replan_delay,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        planning_scene_diff: super::msg::PlanningScene::into_rmw_message(std::borrow::Cow::Borrowed(&msg.planning_scene_diff)).into_owned(),
      plan_only: msg.plan_only,
      look_around: msg.look_around,
      look_around_attempts: msg.look_around_attempts,
      max_safe_execution_cost: msg.max_safe_execution_cost,
      replan: msg.replan,
      replan_attempts: msg.replan_attempts,
      replan_delay: msg.replan_delay,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      planning_scene_diff: super::msg::PlanningScene::from_rmw_message(msg.planning_scene_diff),
      plan_only: msg.plan_only,
      look_around: msg.look_around,
      look_around_attempts: msg.look_around_attempts,
      max_safe_execution_cost: msg.max_safe_execution_cost,
      replan: msg.replan,
      replan_attempts: msg.replan_attempts,
      replan_delay: msg.replan_delay,
    }
  }
}


// Corresponds to moveit_msgs__msg__PositionConstraint
/// This message contains the definition of a position constraint.

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PositionConstraint {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,

    /// The robot link this constraint refers to
    pub link_name: std::string::String,

    /// The offset (in the link frame) for the target point on the link we are planning for
    pub target_point_offset: geometry_msgs::msg::Vector3,

    /// The volume this constraint refers to
    pub constraint_region: super::msg::BoundingVolume,

    /// A weighting factor for this constraint (denotes relative importance to other constraints. Closer to zero means less important)
    pub weight: f64,

}



impl Default for PositionConstraint {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::PositionConstraint::default())
  }
}

impl rosidl_runtime_rs::Message for PositionConstraint {
  type RmwMsg = super::msg::rmw::PositionConstraint;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        link_name: msg.link_name.as_str().into(),
        target_point_offset: geometry_msgs::msg::Vector3::into_rmw_message(std::borrow::Cow::Owned(msg.target_point_offset)).into_owned(),
        constraint_region: super::msg::BoundingVolume::into_rmw_message(std::borrow::Cow::Owned(msg.constraint_region)).into_owned(),
        weight: msg.weight,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        link_name: msg.link_name.as_str().into(),
        target_point_offset: geometry_msgs::msg::Vector3::into_rmw_message(std::borrow::Cow::Borrowed(&msg.target_point_offset)).into_owned(),
        constraint_region: super::msg::BoundingVolume::into_rmw_message(std::borrow::Cow::Borrowed(&msg.constraint_region)).into_owned(),
      weight: msg.weight,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      link_name: msg.link_name.to_string(),
      target_point_offset: geometry_msgs::msg::Vector3::from_rmw_message(msg.target_point_offset),
      constraint_region: super::msg::BoundingVolume::from_rmw_message(msg.constraint_region),
      weight: msg.weight,
    }
  }
}


// Corresponds to moveit_msgs__msg__RobotState
/// This message contains information about the robot state, i.e. the positions of its joints and links

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RobotState {

    // This member is not documented.
    #[allow(missing_docs)]
    pub joint_state: sensor_msgs::msg::JointState,

    /// Joints that may have multiple DOF are specified here
    pub multi_dof_joint_state: sensor_msgs::msg::MultiDOFJointState,

    /// Attached collision objects (attached to some link on the robot)
    pub attached_collision_objects: Vec<super::msg::AttachedCollisionObject>,

    /// Flag indicating whether this scene is to be interpreted as a diff with respect to some other scene
    /// This is mostly important for handling the attached bodies (whether or not to clear the attached bodies
    /// of a moveit::core::RobotState before updating it with this message)
    pub is_diff: bool,

}



impl Default for RobotState {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::RobotState::default())
  }
}

impl rosidl_runtime_rs::Message for RobotState {
  type RmwMsg = super::msg::rmw::RobotState;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        joint_state: sensor_msgs::msg::JointState::into_rmw_message(std::borrow::Cow::Owned(msg.joint_state)).into_owned(),
        multi_dof_joint_state: sensor_msgs::msg::MultiDOFJointState::into_rmw_message(std::borrow::Cow::Owned(msg.multi_dof_joint_state)).into_owned(),
        attached_collision_objects: msg.attached_collision_objects
          .into_iter()
          .map(|elem| super::msg::AttachedCollisionObject::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
        is_diff: msg.is_diff,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        joint_state: sensor_msgs::msg::JointState::into_rmw_message(std::borrow::Cow::Borrowed(&msg.joint_state)).into_owned(),
        multi_dof_joint_state: sensor_msgs::msg::MultiDOFJointState::into_rmw_message(std::borrow::Cow::Borrowed(&msg.multi_dof_joint_state)).into_owned(),
        attached_collision_objects: msg.attached_collision_objects
          .iter()
          .map(|elem| super::msg::AttachedCollisionObject::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      is_diff: msg.is_diff,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      joint_state: sensor_msgs::msg::JointState::from_rmw_message(msg.joint_state),
      multi_dof_joint_state: sensor_msgs::msg::MultiDOFJointState::from_rmw_message(msg.multi_dof_joint_state),
      attached_collision_objects: msg.attached_collision_objects
          .into_iter()
          .map(super::msg::AttachedCollisionObject::from_rmw_message)
          .collect(),
      is_diff: msg.is_diff,
    }
  }
}


// Corresponds to moveit_msgs__msg__RobotTrajectory

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RobotTrajectory {

    // This member is not documented.
    #[allow(missing_docs)]
    pub joint_trajectory: trajectory_msgs::msg::JointTrajectory,


    // This member is not documented.
    #[allow(missing_docs)]
    pub multi_dof_joint_trajectory: trajectory_msgs::msg::MultiDOFJointTrajectory,

}



impl Default for RobotTrajectory {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::RobotTrajectory::default())
  }
}

impl rosidl_runtime_rs::Message for RobotTrajectory {
  type RmwMsg = super::msg::rmw::RobotTrajectory;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        joint_trajectory: trajectory_msgs::msg::JointTrajectory::into_rmw_message(std::borrow::Cow::Owned(msg.joint_trajectory)).into_owned(),
        multi_dof_joint_trajectory: trajectory_msgs::msg::MultiDOFJointTrajectory::into_rmw_message(std::borrow::Cow::Owned(msg.multi_dof_joint_trajectory)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        joint_trajectory: trajectory_msgs::msg::JointTrajectory::into_rmw_message(std::borrow::Cow::Borrowed(&msg.joint_trajectory)).into_owned(),
        multi_dof_joint_trajectory: trajectory_msgs::msg::MultiDOFJointTrajectory::into_rmw_message(std::borrow::Cow::Borrowed(&msg.multi_dof_joint_trajectory)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      joint_trajectory: trajectory_msgs::msg::JointTrajectory::from_rmw_message(msg.joint_trajectory),
      multi_dof_joint_trajectory: trajectory_msgs::msg::MultiDOFJointTrajectory::from_rmw_message(msg.multi_dof_joint_trajectory),
    }
  }
}


// Corresponds to moveit_msgs__msg__VisibilityConstraint
/// The constraint is useful to maintain visibility to a disc (the target) in a particular frame.
/// This disc forms the base of a visibiliy cone whose tip is at the origin of the sensor.
/// Maintaining visibility is done by ensuring the robot does not obstruct the visibility cone.
/// Note:
/// This constraint does NOT enforce minimum or maximum distances between the sensor
/// and the target, nor does it enforce the target to be in the field of view of
/// the sensor. A PositionConstraint can (and probably should) be used for such purposes.

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct VisibilityConstraint {
    /// The radius of the disc that should be maintained visible
    pub target_radius: f64,

    /// The pose of the disc; as the robot moves, the pose of the disc may change as well
    /// This can be in the frame of a particular robot link, for example
    pub target_pose: geometry_msgs::msg::PoseStamped,

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
    pub sensor_pose: geometry_msgs::msg::PoseStamped,

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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::VisibilityConstraint::default())
  }
}

impl rosidl_runtime_rs::Message for VisibilityConstraint {
  type RmwMsg = super::msg::rmw::VisibilityConstraint;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        target_radius: msg.target_radius,
        target_pose: geometry_msgs::msg::PoseStamped::into_rmw_message(std::borrow::Cow::Owned(msg.target_pose)).into_owned(),
        cone_sides: msg.cone_sides,
        sensor_pose: geometry_msgs::msg::PoseStamped::into_rmw_message(std::borrow::Cow::Owned(msg.sensor_pose)).into_owned(),
        max_view_angle: msg.max_view_angle,
        max_range_angle: msg.max_range_angle,
        sensor_view_direction: msg.sensor_view_direction,
        weight: msg.weight,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      target_radius: msg.target_radius,
        target_pose: geometry_msgs::msg::PoseStamped::into_rmw_message(std::borrow::Cow::Borrowed(&msg.target_pose)).into_owned(),
      cone_sides: msg.cone_sides,
        sensor_pose: geometry_msgs::msg::PoseStamped::into_rmw_message(std::borrow::Cow::Borrowed(&msg.sensor_pose)).into_owned(),
      max_view_angle: msg.max_view_angle,
      max_range_angle: msg.max_range_angle,
      sensor_view_direction: msg.sensor_view_direction,
      weight: msg.weight,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      target_radius: msg.target_radius,
      target_pose: geometry_msgs::msg::PoseStamped::from_rmw_message(msg.target_pose),
      cone_sides: msg.cone_sides,
      sensor_pose: geometry_msgs::msg::PoseStamped::from_rmw_message(msg.sensor_pose),
      max_view_angle: msg.max_view_angle,
      max_range_angle: msg.max_range_angle,
      sensor_view_direction: msg.sensor_view_direction,
      weight: msg.weight,
    }
  }
}


// Corresponds to moveit_msgs__msg__WorkspaceParameters
/// This message contains a set of parameters useful in
/// setting up the volume (a box) in which the robot is allowed to move.
/// This is useful only when planning for mobile parts of
/// the robot as well.

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct WorkspaceParameters {
    /// Define the frame of reference for the box corners
    pub header: std_msgs::msg::Header,

    /// The minumum corner of the box, with respect to the robot starting pose
    pub min_corner: geometry_msgs::msg::Vector3,

    /// The maximum corner of the box, with respect to the robot starting pose
    pub max_corner: geometry_msgs::msg::Vector3,

}



impl Default for WorkspaceParameters {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::WorkspaceParameters::default())
  }
}

impl rosidl_runtime_rs::Message for WorkspaceParameters {
  type RmwMsg = super::msg::rmw::WorkspaceParameters;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        min_corner: geometry_msgs::msg::Vector3::into_rmw_message(std::borrow::Cow::Owned(msg.min_corner)).into_owned(),
        max_corner: geometry_msgs::msg::Vector3::into_rmw_message(std::borrow::Cow::Owned(msg.max_corner)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        min_corner: geometry_msgs::msg::Vector3::into_rmw_message(std::borrow::Cow::Borrowed(&msg.min_corner)).into_owned(),
        max_corner: geometry_msgs::msg::Vector3::into_rmw_message(std::borrow::Cow::Borrowed(&msg.max_corner)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      min_corner: geometry_msgs::msg::Vector3::from_rmw_message(msg.min_corner),
      max_corner: geometry_msgs::msg::Vector3::from_rmw_message(msg.max_corner),
    }
  }
}


// Corresponds to moveit_msgs__msg__KinematicSolverInfo
/// A list of joints in the kinematic tree

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct KinematicSolverInfo {

    // This member is not documented.
    #[allow(missing_docs)]
    pub joint_names: Vec<std::string::String>,

    /// A list of joint limits corresponding to the joint names
    pub limits: Vec<super::msg::JointLimits>,

    /// A list of links that the kinematics node provides solutions for
    pub link_names: Vec<std::string::String>,

}



impl Default for KinematicSolverInfo {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::KinematicSolverInfo::default())
  }
}

impl rosidl_runtime_rs::Message for KinematicSolverInfo {
  type RmwMsg = super::msg::rmw::KinematicSolverInfo;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        joint_names: msg.joint_names
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        limits: msg.limits
          .into_iter()
          .map(|elem| super::msg::JointLimits::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
        link_names: msg.link_names
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        joint_names: msg.joint_names
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        limits: msg.limits
          .iter()
          .map(|elem| super::msg::JointLimits::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
        link_names: msg.link_names
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      joint_names: msg.joint_names
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
      limits: msg.limits
          .into_iter()
          .map(super::msg::JointLimits::from_rmw_message)
          .collect(),
      link_names: msg.link_names
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
    }
  }
}


// Corresponds to moveit_msgs__msg__PositionIKRequest
/// A Position IK request message

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PositionIKRequest {
    /// The name of the group which will be used to compute IK
    /// e.g. "right_arm", or "arms" - see IK specification for multiple-groups below
    /// Information from the SRDF will be used to automatically determine which link
    /// to solve IK for, unless ik_link_name is also specified
    pub group_name: std::string::String,

    /// A RobotState consisting of hint/seed positions for the IK computation and positions
    /// for all the other joints in the robot. Additional state information provided here is
    /// used to specify starting positions for other joints/links on the robot.
    /// This state MUST contain state for all joints to be used by the IK solver
    /// to compute IK. The list of joints that the IK solver deals with can be
    /// found using the SRDF for the corresponding group
    pub robot_state: super::msg::RobotState,

    /// A set of constraints that the IK must obey; by default, this set of constraints is empty
    pub constraints: super::msg::Constraints,

    /// Find an IK solution that avoids collisions. By default, this is false
    pub avoid_collisions: bool,

    /// (OPTIONAL) The name of the link for which we are computing IK
    /// If not specified, the link name will be inferred from a combination
    /// of the group name and the SRDF. If any values are specified for ik_link_names,
    /// this value is ignored
    pub ik_link_name: std::string::String,

    /// The stamped pose of the link, when the IK solver computes the joint values
    /// for all the joints in a group. This value is ignored if pose_stamped_vector
    /// has any elements specified.
    pub pose_stamped: geometry_msgs::msg::PoseStamped,

    /// Multi-group parameters
    /// (OPTIONAL) The names of the links for which we are computing IK
    /// If not specified, the link name will be inferred from a combination
    /// of the group name and the SRDF
    pub ik_link_names: Vec<std::string::String>,

    /// (OPTIONAL) The (stamped) poses of the links we are computing IK for (when a group has more than one end effector)
    /// e.g. The "arms" group might consist of both the "right_arm" and the "left_arm"
    /// The order of the groups referred to is the same as the order setup in the SRDF
    pub pose_stamped_vector: Vec<geometry_msgs::msg::PoseStamped>,

    /// Maximum allowed time for IK calculation
    pub timeout: builtin_interfaces::msg::Duration,

}



impl Default for PositionIKRequest {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::PositionIKRequest::default())
  }
}

impl rosidl_runtime_rs::Message for PositionIKRequest {
  type RmwMsg = super::msg::rmw::PositionIKRequest;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        group_name: msg.group_name.as_str().into(),
        robot_state: super::msg::RobotState::into_rmw_message(std::borrow::Cow::Owned(msg.robot_state)).into_owned(),
        constraints: super::msg::Constraints::into_rmw_message(std::borrow::Cow::Owned(msg.constraints)).into_owned(),
        avoid_collisions: msg.avoid_collisions,
        ik_link_name: msg.ik_link_name.as_str().into(),
        pose_stamped: geometry_msgs::msg::PoseStamped::into_rmw_message(std::borrow::Cow::Owned(msg.pose_stamped)).into_owned(),
        ik_link_names: msg.ik_link_names
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        pose_stamped_vector: msg.pose_stamped_vector
          .into_iter()
          .map(|elem| geometry_msgs::msg::PoseStamped::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
        timeout: builtin_interfaces::msg::Duration::into_rmw_message(std::borrow::Cow::Owned(msg.timeout)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        group_name: msg.group_name.as_str().into(),
        robot_state: super::msg::RobotState::into_rmw_message(std::borrow::Cow::Borrowed(&msg.robot_state)).into_owned(),
        constraints: super::msg::Constraints::into_rmw_message(std::borrow::Cow::Borrowed(&msg.constraints)).into_owned(),
      avoid_collisions: msg.avoid_collisions,
        ik_link_name: msg.ik_link_name.as_str().into(),
        pose_stamped: geometry_msgs::msg::PoseStamped::into_rmw_message(std::borrow::Cow::Borrowed(&msg.pose_stamped)).into_owned(),
        ik_link_names: msg.ik_link_names
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        pose_stamped_vector: msg.pose_stamped_vector
          .iter()
          .map(|elem| geometry_msgs::msg::PoseStamped::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
        timeout: builtin_interfaces::msg::Duration::into_rmw_message(std::borrow::Cow::Borrowed(&msg.timeout)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      group_name: msg.group_name.to_string(),
      robot_state: super::msg::RobotState::from_rmw_message(msg.robot_state),
      constraints: super::msg::Constraints::from_rmw_message(msg.constraints),
      avoid_collisions: msg.avoid_collisions,
      ik_link_name: msg.ik_link_name.to_string(),
      pose_stamped: geometry_msgs::msg::PoseStamped::from_rmw_message(msg.pose_stamped),
      ik_link_names: msg.ik_link_names
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
      pose_stamped_vector: msg.pose_stamped_vector
          .into_iter()
          .map(geometry_msgs::msg::PoseStamped::from_rmw_message)
          .collect(),
      timeout: builtin_interfaces::msg::Duration::from_rmw_message(msg.timeout),
    }
  }
}


// Corresponds to moveit_msgs__msg__PipelineState
/// Motion plan response that is processed

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PipelineState {

    // This member is not documented.
    #[allow(missing_docs)]
    pub request: super::msg::MotionPlanRequest,

    /// Current solution
    pub response: super::msg::MotionPlanResponse,

    /// Latest stage of the planning pipeline that got invoked
    pub pipeline_stage: std::string::String,

}



impl Default for PipelineState {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::PipelineState::default())
  }
}

impl rosidl_runtime_rs::Message for PipelineState {
  type RmwMsg = super::msg::rmw::PipelineState;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        request: super::msg::MotionPlanRequest::into_rmw_message(std::borrow::Cow::Owned(msg.request)).into_owned(),
        response: super::msg::MotionPlanResponse::into_rmw_message(std::borrow::Cow::Owned(msg.response)).into_owned(),
        pipeline_stage: msg.pipeline_stage.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        request: super::msg::MotionPlanRequest::into_rmw_message(std::borrow::Cow::Borrowed(&msg.request)).into_owned(),
        response: super::msg::MotionPlanResponse::into_rmw_message(std::borrow::Cow::Borrowed(&msg.response)).into_owned(),
        pipeline_stage: msg.pipeline_stage.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      request: super::msg::MotionPlanRequest::from_rmw_message(msg.request),
      response: super::msg::MotionPlanResponse::from_rmw_message(msg.response),
      pipeline_stage: msg.pipeline_stage.to_string(),
    }
  }
}


// Corresponds to moveit_msgs__msg__ServoStatus
/// Message

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ServoStatus {
    /// will contains integer code
    pub code: i8,

    /// will contain explanatory message
    pub message: std::string::String,

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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::ServoStatus::default())
  }
}

impl rosidl_runtime_rs::Message for ServoStatus {
  type RmwMsg = super::msg::rmw::ServoStatus;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        code: msg.code,
        message: msg.message.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      code: msg.code,
        message: msg.message.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      code: msg.code,
      message: msg.message.to_string(),
    }
  }
}


