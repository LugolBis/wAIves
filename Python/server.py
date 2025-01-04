from flask import Flask, request, jsonify
from flask_cors import CORS
import use_modelTensorflow
import use_modelPytorch
import weather
import numpy as np
import os

def use_model(model_name="wAIves1v5.1.h5", input_data=np.array):
    """The first arg needs to be a `str` and the second a `numpy.array`."""
    if "1v" in model_name:
        return use_modelTensorflow.load_and_predict(model_name, input_data)
    else:
        return use_modelPytorch.load_and_predict(model_name, input_data)

app = Flask(__name__)

CORS(app)

@app.route('/main', methods=['POST'])
def run_main():
    data = request.json
    input_user = data.get('input_string', '')  
    location_ = weather.get_location(input_user)
    if location_ is None:
        return jsonify({"result": "The format of your location is invalid or unavailable."}), 400
    weather_ = weather.get_current_weather(location_)
    if weather_ is None:
        return jsonify({"result": "The connection with the API to retrieve the weather did not work"}), 500
    input_ = weather.transform_weather_data(weather_)
    result = use_model(input_data=input_)
    return jsonify({"result": result})

@app.route('/playground', methods=['POST'])
def run_playground():
    data = request.json
    input_user = data.get('input_string', '')
    if input_user == "null":
        return jsonify({"result": "Invalid data."}), 400
    try:
        input_ = input_user.split(",")
        input__ = [float(nb) for nb in input_[:-1]]
        model_name = input_[-1]
        result = use_model(model_name, input__)
        return jsonify({"result": result})
    except ValueError:
        return jsonify({"result": "Invalid input format. Check your data."}), 400

if __name__ == '__main__':
    host = os.getenv("FLASK_HOST", "0.0.0.0")
    port = int(os.getenv("FLASK_PORT", 5000))
    debug = os.getenv("FLASK_DEBUG", "False").lower() == "true"

    app.run(host=host, port=port, debug=debug)

    # Command bash to start the server : gunicorn -w 4 -b 0.0.0.0:5000 server:app

"""
Dependencies :
Python : 3.10.12
Flask : 3.0.3
Flask_cors : 5.0.0
Tensorflow : 2.15.1
Numpy : 1.26.4
Torch : 2.4.1
"""