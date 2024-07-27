# Data Meteo 

import json
import requests

class OpenWeatherMap :
    def __init__(self,location):
        self.api_key = '0a216b54f594e070778d7d8b8390ac06'
        self.location = location

    def get_weather_data(self):
        url = f"http://api.openweathermap.org/data/2.5/weather?q={self.location}&appid={self.api_key}"
        response = requests.get(url)
        data = response.json()
        return data

lulu = OpenWeatherMap("Sagres,PT") ; print(lulu.get_weather_data())

class NOAA :

    def __init__(self):
        self.data = {
            "B01":{},"C02":{},"C05":{},"A01":{},"F01":{},"E07":{},"N01":{},"D03":{},
            "L01":{},"M01":{},"E01":{},"J02":{},"I01":{},"J03":{},"J04":{}
        }
    
        """
        self.data = {
            *station_name* : { *date* : {
                "input" : [*longitude*, *latitude*, *temperature*, *pression*, *vitesse vent*, *degré vent*], 
                "output" : [*hauteur des vagues*] 
                } }
        }
        """

    def get_data_waves(self):
        for key in self.data.keys():
            link = f"http://www.neracoos.org/erddap/tabledap/{key}_accelerometer_all.json?station%2Ctime%2Cmooring_site_desc%2Csignificant_wave_height%2Csignificant_wave_height_qc%2Cdominant_wave_period%2Cdominant_wave_period_qc%2Clongitude%2Clatitude%2Cdepth&time%3E=2000-01-01T00%3A00%3A00Z&time%3C=2024-07-20T00%3A00%3A00Z"
            response = requests.get(link)
            
            if response.status_code == 200:
                data = response.json()
                filename = r"C:\Users\lddes\OneDrive\Bureau\Discovery\wAIves\DATA" + f"\{key}_WAVES.json"
                with open(filename, 'w') as json_file:
                    json.dump(data, json_file)
                print(f"Data for {key} saved successfully.")
            else:
                print(f"Failed to retrieve data for {key}. Status code: {response.status_code}")

    def get_data_weather(self):
        for key in self.data.keys():
            link = f"http://www.neracoos.org/erddap/tabledap/{key}_met_all.json?station%2Ctime%2Cmooring_site_desc%2Cair_temperature%2Cair_temperature_qc%2Cbarometric_pressure%2Cbarometric_pressure_qc%2Cwind_gust%2Cwind_gust_qc%2Cwind_speed%2Cwind_speed_qc%2Cwind_direction%2Cwind_direction_qc%2Cwind_2_gust%2Cwind_2_gust_qc%2Cwind_2_speed%2Cwind_2_speed_qc%2Cwind_2_direction%2Cwind_2_direction_qc%2Cvisibility%2Cvisibility_qc%2Clongitude%2Clatitude%2Cdepth&time%3E=2000-01-01T00%3A00%3A00Z&time%3C=2024-07-20T00%3A00%3A00Z"
            response = requests.get(link)
            
            if response.status_code == 200:
                data = response.json()
                filename = r"C:\Users\lddes\OneDrive\Bureau\Discovery\wAIves\DATA" + f"\{key}_WEATHER.json"
                with open(filename, 'w') as json_file:
                    json.dump(data, json_file)
                print(f"Data for {key} saved successfully.")
            else:
                try:
                    link = f"http://www.neracoos.org/erddap/tabledap/{key}_met_all.json?station%2Cmooring_site_desc%2Cwater_depth%2Ctime%2Cair_temperature%2Cair_temperature_qc%2Cwind_gust%2Cwind_gust_qc%2Cwind_min%2Cwind_min_qc%2Cwind_speed%2Cwind_speed_qc%2Cwind_gust_1s%2Cwind_gust_1s_qc%2Cwind_direction%2Cwind_direction_qc%2Cwind_percent_good%2Cwind_percent_good_qc%2Ctime_created%2Ctime_modified%2Clongitude%2Clatitude%2Cdepth&time%3E=2000-01-01T00%3A00%3A00Z&time%3C=2024-07-20T00%3A00%3A00Z"
                    response = requests.get(link)
                    if response.status_code == 200:
                        data = response.json()
                        filename = r"C:\Users\lddes\OneDrive\Bureau\Discovery\wAIves\DATA" + f"\{key}_WEATHER.json"
                        with open(filename, 'w') as json_file:
                            json.dump(data, json_file) ; print(f"Data for {key} saved successfully.")
                except:
                    print(f"Failed to retrieve data for {key}. Status code: {response.status_code}")  # Certaines données sont plus simple à télécharger manuellement                

    def transform_data_waves(self):
        for key in self.data.keys():
            dictionnaire__ = {}
            file_name = r"C:\Users\lddes\OneDrive\Bureau\Discovery\wAIves\DATA" + f"\{key}_WAVES.json"
            with open(file_name,'r') as fs:
                data = json.load(fs)
                for day_ in data["table"]["rows"] :
                    if day_[3] != None :
                        dictionnaire__[day_[1]] = {
                            "input" : [day_[-2],day_[-3]],
                            "output" : [day_[3]] }
            self.data[key] = dictionnaire__ 
        
        file_name__ = r"C:\Users\lddes\OneDrive\Bureau\Discovery\wAIves\DATA\DATA_.json"
        with open(file_name__,'w') as fd:
            fd.write(json.dumps(self.data))

    def transform_data_weather(self): # Aproximation des données 
        file = r"C:\Users\lddes\OneDrive\Bureau\Discovery\wAIves\DATA\DATA_.json"
        with open(file,'r') as fs:
            data = json.load(fs)
        for key in self.data.keys():
            try:
                file_name = r"C:\Users\lddes\OneDrive\Bureau\Discovery\wAIves\DATA" + f"\{key}_WEATHER.json"
                with open(file_name,'r') as fs:
                    data__ = json.load(fs)
                    try:
                        time_index = data__["table"]["columnNames"].index("time")    # On récupère les indices des données que l'ont veut extraire
                        temperature_index = data__["table"]["columnNames"].index("air_temperature")
                        pression_index = data__["table"]["columnNames"].index("barometric_pressure")
                        wind_speed_index = data__["table"]["columnNames"].index("wind_speed")
                        wind_direction_index = data__["table"]["columnNames"].index("wind_direction")
                        
                        for day_ in data__["table"]["rows"]:
                            if day_[time_index] in data[key].keys():
                                if (day_[temperature_index] != None) and (day_[pression_index]!=None) and (day_[wind_speed_index]!=None) and (day_[wind_direction_index]!=None):
                                    data[key][day_[time_index]]["input"].append(day_[temperature_index]) # On ajoute la temperature
                                    data[key][day_[time_index]]["input"].append(day_[pression_index]) # On ajoute la pression
                                    data[key][day_[time_index]]["input"].append(day_[wind_speed_index]) # On ajoute la vitesse du vent 
                                    data[key][day_[time_index]]["input"].append(day_[wind_direction_index]) # On ajoute la direction du vent
                                else:
                                    del data[key][day_[time_index]] 
                    except:
                        try:
                            time_index = data__["table"]["columnNames"].index("time")    # On récupère les indices des données que l'ont veut extraire
                            temperature_index = data__["table"]["columnNames"].index("air_temperature")
                            wind_speed_index = data__["table"]["columnNames"].index("wind_speed")
                            wind_direction_index = data__["table"]["columnNames"].index("wind_direction")
                        
                            for day_ in data__["table"]["rows"]:
                                if day_[time_index] in data[key].keys():
                                    if (day_[temperature_index] != None) and (day_[wind_speed_index]!=None) and (day_[wind_direction_index]!=None):
                                        data[key][day_[time_index]]["input"].append(day_[temperature_index]) # On ajoute la temperature
                                        data[key][day_[time_index]]["input"].append(1013.25) # On ajoute la pression moyenne --------------------- Aproximation des données 
                                        data[key][day_[time_index]]["input"].append(day_[wind_speed_index]) # On ajoute la vitesse du vent 
                                        data[key][day_[time_index]]["input"].append(day_[wind_direction_index]) # On ajoute la direction du vent
                                    else:
                                        del data[key][day_[time_index]]
                        except:
                            print(f"Error --- Données capricieuses {key}")
            except:
                print(f"Error --- Pas de données météo pour la station {key}")
        """
        for station_name in data: # Supression des données incomplètes (on a les données pour les vagues mais pas pour la tempértaure)
            for date in data[station_name]:
                if len(data[station_name][date]["input"]) == 2:
                    del data[station_name][date]
        """
        for station, dates in data.items():
            dates_to_remove = [date for date, values in dates.items() if len(values["input"]) != 6]
            for date in dates_to_remove:
                del dates[date]
        file_name__ = r"C:\Users\lddes\OneDrive\Bureau\Discovery\wAIves\DATA\DATA_1.json"
        with open(file_name__,'w') as fd:
            fd.write(json.dumps(data))

    def transform_data_weather_2(self): # On ne prend pas en compte la pression de l'air 
        file = r"C:\Users\lddes\OneDrive\Bureau\Discovery\wAIves\DATA\DATA_.json"
        with open(file,'r') as fs:
            data = json.load(fs)
        for key in self.data.keys():
            try:
                file_name = r"C:\Users\lddes\OneDrive\Bureau\Discovery\wAIves\DATA" + f"\{key}_WEATHER.json"
                with open(file_name,'r') as fs:
                    data__ = json.load(fs)
                    try:
                        time_index = data__["table"]["columnNames"].index("time")    # On récupère les indices des données que l'ont veut extraire
                        temperature_index = data__["table"]["columnNames"].index("air_temperature")
                        wind_speed_index = data__["table"]["columnNames"].index("wind_speed")
                        wind_direction_index = data__["table"]["columnNames"].index("wind_direction")
                    
                        for day_ in data__["table"]["rows"]:
                            if day_[time_index] in data[key].keys():
                                data[key][day_[time_index]]["input"].append(day_[temperature_index]) # On ajoute la temperature
                                data[key][day_[time_index]]["input"].append(day_[wind_speed_index]) # On ajoute la vitesse du vent 
                                data[key][day_[time_index]]["input"].append(day_[wind_direction_index]) # On ajoute la direction du vent 
                    except:
                        print(f"Error --- Données incomplètes {key}_WEATHER.json")
            except:
                print(f"Error --- Pas de données météo pour la station {key}")
        for station, dates in data.items():
            dates_to_remove = [date for date, values in dates.items() if len(values["input"]) != 5]
            for date in dates_to_remove:
                del dates[date]
        file_name__ = r"C:\Users\lddes\OneDrive\Bureau\Discovery\wAIves\DATA\DATA_2.json"
        with open(file_name__,'w') as fd:
            fd.write(json.dumps(data))

