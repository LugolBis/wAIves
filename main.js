// Main 

async function getLocation() {
    const country = countryInput.value;
    const city = document.getElementById("cityInput").value;
    return `${city},${countriesAbbreviation[country]}`;
}

function transformWeatherData(data) { 
    const array_ = []
    array_.push(data['coord']['lon']) ; array_.push(data['coord']['lat']) ;
    array_.push(data["main"]["temp"]-273.15) ; array_.push(data["main"]["pressure"]) ;
    array_.push(data["wind"]["speed"]) ; array_.push(data["wind"]["deg"]) ;
    console.log(`Données : ${array_}`) ;
    return array_
}

async function getWeatherData(location) {
    const url = `https://api.openweathermap.org/data/2.5/weather?q=${encodeURIComponent(location)}&appid=0a216b54f594e070778d7d8b8390ac06`;
    
    try {
        const response = await fetch(url); // Utilisation de fetch pour la requête
        if (!response.ok) {
            const response_test = await fetch(`http://api.openweathermap.org/data/2.5/weather?q=${"Sagres,PT"}&appid=0a216b54f594e070778d7d8b8390ac06`);
            if (!response_test.ok) {
                alert("wAIves fails to connect to server hosting weather data.\nPlease try again later.") ;
            }
            else {
                alert(`Weather data for your chosen location is not available.\nHowever, you can use the Playground space below to manually enter weather data.`);
            }
            
            return null;
        }
        const data = await response.json(); console.log(data);
        return data;
    } catch (error) {
        console.error("Error fetching weather data :", error);
        return null;
    }
}

async function predictWithModel(inputArray, modelName) {
    try {

        let modelSelected = null;
        switch (modelName) {
            case "wAIves1v5.0": modelSelected = wAIves1v5_0;
            case "wAIves1v5.1": modelSelected = wAIves1v5_1;
            case "wAIves1v5.2": modelSelected = wAIves1v5_2;
            default: console.log(`${modelName}`);
        }

        const modelArtifact = {
            modelTopology: modelSelected.modelTopology,
            weightData: modelSelected.weightData
        };
        const model = await tf.loadLayersModel(tf.io.fromMemory(modelArtifact));
        console.log(inputArray) ;console.log("Le modèle à été chargé !") ;
        // Convert the input array to a tensor
        const inputTensor = tf.tensor2d([inputArray]);

        // Make a prediction
        const prediction = model.predict(inputTensor);

        // Get the prediction value from the tensor
        const predictionValue = await prediction.data();
        let hauteurM = Math.round(predictionValue[0] * 10) / 10 ;
        hauteurM = Math.abs(hauteurM) ;
        console.log("Prédiction : ", hauteurM);
        return hauteurM;
    } catch (error) {
        console.error("Error in model prediction: ", error); 
        return null;
    }
}

async function run() {
    const modelName = document.getElementById('modelMain').value ;
    const resultat = document.getElementById('resultat') ;
    resultat.innerHTML = "Processing...";

    const location = await getLocation();
    const weather = await getWeatherData(location);
    if (weather===null) {
        resultat.innerHTML = `Error when trying to request weather data to OpenWeatherMap.`;
        return null;
    }

    const inputArray = transformWeatherData(weather);
    const response = await predictWithModel(inputArray,modelName);
    if (response===null) {
        resultat.innerHTML = `Error when trying to use the model. Try with an other model or again later.`;
        return null;
    }
    resultat.innerHTML = `${response} m`;
}