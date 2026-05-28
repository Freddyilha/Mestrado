#!/bin/bash
set -e

# Detect the UID/GID of the mounted workspace
WORKSPACE_UID=$(stat -c "%u" /workspace)
WORKSPACE_GID=$(stat -c "%g" /workspace)

# Create group if it doesn't exist
if ! getent group "$WORKSPACE_GID" > /dev/null 2>&1; then
    groupadd -g "$WORKSPACE_GID" devgroup
fi

# Create user if it doesn't exist
if ! getent passwd "$WORKSPACE_UID" > /dev/null 2>&1; then
    useradd -m -u "$WORKSPACE_UID" -g "$WORKSPACE_GID" -s /bin/bash devuser
fi

# Run the command as that user
exec gosu "$WORKSPACE_UID" "$@"
