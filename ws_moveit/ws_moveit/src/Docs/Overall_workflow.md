This is the overall workflow of the project which consists of 2 phases - A and B -> 

"put the red cube in the bowl"
                              |
        +---------------------+----------------------+
        |                                            |
  [A] Modular path                            [B] VLA path
  VLM task parser -> JSON steps               SmolVLA policy (RGB x2 + joint state + text)
  Grounding DINO / OWLv2 + SAM2 -> masks         -> action chunk (joint targets or EEF deltas)
  depth deprojection -> 3D object poses                    |
        |                                            |
        v                                            v
  MoveIt Task Constructor (pick/place)         Safety gate node
        |                                      (limits, step size, workspace box,
        |                                       /check_state_validity, watchdog)
        |                                            |
        +-------------------+------------------------+
                            v
          MoveIt 2 planning scene (URDF + collision objects + Octomap from depth)
          move_group / MoveIt Servo (with collision checking)
                            v
          ros2_control -> joint_trajectory_controller -> Isaac Sim / Gazebo / real UR5e


Rough phase and stages I would be following -> 

| Phase | Weeks | Deliverable | Done when |
| :--- | :--- | :--- | :--- |
| 0. Baseline | 0.5 | MoveIt + UR5e in RViz (UR mock hardware) | Gen3 mock demo done; redo with UR5e (about an hour) |
| 1. Sim hardware loop | 1–2 | UR5e + 2F-85 description/MoveIt config, then Isaac Sim via ros2_control | RViz Plan + Execute moves the simulated arm |
| 2. Cameras + collision avoidance | 1 | RGB-D cameras, Octomap in planning scene | Arm plans around an unknown box placed in the scene |
| 3. Vision understanding | 2 | Language -> 3D object pose -> CollisionObject | "the red cube" yields a correct pose within ~1–2 cm in sim |
| 4. Classical pick and place | 2 | MTC pick/place from language | 80%+ success over 50 randomized trials |
| 4b. Hardening + classical eval | 1–2 | Retries, scene refresh, multi-step tasks, benchmark | Stage 1 results table + video; end of Stage 1 |
| 5. Data generation (Stage 2) | 1–2 | 200–400 auto-labelled episodes in LeRobot format | Dataset loads and replays correctly |
| 6. VLA fine-tune + gated execution (Stage 2) | 2–3 | SmolVLA closed loop through safety gate | Runs end-to-end, gate blocks unsafe actions |
| 7. Evaluation + write-up (Stage 2) | 1 | Benchmark table, videos, README/blog | Classical vs VLA vs hybrid compared |
