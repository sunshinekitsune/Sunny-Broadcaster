# Telemetry
This is a list of different information that might be interesting to collect for the display.

## CPU Metrics
- Total CPU usage
- Core clock and package clock frequencies
- Temperature *C and *F
- Total power draw
- CPU name
- Add/remove CPUs, for the edge case of PCs with multiple CPUs

## RAM Metrics
- RAM utilization
    - Percentage used
    - Total used
    - Total available
- System commit charge
- Swap memory usage

## GPU Metrics
- Core GPU load percentage
- Core temperature and hotspot temperature
- Core memory and clock
- VRAM utilization
    - Percentage used
    - Total used
    - Total available
- Total power draw
- GPU name
- Add/remove GPUs (to account for multiGPU setups and iGPU+dGPU edgecases)

## Network Metrics
- Current download and upload speed
- Total data transferred up and down since launch
- Latency of local network
- Interface Name

## Media Metrics
- Audio title
- Artist name
- Album name
- Playback state
    - Is playing
    - Progress bar with ASCII

## SteamVR Metrics
- HMD Battery and charing state
- Controller battery
- FPS
- Dropped frames and reprojection
- Frame timing latency
- Headset uptime
- Current game session time
- AFK time

## OyasumiVR
- Sleep status
- Total sleep counter
- Alarm
    - Countdown till goes off

## Clock
- Local time (12hr, 24hr)
- Current UTC timezoen

## Desktop
- Name of focus application on desktop
- Total processes

## Misc. Metrics
- Total session time of program
- Messages sent
- Region of user
- Arbitrary text
- Discord RPC integration