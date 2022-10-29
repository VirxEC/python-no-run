import subprocess
import sys


def start():
    procs = []
    while True:
        print("Listening for new command...")
        command = sys.stdin.readline()
        params = command.split(" | ")
        print(f"Got command: {command}")

        if params[0] == "stop":
            print("Stopping...")
            # Break from loop
            break
        elif params[0] == "start":
            print("Starting...")
            # Spawn the subprocess
            proc = subprocess.Popen([sys.executable, "say-hi.py"])
            # Use shell=True to see "Hello world!" start after the proccess has been terminated (???)
            # proc = subprocess.Popen([sys.executable, "say-hi.py"], shell=True)

            # Add the subprocess to the list so we can kill it later
            procs.append(proc)

    print("Shutting down")
    # Terminate processes
    for proc in procs:
        proc.terminate()
    