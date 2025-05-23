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
        
        dataset_np = np.array(dataset, dtype=np.float32)
        if len(dataset_np) % 7 != 0:
            raise ValueError("The dataset need to be a multiple of 7.")
        dataset_np = dataset_np.reshape(-1, 7)

        inputs = dataset_np[:, 2:6]   
        outputs = dataset_np[:, 6:]

        print("\nSuccessfully format the data.\n")
        return inputs, outputs
    except Exception as e:
        print(f"\nERROR : {e}\n")
        return None, None

def training_model(data_path: str, model_path: str):
    data_inputs, data_outputs = formatage_data(data_path)
    if data_inputs is None:
        return

    model = tf.keras.Sequential([
        tf.keras.layers.Dense(128, activation='relu', input_shape=(4,)),
        tf.keras.layers.Dense(64, activation='relu'),
        tf.keras.layers.Dense(1)
    ])

    model.compile(optimizer='adam', loss='mean_squared_error')
    
    # Data pipeline
    dataset = tf.data.Dataset.from_tensor_slices((data_inputs, data_outputs))
    dataset = dataset.batch(32).prefetch(tf.data.AUTOTUNE)
    
    model.fit(
        dataset,
        epochs=50,
        callbacks=[
            tf.keras.callbacks.EarlyStopping(monitor="loss", patience=3)
        ]
    )
    
    model.save(model_path)
    print("\nSuccessfully train the model.\n")

if __name__ == '__main__':
    args = sys.argv

    if len(args) == 3:
        training_model(args[1], args[2])
    else:
        print("\nUsage : training_model.py path/to/dataset.json path/to/model.h5")

"""
Requierment :
python v3.10.12
tensorflow v2.15.1
"""
