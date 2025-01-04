# Use an Pytorch Model

import torch
import numpy as np

class SimpleModel(torch.nn.Module):
    def __init__(self,input_size):
        super(SimpleModel, self).__init__()
        self.fc1 = torch.nn.Linear(input_size, 128)
        self.fc2 = torch.nn.Linear(128, 64)
        self.fc3 = torch.nn.Linear(64, 1)
    
    def forward(self, x):
        x = torch.relu(self.fc1(x))
        x = torch.relu(self.fc2(x))
        x = self.fc3(x)
        return x

def load_and_predict(model_name, input_data):
    try:
        model_path = "../Models/" + model_name
        input_size = 6
        model = SimpleModel(input_size)
        model.load_state_dict(torch.load(model_path,weights_only=True))
        
        model.eval()
        
        input_data = np.expand_dims(input_data, axis=0)
        input_tensor = torch.tensor(input_data, dtype=torch.float32)
        
        with torch.no_grad():
            output = model(input_tensor)
        
        return str(round(abs(output.numpy()[0][0]),2))
    except Exception as error:
        print(error) ; return None

if __name__ == "__main__" :
    input_data = np.array([0.5, 1.2, -0.3, 0.8, 0.1, -0.5])  # Exemples de données
    prediction = load_and_predict("wAIves2v1.0.pth", input_data)
    print("Predicted output:", prediction) ; print(type(prediction))

"""
Requierment :
Python : v3.10.12
Torch : 2.4.1+cu118
"""
