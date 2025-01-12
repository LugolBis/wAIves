// Main

function showLoading(button, message) {
    const originalText = button.innerHTML;
    button.disabled = true;
    button.innerHTML = message || "Loading...";
    setTimeout(() => {
      button.disabled = false;
      button.innerHTML = originalText;
    }, 3000); // Réinitialise après 3 secondes
  }

async function run() {
    const button = document.getElementById("getPrediction");
    showLoading(button, "Fetching Prediction...");

    const inputString = document.getElementById('input').value;  // Récupérer la valeur de l'input

    try {
        const response = await fetch('http://127.0.0.1:5000/main', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({ input_string: inputString })  // Envoyer l'input sous forme de JSON
        });

        const result = await response.json();  // Attendre la réponse en JSON
        document.getElementById('resultat').innerText = result.result + " m";  // Afficher la réponse
    } catch (error) {
        console.error('Erreur:', error);
    }
}

function transformWeatherDataPlayground() {
    const button = document.getElementById("getPredictionPlayground");
    showLoading(button, "Running...");

    const inputLatitude = document.getElementById("inputLatitude").value ;
    const inputLongitude = document.getElementById('inputLongitude').value ;
    const inputWindSpeed = document.getElementById("inputWindSpeed").value ;
    const inputWindDirection = document.getElementById("inputWindDirection").value ;
    const inputTemperature = document.getElementById("inputTemperature").value ;
    const inputPressure = document.getElementById("inputPressure").value ;
    const array_ = [inputLatitude,inputLongitude,inputTemperature,
        inputPressure,inputWindSpeed,inputWindDirection].map(parseFloat).filter(Number.isFinite) ;

    if (array_.length===6) {
        return array_ ;
    }
    else {
        document.getElementById('resultatPlayground').innerHTML = "Invalid inputs, please fill in all fields." ;
        return null ;
    }
}

async function runPlayground() {
    const modelName = document.getElementById("model_").value ;
    const inputString = String(transformWeatherDataPlayground());  // Récupérer la valeur de l'input

    try {
        const response = await fetch('http://127.0.0.1:5000/playground', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({ input_string: inputString + ',' + modelName })  // Envoyer l'input sous forme de JSON
        });

        const result = await response.json();  // Attendre la réponse en JSON
        document.getElementById('resultatPlayground').innerText = result.result + " m";  // Afficher la réponse
    } catch (error) {
        console.error('Erreur:', error);
    }
}