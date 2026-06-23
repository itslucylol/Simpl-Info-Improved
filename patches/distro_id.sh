#!/bin/bash
#!sim
#(sim_id:distro_id)

DISTRO_ID="linux"

# Extract the clean machine-readable ID from os-release
if [ -f /etc/os-release ]; then
    # Source the file in a subshell to avoid overwriting existing script variables
    DISTRO_ID=$((. /etc/os-release && echo "$ID") | tr '[:lower:]' '[:upper:]' | tr '-' '_')
fi

# Handle specific edge cases or derivatives if necessary
# (e.g., if it's Pop!_OS, its ID is "pop", but it uses the Ubuntu logo framework)
case "$DISTRO_ID" in
    POP)          DISTRO_ID="UBUNTU" ;;
    LINUXMINT)    DISTRO_ID="MINT" ;;
    ARTIX)        DISTRO_ID="ARCH" ;;
    FEDORA)       DISTRO_ID="FEDORA" ;;
esac

# Safely pull the variable using indirect expansion
LOGO_VAR="SIM_${DISTRO_ID}"
LOGO_CONTENT="${!LOGO_VAR}"

# Ultimate Fallback: If the specific logo isn't compiled, try generic "Linux" type
if [ -z "$LOGO_CONTENT" ]; then
    if [ -n "$SIM_LINUX" ]; then
        LOGO_CONTENT="$SIM_LINUX"
    fi
fi