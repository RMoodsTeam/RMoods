import os
import subprocess
import sys


def run_uvicorn():
    # Get port from environment variable, default to 8002 if not set
    port = os.environ.get('PORT', '8002')

    # Ensure port is a valid integer
    try:
        port = int(port)
    except ValueError:
        print(f"Error: PORT environment variable must be a valid integer. Got: {port}")
        sys.exit(1)

    # Command as a list of arguments
    command = [
        "uvicorn",
        "src.main:app",
        "--host", "0.0.0.0",
        "--port", str(port),
        "--reload"
    ]

    try:
        # Run the command
        subprocess.run(command, check=True)
    except subprocess.CalledProcessError as e:
        print(f"Error running uvicorn: {e}")
        sys.exit(1)
    except FileNotFoundError:
        print("Error: uvicorn not found. Please ensure it is installed and in your PATH")
        sys.exit(1)


if __name__ == "__main__":
    run_uvicorn()
