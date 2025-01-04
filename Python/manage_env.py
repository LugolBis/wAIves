import os
import subprocess
import sys

"""
This module encapsulates the management logic of the python virtual environment
necessary for the proper functioning of the project wAIves.
"""

def exist_env(name_virtual_env):
    """Verify if the virtual environment *```name_virtual_env```* exist in the current directory."""
    env_path = os.path.join(os.getcwd(), name_virtual_env, 'bin', 'activate')
    return os.path.exists(env_path)

def connect_env(name_virtual_env):
    """Connect to the virtual environment called *```name_virtual_env```*."""
    command = f"bash -i -c 'source {name_virtual_env}/bin/activate'"
    command2 = f"source {name_virtual_env}/bin/activate"
    try:
        result = subprocess.run(command2)
        #result = subprocess.run(command, shell=True, check=True, text=True, capture_output=True)
        return result.stdout
    except subprocess.CalledProcessError as e:
        return f"Error: {e.stderr}"

def create_env(name):
    """Create a new virtual environment in the current directory."""
    try:
        # Create a new virtual environment
        subprocess.run([sys.executable, '-m', 'virtualenv ', name])
        print(f"Virtual environment '{name}' successfully created.")
    except:
        print(f"ERROR : Virtual environment '{name}' can't be created.")

def install_dependencies():
    """Installing the necessary python dependencies."""
    dependencies = [
        ("flask","3.0.3"), ("flask_cors","5.0.0"), ("tensorflow","2.15.1")
    ]

    for module in dependencies:
        try:
            subprocess.run([sys.executable, '-m', 'pip', 'install', module[0]+'=='+module[1]])
            print(f"Module '{module[0]}' successfully installed.")
        except:
            print(f"ERROR : module '{module[0]}' wasn't installed.")