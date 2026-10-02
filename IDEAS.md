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

# Preset samples
Some ideas for preset configuration

```jsonc
{
    "content": [ // length > 0 and length < 10, just need to verify < 10 lines and < 144 characters, we should NOT allow the \n character, consider preset invalid if present.
        "Hello World"
    ]
}
```

```jsonc
{
    "content": [
        "CPU: {cpu}" // telemetry with automatic compaction, must verify preset by assuming worst case scenario with most compaction, for some telemetries we should consider stripping some characters like \n to not mess with calculations
    ]
}
```

```jsonc
{
    "compaction": { // optional
        "overrides": { // optional
            "cpu": 1 // forces a specific compaction level, 1 being least and progressively more, this should also be easier to parse then trying to do like {cpu:1}, must always calculate worse case scenario to verify preset
        }
    },
    "content": [
        "CPU: {cpu}"
    ]
}
```

```jsonc
{
    "compaction": {
        "overrides": {
            "cpu": 1
        },
        "priority": [
            "media",
            "ram",
            "gpu",
            "cpu",
            "fps"
        ], // optional compaction priority list, if not provided default ruleset for all telemetry types will be used, if provided all telemetry used must be included in the list (to avoid the edge case of where should priority list insert itself)
    },
    "content": [
        "CPU: {cpu} | RAM: {ram}",
        "GPU: {gpu} | FPS: {fps}",
        "[🎵 {media}]"
    ],
}
```

```jsonc
{
    "content": [
        "Pretend there are >= 10 strings in here..."
    ],
    "carousel": {
        "maxLines": 4, // optional, determines max number of lines, some people may prefer only 3 lines for a shorter box with more information, we need to calculate the worst case scenario by doing something like calculating worse line length for each line, then checking for each line if the sum of any of those line lengths + the next X line lengths is greater than 144 then its invalid
        "linesPerUpdate": 2 // optional, how many lines to scroll by every update, default to 1, must be less than max lines.
    } // optional, makes content support more than 9 lines by basically scrolling the entire thing every update by this many lines, this will be super useful for being stylish and showing more information
}
```