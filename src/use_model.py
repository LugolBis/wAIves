import sys
import tensorflow as tf
import numpy as np

def load_and_predict(model_path, input_data=np.array) :
    """The first arg need to be a ```str``` and the second need to be a ```numpy.array```."""
    try:
        # Load the model
        model = tf.keras.models.load_model(model_path)

        input_data = np.expand_dims(input_data, axis=0)
        # Predict
        predictions = model.predict(input_data)
        return round(abs(predictions[0][0]),2)
    except Exception as error:
        print(error) ; return None
    
if __name__ == '__main__':
    args = sys.argv

    if len(args) == 6:
        model_path = args[1]
        temperature, pressure, winds, windd = (float(nb) for nb in [args[2], args[3], args[4], args[4]])
        inputs = np.array([temperature, pressure, winds, windd], dtype=np.float32)

        print(load_and_predict(model_path, inputs))
    else:
        print("Incosistent data.")