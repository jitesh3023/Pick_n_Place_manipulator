# Step 01 - Install and Base Cell Setup (URSim)

## Objective
Install URSim (Universal Robots official free simulator) and load a virtual UR5e robot arm.

## What is URSim?
URSim runs an actual PolyScope controller inside a VM or Docker container — the exact same
software that runs on a real UR robot pendant. No license needed.

## Installation (Docker — easiest on Linux)

```bash
# Pull the official UR image (pick the e-Series controller version)
docker pull universalrobots/ursim_e-series

# Run it — exposes PolyScope web UI on port 6080 and URScript on 30001 
docker run --rm -it \                   # host-port:container-port
  -p 6080:6080 \                        # Exposes URSim's web/VNC UI so you can open the pendant in browser at 6080 port
  -p 29999:29999 \                      # Exposes UR Dashboard server port. Used for high level controller commands like load/start/stop/power,etc
  -p 30001-30004:30001-30004 \          # Exposes for robot communication interfaces. 30001: Primary Interface. 30002: Secondary Interface, 30003: Real-time Interface, 30004: RTDE(Structured real-time data exchange) 
                                        # These are used by external apps like PLC, monitoring/control tools, python, etc
  universalrobots/ursim_e-series:latest
```
<!-- docker run --rm -it \
  -p 6080:6080 \
  -p 29999:29999 \
  -p 30001-30004:30001-30004 \
  universalrobots/ursim_e-series:latest -->

Then open a browser and go to: `http://localhost:6080/vnc.html`

You will see the PolyScope 5 pendant interface — this is what operators use on real factory floors.

## Tasks
1. Run the Docker command above.
2. Open `http://localhost:6080/vnc.html` in your browser.
3. Click **"Skip"** past the startup wizard (or configure robot type as UR5e).
4. Confirm the robot arm 3D view loads without errors.
5. Note down which PolyScope version is shown (bottom-left of screen).

## Validation
- PolyScope 3D arm renders inside the browser.
- The **Freedom Drive** button on the virtual pendant is responsive.
- No red fault banner on screen.
