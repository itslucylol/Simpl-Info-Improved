#!/bin/bash
#!sim
#(sim_id:render)

# Define the info lines into a variable block
info_output=$(cat <<EOF
${c21}${user}@${HOST}${c20}
-----------------------
${c17}Distro:${c20} ${DIS}
${c1}Kernel:${c20} ${OS}
${c16}Uptime:${c20} ${UPTIME}
${c11}DE:${c20} ${DE}
${c2}Device:${c20} ${DVI}
${c6}CPU:${c20} ${CPU}
${c4}GPU:${c20} ${GPU}
${c5}RAM:${c20} ${RAM}
${c8}Best Anime:${c21} ${Best_anime}

${c20}Simplinfo LE v2.0 (${CURRENT_YEAR})
${c20}SIM          v1.0
EOF
)

# Read both string blocks into clean Bash arrays line-by-line
mapfile -t logo_lines <<< "${LOGO_CONTENT//\\n/$'\n'}"
mapfile -t info_lines <<< "$info_output"

# DYNAMIC WIDTH CALCULATION
max_logo_width=0
for line in "${logo_lines[@]}"; do
    clean_line=$(echo "$line" | sed 's/\x1b\[[0-9;]*m//g; s/\x1b(B//g; s/\x1b\[?25[hl]//g')
    line_len=${#clean_line}
    if [ "$line_len" -gt "$max_logo_width" ]; then
        max_logo_width=$line_len
    fi
done

# Safety threshold fallbacks
if [ "$max_logo_width" -eq 0 ] || [ "$max_logo_width" -gt 80 ]; then
    max_logo_width=35
fi

# Determine total lines to process
max_lines=${#logo_lines[@]}
if [ ${#info_lines[@]} -gt $max_lines ]; then
    max_lines=${#info_lines[@]}
fi

echo ""

# If length parsing goes wonky or empty, fall back to a reasonable base margin
if [ "$max_logo_width" -eq 0 ] || [ "$max_logo_width" -gt 80 ]; then
    max_logo_width=35
fi

# Add a neat 4-space gap between the edge of the logo and the text column
padding_width=$((max_logo_width + 4))

# Determine whichever block has more lines so we don't truncate anything
max_lines=${#logo_lines[@]}
if [ ${#info_lines[@]} -gt $max_lines ]; then
    max_lines=${#info_lines[@]}
fi

echo ""

# Print them perfectly side-by-side using the calculated dynamic padding width
for ((i=0; i<max_lines; i++)); do
    log_line="${logo_lines[i]}"
    inf_line="${info_lines[i]}"
    
    # Extract the visible length of the current logo line
    clean_log_line=$(echo "$log_line" | sed 's/\x1b\[[0-9;]*m//g; s/\x1b(B//g; s/\x1b\[?25[hl]//g')
    log_line_len=${#clean_log_line}
    
    # Calculate the precise number of spaces needed to reach the text column
    # (max_logo_width + 4 spaces for the gap)
    needed_spaces=$((max_logo_width + 4 - log_line_len))
    if [ "$needed_spaces" -lt 0 ]; then
        needed_spaces=0
    fi
    
    # Construct the exact spacing block
    padding=$(printf '%*s' "$needed_spaces" "")
    
    # Print the logo chunk, the custom spacing, and the info data safely
    printf "%s%s%b\n" "$log_line" "$padding" "$inf_line"
done

echo ""