#noaa = NOAA()
#noaa.get_data_waves()  -- Success
#noaa.get_data_weather() -- Success
#noaa.transform_data_waves() -- Success
#noaa.transform_data_weather() -- Success
#noaa.transform_data_weather_2() -- Success




# NCEI NOAA -- solution pour récupérer du data sur les vagues
# lien vers la documentation : https://www.ncei.noaa.gov/support/access-data-service-api-user-documentation
# requête : https://www.ncei.noaa.gov/access/services/data/v1?dataset=daily-summaries&dataTypes=DP01,DP05,DP10,DSND,DSNW,DT00,DT32,DX32,DX70,DX90,SNOW,PRCP&stations=ASN00084027&startDate=1960-01-01&endDate=1970-12-31&includeAttributes=true&format=json

# Obtention des données sur les vagues : 
# lien du site : https://www.neracoos.org/erddap/tabledap/B01_accelerometer_all.html
# Requête à mettre dans le naviguateur - vagues : http://www.neracoos.org/erddap/tabledap/B01_accelerometer_all.json?station%2Ctime%2Cmooring_site_desc%2Csignificant_wave_height%2Csignificant_wave_height_qc%2Cdominant_wave_period%2Cdominant_wave_period_qc%2Clongitude%2Clatitude%2Cdepth&time%3E=2000-07-13T00%3A00%3A00Z&time%3C=2024-07-20T07%3A00%3A00Z
# Sélection d'une collection : https://data.noaa.gov/onestop/collections?q=temperature

# Liens HTTP de la page internet pour télécharger les fichiers :
# WAVES : https://www.neracoos.org/erddap/tabledap/C05_accelerometer_all.html
# DEGRES : https://www.neracoos.org/erddap/tabledap/A01_met_all.html
# DEGRES 2 : https://www.neracoos.org/erddap/tabledap/J04_met_all.html 