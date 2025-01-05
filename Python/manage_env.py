import os
import subprocess
import sys

"""
This module encapsulates the management logic of the python virtual environment
necessary for the proper functioning of the project wAIves.
"""

def exist_env(name_virtual_env:str):
    """Verify if the virtual environment ```name_virtual_env``` exist in the current directory."""
    env_path = os.path.join(os.getcwd(), name_virtual_env, 'bin', 'activate')
    return os.path.exists(env_path)

def create_env(name_virtual_env:str):
    """Create a new virtual environment in the current directory."""
    try:
        # Create a new virtual environment using the venv module
        subprocess.run([sys.executable, '-m', 'virtualenv', name_virtual_env], check=True)
        print(f"Virtual environment '{name_virtual_env}' successfully created.")
    except subprocess.CalledProcessError as e:
        print(f"ERROR: Virtual environment '{name_virtual_env}' can't be created.")
        print(f"Details: {e}")
    except Exception as e:
        print(f"An unexpected error occurred: {e}")

def run_in_env(name_virtual_env:str, command:str):
    """
    name_virtual_env (str): The name or path of the virtual environment.
    command (str): The command to execute in the virtual environment.
    """
    activate_script = os.path.join(name_virtual_env, 'bin', 'activate')
    if not os.path.exists(activate_script):
        return f"Error: Virtual environment '{name_virtual_env}' not found."

    try:
        full_command = f"source {activate_script} && {command}"
        result = subprocess.run(full_command, shell=True, check=True, text=True, capture_output=True, executable='/bin/bash')
        return result.stdout
    except subprocess.CalledProcessError as e:
        return f"Error: {e.stderr}"

def install_dependencies(name_virtual_env:str):
    """Installing the necessary python dependencies."""
    dependencies = [
        ("flask","3.0.3"), ("flask_cors","5.0.0"), ("tensorflow","2.15.1"), ("torch","2.4.1"), ("gunicorn", "20.1.0")
    ]
    # path of the Python interpreter of the virtual environment
    python_executable = os.path.join(name_virtual_env, 'bin', 'python')

    # Create a command to automate the installations
    command = ""
    for module in dependencies:
        command += f" {python_executable} -m pip install {module[0]}=={module[1]} &&"
    command = command[:-2]
    print(f"Command executed : {command}\n")

    # Running installation commands in the virtual environment
    state = run_in_env(name_virtual_env, command)
    print(f"State of installation of dependencies :\n\n{state}")

def start_server(name_virtual_env:str):
    """
    Start the server in the virtual environnement.
    """

    # Command to start the server
    command = "gunicorn -w 4 -b 0.0.0.0:5000 server:app"
    state = run_in_env(name_virtual_env, command)
    print(state)


if __name__ == '__main__' :
    ENV_NAME = "wAIvesENV"
    
    if exist_env(ENV_NAME) == False :
        create_env(ENV_NAME)

    install_dependencies(ENV_NAME)
    start_server(ENV_NAME)