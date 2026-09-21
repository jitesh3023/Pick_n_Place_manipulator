#!/usr/bin/python3
"""Minimal joint commands for Isaac Sim. No collision or joint-limit checks."""

import math

import rclpy
from sensor_msgs.msg import JointState


def main():
    rclpy.init()
    node = rclpy.create_node("joint_teleop")
    publisher = node.create_publisher(JointState, "/joint_command", 10)
    joints = [
        "shoulder_pan_joint", "shoulder_lift_joint", "elbow_joint",
        "wrist_1_joint", "wrist_2_joint", "wrist_3_joint",
    ]
    for number, name in enumerate(joints, 1):
        print(number, name)
    print("Enter: joint_number angle_degrees (example: 6 10). Enter q to quit.")
    try:
        while rclpy.ok():
            text = input("> ").strip()
            if text.lower() == "q":
                break
            try:
                joint, angle = text.split()
                index, degrees = int(joint) - 1, float(angle)
                if not 0 <= index < 6 or not math.isfinite(degrees):
                    raise ValueError
            except ValueError:
                print("Use joint 1–6 and a finite angle in degrees, e.g. 6 10.")
                continue
            if publisher.get_subscription_count() == 0:
                print("No subscriber. Press Play in Isaac and try again.")
                continue
            message = JointState()
            message.name = [joints[index]]
            message.position = [math.radians(degrees)]
            publisher.publish(message)
            print(f"Sent {joints[index]} -> {degrees} degrees")
    except (KeyboardInterrupt, EOFError):
        pass
    finally:
        node.destroy_node()
        if rclpy.ok():
            rclpy.shutdown()


if __name__ == "__main__":
    main()