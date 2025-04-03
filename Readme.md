## Experiment in A.I to create a RUST program to read Betaflight Blackbox PID, FF

### Prerequisite

https://www.rust-lang.org/tools/install

### Build and execute

```shell
clear
cargo build --release && ./target/release/blackbox_analyzer --input path/to/BTFL_Log.BBL
ls -lhrt *.png *.csv
```

### 100% "Vibe Coded"

This project was created with https://perplexity.ai, at a cost of many attempts to explain and fix issues during the process. It does not currently produce proper values nor a proper graph; However, it was the closest to a working state of reading BBL files Is i was able to get with strict A.I.

A.I was asked to use both Blackbox-log-viewer and PlasmaTree as refrences to write a RUST program to read BBL files and produce step-responses but calculate the PID's without FF.  When that repeatedly failed, I took a step back and asked A.I. to only read, plot and output PID and FF.  After many fixes, I asked A.I. to reintruduce removal of FF, but output both PID and PID without FF.

This is the result (9d5cf7b); However, the data presented is nonsense. Review issue #1 for current status and example data. It reads rather agnostically. Seemingly with no skipping of headers and does use the correct PID field order that I can tell. (At least according to the BBE's csv export for comparison). I decided to commit and upload the project because it may be a good starting point for further development.
