"""
Training the model
"""

import sys
import json
import numpy as np
import tensorflow as tf

def formatage_data(file_path: str):
    try:
        with open(file_path, 'r') as fs:
            dataset = json.load(fs)
        inputs = []
        outputs = []

        for index in range(0,len(dataset),7):
            temperature, pressure, winds, windd, waves = dataset[index+2], dataset[index+3], dataset[index+4], dataset[index+5], dataset[index+6]
            inputs.append(np.array([temperature, pressure, winds, windd], dtype=np.float32))
            outputs.append(np.array([waves], dtype=np.float32))

        inputs = np.array(inputs, dtype=np.float32)
        outputs = np.array(outputs, dtype=np.float32)
        print("Successfully format the data")
        return inputs, outputs
    except Exception as e:
        print(f"ERROR - inconsistent format : {e}")
        return None, None

def training_model(data_path: str, model_path: str):
    data_inputs, data_outputs = formatage_data(data_path)
    if data_inputs is None or data_outputs is None:
        print("ERROR when try to format the data")
        return

    model = tf.keras.Sequential([
        tf.keras.layers.Dense(128, activation='relu', input_shape=(4,)), 
        tf.keras.layers.Dense(64, activation='relu'),
        tf.keras.layers.Dense(1)
    ])

    model.compile(optimizer='adam', loss='mean_squared_error')

    model.fit(data_inputs, data_outputs, epochs=50)

    model.save(model_path)
    print("\nSuccessfully train the model.\n")

if __name__ == '__main__':
    args = sys.argv

    if len(args) == 3:
        training_model(args[1], args[2])
    else:
        print("Usage : [training_model.py] dataset.json model.h5")

"""
Requierment :
python v3.10.12
tensorflow v2.15.1
"""
