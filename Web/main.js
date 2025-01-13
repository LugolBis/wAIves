// Main 

async function getLocation(cityCountry) {
    try {
        // Expression régulière pour extraire la ville et le pays
        const regex = /^\s*([\w\s'-]+)\s*,\s*([\w\s'-]+)\s*$/;
        const match = cityCountry.match(regex);
        const resultat = document.getElementById('resultat');

        if (!match) {
            resultat.innerHTML = "Incorrect format. Please check the entry.";
            return "Incorrect format. Please check the entry.";
        }

        const city = match[1].trim();
        const country = match[2].trim();

        // Obtenir l'abréviation du pays
        const countryCode = countriesAbbreviation[country];

        if (!countryCode) {
            resultat.innerHTML = "Country abbreviation not found. Please check the country entered.";
            return "Abréviation du pays introuvable.";
        }

        return `${city},${countryCode}`;
    }
    catch (error) {
        console.log(`Error -- getLocation() : ${error}`) ; return `${error}` ;
    }
}

function transformWeatherData(data) { 
    const array_ = []
    array_.push(data['coord']['lon']) ; array_.push(data['coord']['lat']) ;
    array_.push(data["main"]["temp"]-273.15) ; array_.push(data["main"]["pressure"]) ;
    array_.push(data["wind"]["speed"]) ; array_.push(data["wind"]["deg"]) ;
    console.log(`Données : ${array_}`) ;
    return array_
}

async function getWeatherData(api_key, location) {
    const url = `http://api.openweathermap.org/data/2.5/weather?q=${encodeURIComponent(location)}&appid=${api_key}`;
    
    try {
        const response = await fetch(url); // Utilisation de fetch pour la requête
        if (!response.ok) {
            const response_test = await fetch(`http://api.openweathermap.org/data/2.5/weather?q=${"Sagres,PT"}&appid=${api_key}`);
            if (!response_test.ok) {
                alert("wAIves fails to connect to server hosting weather data.\nPlease try again later.") ;
            }
            else {
                alert("Weather data for your chosen location is not available.\nHowever, you can use the Playground space to manually enter weather data.") ;
            }
            
            throw new Error("Erreur lors de la récupération des données.");
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
        return error ;
    }
}

async function run() {
    const modelName = document.getElementById('modelMain').value ;
    const inputValue = document.getElementById('input').value ;
    const resultat = document.getElementById('resultat') ;
    const apiKey = 'My api key';

    const location = await getLocation(inputValue);
    const weather = await getWeatherData(apiKey,location) ;

    const inputArray = transformWeatherData(weather) ;
    const response = await predictWithModel(inputArray,modelName) ;
    resultat.innerHTML = `${response} m`;
}