# Use an tensorflow model

import tensorflow as tf
import numpy as np

def load_and_predict(model_name='wAIves1v5.1.h5', input_data=np.array) :
    """The first arg need to be a ```str``` and the second need to be a ```numpy.array```."""
    try:
        # Charger le modèle
        model_path = "../Models/" + model_name
        model = tf.keras.models.load_model(model_path)

        input_data = np.expand_dims(input_data, axis=0)
        # Effectuer la prédiction
        predictions = model.predict(input_data)
        return str(round(abs(predictions[0][0]),2))
    except Exception as error:
        print(error) ; return None