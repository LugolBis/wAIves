# Entrainement du model

import json
import numpy as np
import tensorflow as tf

def formatage_data(file_path: str):
    try:
        with open(file_path, 'r') as fs:
            data = json.load(fs)
        inputs = []
        outputs = []

        for station_name, dates in data.items():
            for date, values in dates.items():
                if isinstance(values["input"], list) and len(values["input"]) == 6: # Modifier ce paramètre selon le nombre d'inputs
                    inputs.append(np.array(values["input"], dtype=np.float32))
                else:
                    print(f"Invalid input format for {station_name} at {date} ---- {values}")

                if isinstance(values["output"], list) and len(values["output"]) == 1:
                    outputs.append(np.array(values["output"], dtype=np.float32))
                else:
                    print(f"Invalid output format for {station_name} at {date}")

        inputs = np.array(inputs, dtype=np.float32)
        outputs = np.array(outputs, dtype=np.float32)
        print("Formatage des données réussi.")
        return inputs, outputs
    except Exception as e:
        print(f"Error --- Le formatage n'a pas abouti : {e}")
        return None, None

def training_model(data_path: str, model_path: str):
    data_inputs, data_outputs = formatage_data(data_path)
    if data_inputs is None or data_outputs is None:
        print("Les données n'ont pas pu être formatées correctement.")
        return

    model = tf.keras.Sequential([
        #tf.keras.layers.Dense(64, activation='relu', input_shape=(data_inputs.shape[1],)),
        tf.keras.layers.Dense(64, activation='relu', input_shape=(6,)), 
        tf.keras.layers.Dense(32, activation='relu'),
        tf.keras.layers.Dense(1)
    ])

    model.compile(optimizer='adam', loss='mean_squared_error')

    model.fit(data_inputs, data_outputs, epochs=50)

    model.save(model_path)
    print("\nModèle entraîné et sauvegardé avec succès.\n")

training_model(r"C:\Users\lddes\OneDrive\Bureau\Discovery\wAIves\python\data_test.json", r"C:\Users\lddes\OneDrive\Bureau\Discovery\wAIves\python\ModelTEST.h5")

"""
Requierment :
python v3.10.12
tensorflow v2.15.1
"""
