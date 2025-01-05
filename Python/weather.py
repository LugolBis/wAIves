import json
import re
import requests

def get_location(city_country):
    try:
        # Chargement du fichier countries.json (si vous avez ce fichier localement, sinon téléchargez-le depuis un serveur)
        with open('countries.json', 'r') as f:
            data = json.load(f)

        # Expression régulière pour extraire la ville et le pays
        regex = r"^\s*([\w\s'-]+)\s*,\s*([\w\s'-]+)\s*$"
        match = re.match(regex, city_country)

        if not match:
            print("Incorrect format. Please check the entry.")
            return None
        
        city = match.group(1).strip()
        country = match.group(2).strip()

        # Obtenir l'abréviation du pays
        country_code = data.get(country)

        if not country_code:
            print("Country abbreviation not found. Please check the country entered.")
            return None
        return f"{city},{country_code}"

    except Exception as error:
        print(error)
        return None
    
def get_api_key():
    try:
        with open('api_key.txt') as fs:
            return fs.read()
    except FileNotFoundError :
        return "You haven't got an API key to acces to the data of OpenWeatherMap."
    
def get_current_weather(location):
    api_key = get_api_key()
    url = f"http://api.openweathermap.org/data/2.5/weather?q={location}&appid={api_key}"
    try:
        response = requests.get(url)
        if response.status_code != 200:
            print(f"Error: Failed to fetch weather data for {location}. Trying default location.")
            default_location = "Sagres,PT"
            response_test = requests.get(f"http://api.openweathermap.org/data/2.5/weather?q={default_location}&appid={api_key}")

            if response_test.status_code != 200:
                print("wAIves fails to connect to server hosting weather data. Please try again later.") ; return None
            else:
                print("Weather data for your chosen location is not available. Using default location.") ; return None
        else:
            data = response.json()
            return data
    except Exception as error:
        print(f"Error fetching weather data: {error}") ; return None
    
def transform_weather_data(data):
    input = []
    input.append(data['coord']['lon']) ; input.append(data['coord']['lat'])
    input.append(data['main']['temp'] - 273.15)  # Conversion de Kelvin à Celsius
    input.append(data['main']['pressure'])
    input.append(data['wind']['speed']) ; input.append(data['wind']['deg'])
    return input