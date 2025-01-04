# Training model with Pytorch

import numpy as np
import json
import torch

def generatorData(files_path):
    """Générateur qui parcourt les fichiers JSON."""
    for path in files_path:
        with open(path, 'r') as file:
            data = json.load(file)
        for liste in data:
            yield np.array(liste[:-1], dtype=np.float32), np.array(liste[-1], dtype=np.float32)

class CustomDataset(torch.utils.data.Dataset):
    """Dataset personnalisé"""
    def __init__(self, files_path):
        self.data = list(generatorData(files_path))
    
    def __len__(self):
        return len(self.data)
    
    def __getitem__(self, idx):
        return self.data[idx]

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

def trainModel(model_path, data_path, batch_size=32, epochs=50):
    
    input_size = 6
    learning_rate = 0.001

    dataset = CustomDataset(data_path) 
    dataloader = torch.utils.data.DataLoader(dataset, batch_size=batch_size, shuffle=True)

    device = torch.device("cuda" if torch.cuda.is_available() else "cpu")
    model = SimpleModel(input_size).to(device)
    criterion = torch.nn.MSELoss()  # Utilisation d'une perte MSE pour la régression
    optimizer = torch.optim.Adam(model.parameters(), lr=learning_rate)
    
    model.train()

    for epoch in range(epochs):
        running_loss = 0.0
        for inputs, target in dataloader:
            inputs = inputs.to(device)
            target = target.to(device)

            outputs = model(inputs).squeeze(1)
            loss = criterion(outputs, target)

            optimizer.zero_grad()
            loss.backward()
            optimizer.step()

            running_loss += loss.item()
        
        print(f'Epoch [{epoch+1}/{epochs}], Loss: {running_loss/len(dataloader):.4f}')
    
    torch.save(model.state_dict(), model_path) ; print("Model sauvegardé avec succès !")

def evaluateModel(model_path, data_path):

    dataset = CustomDataset([data_path])
    dataloader = torch.utils.data.DataLoader(dataset, batch_size=1, shuffle=False)

    input_size = 6  
    device = torch.device("cuda" if torch.cuda.is_available() else "cpu")

    model = SimpleModel(input_size)
    model.load_state_dict(torch.load(model_path))
    model.to(device)
    model.eval()

    criterion = torch.nn.MSELoss()
    total_loss = 0.0
    with torch.no_grad():
        for inputs, target in dataloader:
            inputs = inputs.to(device)
            target = target.to(device)

            outputs = model(inputs).squeeze(1)
            loss = criterion(outputs, target)
            total_loss += loss.item()
    print(f"Évaluation terminée. Perte totale : {total_loss/len(dataloader):.4f}")

def convertModel(model_path, modelConverted_path):
    """This function convert the .pth file model to a .onnx file.
    \nmodel_path need to be a .pth file."""
    input_size=(1, 6)
    model = SimpleModel(input_size[1])

    model.load_state_dict(torch.load(model_path))   # Charger les états des paramètres (state_dict)

    scripted_model = torch.jit.script(model)
    
    scripted_model.save(modelConverted_path)
    print(f"Model converti et sauvegardé au format web :\n{modelConverted_path}")

def export_to_onnx(model, dummy_input, output_path="model.onnx"):
    # Exporter le modèle PyTorch vers ONNX
    input_size = 6  
    device = torch.device("cuda" if torch.cuda.is_available() else "cpu")

    model = SimpleModel(input_size)
    model.load_state_dict(torch.load(model_path))
    model.to(device)

    torch.onnx.export(
        model,                         # Modèle PyTorch
        dummy_input,                   # Exemple de donnée d'entrée
        output_path,                   # Fichier ONNX
        export_params=True,            # Exporter les paramètres
        opset_version=11,              # Version ONNX
        do_constant_folding=True,      # Optimisation
        input_names=['input'],         # Nom de l'entrée
        output_names=['output'],       # Nom de la sortie
        dynamic_axes={'input': {0: 'batch_size'}, 'output': {0: 'batch_size'}} # Prise en charge des batchs dynamiques
    )
    print(f"Modèle exporté vers {output_path}")