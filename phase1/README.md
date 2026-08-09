# Phase 1 - Digital Twin Warehouse

Build an industrial-style virtual robotic cell:
- Conveyor with incoming boxes
- Robot arm with vacuum gripper
- Pick-and-place into category bins

## Deliverables
- A saved simulator station/project file
- Defined TCP for gripper tip
- 6-10 stable waypoints
- A repeatable loop that sorts boxes

## Primary Tool
**URSim** — free official UR simulator. Runs via Docker on Linux.

## Progress Checklist
- [ ] Docker installed and URSim container running
- [ ] PolyScope pendant accessible at `http://localhost:6080/vnc.html`
- [ ] TCP `vacuum_tip` defined (Z offset 0.08 m)
- [ ] 8 waypoints taught and saved
- [ ] Pick-and-place program built in PolyScope program editor
- [ ] Program runs 10 cycles without collision or joint-limit warnings
