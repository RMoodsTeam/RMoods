import subprocess
import sys


def run_uvicorn():
    port = '8002'

    # Command as a list of arguments
    command = [
        ".venv/bin/uvicorn",
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
