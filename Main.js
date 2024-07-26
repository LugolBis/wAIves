// Main 

function getLocation(cityCountry) {
    // Dictionnaire des abréviations de pays
    const countryAbbr = {
        "Portugal": "PT", "France": "FR",
        "Espagne": "ES", "Spain": "ES",
        "Grande-Bretagne": "GB", "Angleterre": "GB", "England": "GB",
        "Monaco": "MC", "Senegal": "SN", "Canada": "CA",
        "Italy": "IT", "Italie": "IT",
        "Egypt": "EG", "Egypte": "EG",
        "Ireland": "IE", "Irlande": "IE",
        "Morocco": "MA", "Maroc": "MA",
        "Algeria": "DZ", "Algérie": "DZ",
        "Tunisia": "TN", "Tunisie": "TN",
        "Libya": "LY", "Libye": "LY",
        "Mauritania": "MR", "Mauritanie": "MR",
        "Etats-Unis": "US", "USA": "US", "United-States": "US",
        "Bermude": "BM", "Nicaragua": "NI",
        "Mexico": "MX", "Mexique": "MX",
        "Costa-Rica": "CR", "Costa Rica": "CR",
        "Panama": "PA", "Cuba": "CU", "Paraguay": "PY",
        "Columbia": "CO", "Colomnbie": "CO", "Uruguay": "UY",
        "Bahamas": "BS", "The-Bahamas": "BS", "The Bahamas": "BS", "Les Bahamas": "BS",
        "Domican Republic": "DO", "Dominica": "DM", "République Dominicaine": "DM",
        "Jamaica": "JM", "Jamaïque": "JM",
        "Honduras": "HN", "Guatemala": "GT", "El Salvador": "SV",
        "Guyana": "GY", "Suriname": "SR",
        "Brazil": "BR", "Brasil": "BR", "Brésil": "BR",
        "Peru": "PE", "Pérou": "PE",
        "Argentina": "AR", "Argentine": "AR"
    };

    // Expression régulière pour extraire la ville et le pays
    const regex = /^\s*([\w\s]+)\s*,\s*([\w\s]+)\s*$/;
    const match = cityCountry.match(regex);

    if (!match) {
        return "Format incorrect. Veuillez vérifier l'entrée.";
    }

    const city = match[1].trim();
    const country = match[2].trim();

    // Obtenir l'abréviation du pays
    const countryCode = countryAbbr[country];

    if (!countryCode) {
        return "Abréviation du pays introuvable.";
    }

    return `${city},${countryCode}`;
}

function getWeatherData(api_key, location, callback) {
    const url = `http://api.openweathermap.org/data/2.5/weather?q=${encodeURIComponent(location)}&appid=${api_key}`;
    const xhr = new XMLHttpRequest();

    xhr.open('GET', url, true);

    xhr.onreadystatechange = function() {
        if (xhr.readyState === 4) { // Request completed
            if (xhr.status === 200) {
                const data = JSON.parse(xhr.responseText);
                callback(null, data);
            } else {
                callback("Erreur lors de la récupération des données.", null);
            }
        }
    };

    xhr.send();
}

function transformWeatherData(data) { 
    const array_ = []
    array_.push(data['coord']['lon']) ; array_.push(data['coord']['lat']) ;
    array_.push(data["main"]["temp"]-273.15) ; array_.push(data["main"]["pressure"]) ;
    array_.push(data["wind"]["speed"]) ; array_.push(data["wind"]["deg"]) ;
    console.log(`Données : ${array_}`) ;
    return array_
}

async function formatData(filePath, nb_inputs) {
    try {
        const response = await fetch(filePath);
        const data = await response.json();

        let inputs = [];
        let outputs = [];

        for (let station_name in data) {
            if (data.hasOwnProperty(station_name)) {
                let dates = data[station_name];
                for (let date in dates) {
                    if (dates.hasOwnProperty(date)) {
                        let values = dates[date];
                        if (Array.isArray(values.input) && values.input.length === nb_inputs) {
                            inputs.push(new Float32Array(values.input));
                        } else {
                            console.log(`Invalid input format for ${station_name} at ${date} ---- ${values}`);
                        }
                        if (Array.isArray(values.output) && values.output.length === 1) {
                            outputs.push(new Float32Array(values.output));
                        } else {
                            console.log(`Invalid output format for ${station_name} at ${date}`);
                        }
                    }
                }
            }
        }
        inputs = tf.tensor2d(inputs, [inputs.length / nb_inputs, nb_inputs]);
        outputs = tf.tensor2d(outputs, [outputs.length, 1]);
        console.log("Formatage des données réussi.");
        return [inputs, outputs];
    } catch (e) {
        console.error(`Error --- Le formatage n'a pas abouti : ${e}`);
        return [null, null];
    }
}

async function trainModel(data_path, path_out, nb_inputs) {
    const [inputs, outputs] = await formatData(data_path, nb_inputs);
    
    if (inputs === null || outputs === null) {
        console.log("Les données n'ont pas pu être formatées correctement.");
        return;
    }

    const model = tf.sequential();
    model.add(tf.layers.dense({inputShape: [nb_inputs], units: 10, activation: 'relu'}));
    model.add(tf.layers.dense({units: 5, activation: 'relu'}));
    model.add(tf.layers.dense({units: 1, activation: 'linear'}));

    model.compile({
        optimizer: tf.train.adam(),
        loss: 'meanSquaredError',
        metrics: ['mse']
    });

    await model.fit(inputs, outputs, {
        epochs: 100,
        batchSize: 32,
        validationSplit: 0.2,
        callbacks: tf.callbacks.earlyStopping({monitor: 'val_loss'})
    });

    await model.save(path_out);
    console.log("Modèle entraîné et sauvegardé avec succès.");
}

async function predictWithModel(inputArray) {
    try {
        // Load the model
        const model = await tf.loadLayersModel("C:\Users\lddes\OneDrive\Bureau\Discovery\wAIves\Model2_wAIves.json");

        // Convert the input array to a tensor
        const inputTensor = tf.tensor2d([inputArray]);

        // Make a prediction
        const prediction = model.predict(inputTensor);

        // Get the prediction value from the tensor
        const predictionValue = prediction.dataSync();

        console.log("Prediction: ", predictionValue);
        return predictionValue;
    } catch (error) {
        console.error("Error in model prediction: ", error); 
        return error ;
    }
}

// Exemple d'utilisation
const apiKey = '0a216b54f594e070778d7d8b8390ac06';
const location_ = getLocation("Sagres, Portugal");

if (!location_.startsWith("Format incorrect") && !location_.startsWith("Abréviation du pays introuvable")) {
    getWeatherData(apiKey, location_, (error, data) => {
        if (error) {
            console.log(error);
        } else {
            console.log(data); transformWeatherData(data) ;
        }
    });
} else {
    console.log(location_);
}    

trainModel("C:\Users\lddes\OneDrive\Bureau\Discovery\wAIves\DATA\DATA_1.json","C:\Users\lddes\OneDrive\Bureau\Discovery\wAIves",6) ;
trainModel("C:\Users\lddes\OneDrive\Bureau\Discovery\wAIves\DATA\DATA_2.json","C:\Users\lddes\OneDrive\Bureau\Discovery\wAIves",5) ;