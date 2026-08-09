#!/usr/bin/env python3
"""Sample simple top-down grasp candidates around a mesh and filter collisions.

This is an educational starter script, not a production grasp planner.
"""

from __future__ import annotations

import argparse
import json
import math
from dataclasses import asdict, dataclass
from pathlib import Path

import numpy as np
import trimesh


@dataclass
class GraspCandidate:
    angle_deg: float
    contact_center: list[float]
    jaw_width: float


def build_primitive(name: str) -> trimesh.Trimesh:
    if name == "box":
        return trimesh.creation.box(extents=(0.08, 0.06, 0.12))
    if name == "cylinder":
        return trimesh.creation.cylinder(radius=0.035, height=0.12)
    if name == "capsule":
        return trimesh.creation.capsule(radius=0.03, height=0.10)
    raise ValueError(f"Unsupported primitive: {name}")


def sample_angles(count: int) -> np.ndarray:
    return np.linspace(0.0, 2.0 * math.pi, num=count, endpoint=False)


def estimate_jaw_width(bounds_xy: np.ndarray, theta: float) -> float:
    # Rotate XY corners and measure projected width along local Y axis.
    c, s = math.cos(theta), math.sin(theta)
    rot = np.array([[c, -s], [s, c]])
    rotated = bounds_xy @ rot.T
    y_min, y_max = rotated[:, 1].min(), rotated[:, 1].max()
    return float(y_max - y_min)


def generate_candidates(mesh: trimesh.Trimesh, num_samples: int, max_jaw: float) -> list[GraspCandidate]:
    bounds = mesh.bounds
    min_corner, max_corner = bounds[0], bounds[1]

    # Use XY corners of the AABB as a lightweight proxy for collision width checks.
    corners_xy = np.array(
        [
            [min_corner[0], min_corner[1]],
            [min_corner[0], max_corner[1]],
            [max_corner[0], min_corner[1]],
            [max_corner[0], max_corner[1]],
        ]
    )

    center = mesh.centroid
    valid: list[GraspCandidate] = []

    for theta in sample_angles(num_samples):
        jaw = estimate_jaw_width(corners_xy, theta)

        # If required jaw opening exceeds gripper capability, reject candidate.
        if jaw <= max_jaw:
            valid.append(
                GraspCandidate(
                    angle_deg=float(np.degrees(theta)),
                    contact_center=[float(center[0]), float(center[1]), float(max_corner[2])],
                    jaw_width=jaw,
                )
            )

    return valid


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Synthetic grasp angle sampler")
    parser.add_argument("--mesh", type=str, default="", help="Path to input mesh file (stl/obj/ply)")
    parser.add_argument(
        "--primitive",
        type=str,
        default="cylinder",
        choices=["box", "cylinder", "capsule"],
        help="Fallback primitive if no --mesh is supplied",
    )
    parser.add_argument("--samples", type=int, default=100, help="Number of angle samples around Z axis")
    parser.add_argument("--max-jaw", type=float, default=0.08, help="Maximum gripper jaw opening in meters")
    parser.add_argument("--out", type=str, default="phase2/output/grasps.json", help="Output JSON path")
    return parser.parse_args()


def main() -> None:
    args = parse_args()

    if args.mesh:
        mesh = trimesh.load(args.mesh, force="mesh")
        if not isinstance(mesh, trimesh.Trimesh):
            raise TypeError("Loaded geometry is not a single mesh")
    else:
        mesh = build_primitive(args.primitive)

    candidates = generate_candidates(mesh, num_samples=args.samples, max_jaw=args.max_jaw)

    out_path = Path(args.out)
    out_path.parent.mkdir(parents=True, exist_ok=True)
    out_path.write_text(json.dumps([asdict(c) for c in candidates], indent=2), encoding="utf-8")

    print(f"Candidates checked: {args.samples}")
    print(f"Valid grasps: {len(candidates)}")
    print(f"Saved: {out_path}")


if __name__ == "__main__":
    main()
