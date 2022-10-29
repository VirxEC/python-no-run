# python-no-run

Minimal reproducable example for a very stange bug that happens in Windows when interfacing with Python from Rust via stdin

## In the root folder
 - Run `cargo r` and see that nothing happens
 - In `python/main.py` comment out line 20 and uncomment line 22
 - Run `cargo r` and see that "Hello world" gets spammed when the stop command is issued and never stops

## In the `python` folder
 - Run `python -c "from main import start; start()"` to start the process
 - Enter `start | ` exactly and see "Hello world" gets spammed
 - Enter `stop | ` exactly and see "Hello world" stop being spammed, processes all exit successfully
