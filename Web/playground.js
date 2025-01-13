// Playground 

function transformWeatherDataPlayground() { 
    const inputLatitude = document.getElementById("inputLatitude").value ;
    const inputLongitude = document.getElementById('inputLongitude').value ;
    const inputWindSpeed = document.getElementById("inputWindSpeed").value ;
    const inputWindDirection = document.getElementById("inputWindDirection").value ;
    const inputTemperature = document.getElementById("inputTemperature").value ;
    const inputPressure = document.getElementById("inputPressure").value ;

    const result = [inputLatitude,inputLongitude,inputTemperature,
        inputPressure,inputWindSpeed,inputWindDirection].map(parseFloat).filter(Number.isFinite) ;
    
    if (result.length===6) {
        return result ;
    }
    else {
        document.getElementById('resultatPlayground').innerHTML = "Invalid inputs, please fill in all fields." ;
        return null ;
    }
}

async function runPlayground() {
    const modelName = document.getElementById("modelPlayground").value ;
    const result = document.getElementById('resultatPlayground') ;

    const inputArray = transformWeatherDataPlayground() ;
    if (inputArray===null){
        return;
    }
    const response__ = await predictWithModel(inputArray,modelName) ;
    result.innerHTML = `${response__} meters`;
}