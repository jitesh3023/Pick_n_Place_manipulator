### Creating moveit workspace and mounting it on ubuntu
Our external ssd is of filetype exFAT due to which we cannot give linux permissions and symbolic links. So, as a storage workaround we will reserve around 60gb from the external exFAT ssd as one large file and then let my system Ubuntu use that file like a small linux disk.
![alt text](../Images/exFAT_ext4_hierarchy.png)
/mnt/moveit is just the access point, called a mount point. /mnt is a conventional location for mounting filesystems.

# For creating ext4 and mounting it to my ubuntu
(
  set -e

  SSD="/media/jitesh/Extreme SSD"
  IMG="$SSD/linux-workspaces/moveit.ext4"
  MOUNT="/mnt/moveit"

  if ! mountpoint -q "$SSD"; then
    echo "STOP: External SSD is not mounted."
    exit 1
  fi

  if [ -e "$IMG" ] || [ -L "$IMG" ]; then
    echo "STOP: Image already exists. Do not format it."
    exit 1
  fi

  if mountpoint -q "$MOUNT"; then
    echo "STOP: Workspace is already mounted."
    exit 1
  fi

  if [ -d "$MOUNT" ] && [ -n "$(ls -A "$MOUNT")" ]; then
    echo "STOP: Mount directory contains files."
    exit 1
  fi

  sudo -v
  mkdir -p "$(dirname "$IMG")"

  # Allocate 60 GiB; refuse to overwrite an existing file.
  dd if=/dev/zero of="$IMG" bs=16M count=3840 \
    conv=excl,fsync status=progress

  # Format only the newly created image file.
  sudo mkfs.ext4 -F -m 0 -L moveit_ws "$IMG"

  sudo mkdir -p "$MOUNT"
  sudo mount -o loop "$IMG" "$MOUNT"
  sudo chown "$(id -u):$(id -g)" "$MOUNT"

  mkdir -p "$MOUNT/ws_moveit/src"

  findmnt --mountpoint "$MOUNT" -o SOURCE,FSTYPE,TARGET
  df -h "$MOUNT"
)


# Mounting and Unmounting
Mounting -
sudo mount -o loop \
  "/media/jitesh/Extreme SSD/linux-workspaces/moveit.ext4" \
  /mnt/moveit

Unmounting -
cd ~
sudo umount /mnt/moveit