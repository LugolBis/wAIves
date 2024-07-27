# Convert model

import tensorflow as tf
import tf2onnx
import subprocess
import os

def convert_to_ONNX():
    # Chemin du modèle sauvegardé en .keras
    keras_model_path = r"C:\Users\lddes\OneDrive\Bureau\Discovery\wAIves\Model1_wAIves.keras"

    # Charger le modèle Keras
    model = tf.keras.models.load_model(keras_model_path)

    # Chemin où le modèle ONNX sera sauvegardé
    onnx_model_path = r"C:\Users\lddes\OneDrive\Bureau\Discovery\wAIves\Model1_wAIves.onnx"

    # Convertir le modèle Keras en modèle ONNX
    spec = (tf.TensorSpec((None, 6), tf.float32, name="input"),)
    model_proto, _ = tf2onnx.convert.from_keras(model, input_signature=spec, opset=13)

    # Sauvegarder le modèle ONNX
    with open(onnx_model_path, "wb") as f:
        f.write(model_proto.SerializeToString())

    print(f"Modèle converti et sauvegardé avec succès à {onnx_model_path}")



def convert_h5_to_tfjs(h5_model_path, tfjs_output_dir):
    """
    Convertit un modèle Keras sauvegardé au format .h5 en un format compatible avec TensorFlow.js.

    :param h5_model_path: Chemin vers le fichier du modèle Keras .h5.
    :param tfjs_output_dir: Chemin vers le répertoire de sortie pour le modèle TensorFlow.js.
    """
    try:
        # Vérifier si le répertoire de sortie existe, sinon le créer
        if not os.path.exists(tfjs_output_dir):
            os.makedirs(tfjs_output_dir)
        
        # Commande de conversion
        conversion_command = [
            'tensorflowjs_converter',
            '--input_format=keras',
            h5_model_path,
            tfjs_output_dir
        ]

        # Exécuter la commande de conversion
        result = subprocess.run(conversion_command, capture_output=True, text=True)

        # Vérifier si la conversion a réussi
        if result.returncode == 0:
            print(f"Conversion réussie ! Le modèle TensorFlow.js a été sauvegardé dans : {tfjs_output_dir}")
        else:
            print(f"Erreur lors de la conversion : {result.stderr}")

    except Exception as e:
        print(f"Exception lors de la conversion : {str(e)}")

# Exemple d'utilisation
path_in = r"C:\Users\lddes\OneDrive\Bureau\Discovery\wAIves\Model1_wAIves_v2_8_0.h5"
path_out = r"C:\Users\lddes\OneDrive\Bureau\Discovery\wAIves"
#convert_h5_to_tfjs(path_in, path_out)
convert_to_ONNX()