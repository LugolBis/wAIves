# wAIves 🏄

## Project Objective :
The objective of this **personal** project is to develop **AI** models that predicting Surf conditions based on weather data.  
This involves Data Mining, Supervised Learning, and Web Application development.

## Key Project Steps :
- **Data Mining** : Text Scraping, Data Analysis, and Formatting to build a large dataset (4GB)
- **Supervised Learning** : Training models using *TensorFlow* and *PyTorch* libraries
- **Web Development** : Setting up server logic (each project branch contains a different implementation of this part) and communication with an API (*OpenWeatherMap*)

## Weather Data :
The weather data collected using *OpenWeatherMap* (and provided to the models) includes: **longitude**, **latitude**, **temperature**, **pressure**, **wind speed**, **wind direction**.  
The value inferred by the models: **wave height**.  
For more information, see the [Data](https://github.com/LugolBis/wAIves/tree/main/DATA) folder.

## Models :
Numerous models have been trained with various parameter variations (epochs, metrics, layers, and datasets).  
These tests have identified the models providing the best results.  
For more information, see the [Models](https://github.com/LugolBis/wAIves/tree/main/Models) folder.

## Local Usage :
### Linux
Download the project and add an ```api_key.txt``` file containing your *OpenWeatherMap* API key in the **wAIves/Python/** folder.  

Run the Python script ***manage_env.py*** :
```
$ python3 manage_env.py
```
Open the ***index.html*** file and start surfing !

### Windows :
Download the project and add an ```api_key.txt``` file containing your *OpenWeatherMap* API key in the **wAIves/Python/** folder.  

Run the Python script ***manage_env.py*** from the **wAIves/Python/** folder.  

Open the ***index.html*** file and start surfing !

## Requirements :
- [OpenWeatherMap](https://openweathermap.org/appid) API key (free)

- | Python version | Compatibility |
  |:-:|:-:|
  | >= v3.10 | ✅ |
  | >= v3.9 | ✅ |
  | >= v3.8 | ✅ |
  | >= v3.7 | ✅ |
  | >= v3.6 | ✅ |
  | >= v3.5 | ✅ |
  | >= v2.7  | ✅ |
  | v3.0.* | ❌ |
  | v3.1.* | ❌ |
  | v3.2.* | ❌ |
  | v3.3.* | ❌ |
  | v3.4.* | ❌ |

## Weather Data Sources :
- NOAA
- NDBC
- Météo France
- SeaDataNet
