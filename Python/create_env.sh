#!/bin/bash

# Install the module virtualenv
python3 -m pip install virtualenv

# Create the virtual environment
python3 -m virtualenv TPenv

# Connect to the virtual environment
source TPenv/bin/activate

# Install dependencies
python3 -m pip install flask==3.0.3
python3 -m pip install flask_cors==5.0.0
python3 -m pip install tensorflow==2.15.1
python3 -m pip install torch==2.4.1

# chmod +x create_env.sh