# Step 03 - Hardcoded Pick-and-Place Loop

## Objective
Create a repeatable program loop that sorts boxes into two bins using simple rules.

## Suggested Rule
- Odd index box -> Bin A
- Even index box -> Bin B

## Program Logic (PolyScope URScript / Teach Pendant)

### Build this in the PolyScope Program editor:

```
Program
  Loop 10 times
    MoveJ  home
    MoveL  pick_approach
    MoveL  pick_contact
    Set Digital Out[0] = HIGH   # simulates Vacuum ON
    Wait 0.2s
    MoveL  pick_lift

    If  loop_index mod 2 == 0:
      MoveJ  placeA_approach
      MoveL  placeA_drop
    Else:
      MoveJ  placeB_approach
      MoveL  placeB_drop
    End If

    Set Digital Out[0] = LOW    # simulates Vacuum OFF
    Wait 0.1s
  End Loop
```

> **MoveJ** = joint-space move (fast, ignores path).
> **MoveL** = linear move (follows straight TCP path — use near objects).

### URScript equivalent (for reference / automation):

```python
# This is the low-level language PolyScope compiles to.
# You can also send it over TCP port 30001 from Python.
movej([0, -1.57, 0, -1.57, 0, 0], a=1.2, v=0.25)  # home
movel(p[0.3, 0.1, 0.4, 0, 3.14, 0], a=0.5, v=0.1)  # pick_approach
```

## Validation
- Runs 10 consecutive cycles without collision.
- Tool orientation stays consistent on approach.
- No singularity or joint limit warnings in PolyScope log.

## Stretch Goal
Control the robot loop from Python over the URScript TCP socket (port 30001) instead of the
teach pendant — this bridges Phase 1 to Phase 2.
