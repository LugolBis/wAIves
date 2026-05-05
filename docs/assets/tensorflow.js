// tensorflow.js
// This script need to be in the /public folder of the 

async function predictWithModel(inputArray, url, truncate) {
    try {
        const model = await tf.loadLayersModel(url);
        console.log("Input array:", inputArray);
        console.log(`Successfully load the model : ${url}`);

        if (truncate) {
            inputArray.splice(0,2);
        }

        const inputTensor = tf.tensor2d([inputArray]);
        const prediction = model.predict(inputTensor);

        const predictionValue = await prediction.data();
        let hauteurM = Math.round(predictionValue[0] * 10) / 10;
        hauteurM = Math.abs(hauteurM);
        console.log("Prédiction : ", hauteurM);
        return hauteurM;
    } catch (error) {
        console.error("Error in model prediction: ", error); 
        return null;
    }
}

window.tfjsHelpers = {
    predict: predictWithModel
};

console.log("Successfully load the custom Tensorflow script !")