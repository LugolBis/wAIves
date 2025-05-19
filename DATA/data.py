"""
Meteorological Data
"""

import os
import json
import requests
import gzip
import shutil
import csv
from bs4 import BeautifulSoup
import time
import re

class OpenWeatherMap :
    """This class is useless. It's permit to acces to the current weather."""
    def __init__(self,apiKey,location):
        self.api_key = apiKey
        self.location = location

    def getWeatherData(self):
        url = f"http://api.openweathermap.org/data/2.5/weather?q={self.location}&appid={self.api_key}"
        response = requests.get(url)
        data = response.json()
        return data

class NOAA :
    def __init__(self):
        self.data = {
            "B01":{},"C02":{},"C05":{},"A01":{},"F01":{},"E07":{},"N01":{},"D03":{},
            "L01":{},"M01":{},"E01":{},"J02":{},"I01":{},"J03":{},"J04":{}
        }
    
        """
        self.data = {
            *station_name* : { *date* : {
                "input" : [*longitude*, *latitude*, *temperature*, *pressure*, *vitesse vent*, *degré vent*], 
                "output" : [*hauteur des vagues*] 
                } }
        }
        """

    def getDataWaves(self,data_dir:str):
        """This function download the data of the waves from the web in the *data_dir*."""
        for key in self.data.keys():
            link = f"http://www.neracoos.org/erddap/tabledap/{key}_accelerometer_all.json?station%2Ctime%2Cmooring_site_desc%2Csignificant_wave_height%2Csignificant_wave_height_qc%2Cdominant_wave_period%2Cdominant_wave_period_qc%2Clongitude%2Clatitude%2Cdepth&time%3E=2000-01-01T00%3A00%3A00Z&time%3C=2024-07-20T00%3A00%3A00Z"
            response = requests.get(link)
            
            if response.status_code == 200:
                data = response.json()
                filename = os.path.join(os.path.normpath(data_dir), f"{key}_WAVES.json")
                with open(filename, 'w') as json_file:
                    json.dump(data, json_file)
                print(f"Data for {key} saved successfully.")
            else:
                print(f"Failed to retrieve data for {key}. Status code: {response.status_code}")

    def getDataWeather(self,data_dir:str):
        """This function download the data of the weather from the web in the *data_dir*."""
        for key in self.data.keys():
            link = f"http://www.neracoos.org/erddap/tabledap/{key}_met_all.json?station%2Ctime%2Cmooring_site_desc%2Cair_temperature%2Cair_temperature_qc%2Cbarometric_pressure%2Cbarometric_pressure_qc%2Cwind_gust%2Cwind_gust_qc%2Cwind_speed%2Cwind_speed_qc%2Cwind_direction%2Cwind_direction_qc%2Cwind_2_gust%2Cwind_2_gust_qc%2Cwind_2_speed%2Cwind_2_speed_qc%2Cwind_2_direction%2Cwind_2_direction_qc%2Cvisibility%2Cvisibility_qc%2Clongitude%2Clatitude%2Cdepth&time%3E=2000-01-01T00%3A00%3A00Z&time%3C=2024-07-20T00%3A00%3A00Z"
            response = requests.get(link)
            
            if response.status_code == 200:
                data = response.json()
                filename = os.path.join(os.path.normpath(data_dir), f"{key}_WEATHER.json")
                with open(filename, 'w') as json_file:
                    json.dump(data, json_file)
                print(f"Data for {key} saved successfully.")
            else:
                try:
                    link = f"http://www.neracoos.org/erddap/tabledap/{key}_met_all.json?station%2Cmooring_site_desc%2Cwater_depth%2Ctime%2Cair_temperature%2Cair_temperature_qc%2Cwind_gust%2Cwind_gust_qc%2Cwind_min%2Cwind_min_qc%2Cwind_speed%2Cwind_speed_qc%2Cwind_gust_1s%2Cwind_gust_1s_qc%2Cwind_direction%2Cwind_direction_qc%2Cwind_percent_good%2Cwind_percent_good_qc%2Ctime_created%2Ctime_modified%2Clongitude%2Clatitude%2Cdepth&time%3E=2000-01-01T00%3A00%3A00Z&time%3C=2024-07-20T00%3A00%3A00Z"
                    response = requests.get(link)
                    if response.status_code == 200:
                        data = response.json()
                        filename = os.path.join(os.path.normpath(data_dir), f"{key}_WEATHER.json")
                        with open(filename, 'w') as json_file:
                            json.dump(data, json_file) ; print(f"Data for {key} saved successfully.")
                except:
                    print(f"Failed to retrieve data for {key}. Status code: {response.status_code}")  # Certaines données sont plus simple à télécharger manuellement                

    def transformDataWaves(self,data_dir:str):
        for key in self.data.keys():
            dictionnaire__ = {}
            file_name = os.path.join(os.path.normpath(data_dir), f"{key}_WAVES.json")
            with open(file_name,'r') as fs:
                data = json.load(fs)
                for day_ in data["table"]["rows"] :
                    if day_[3] != None :
                        dictionnaire__[day_[1]] = {
                            "input" : [day_[-2],day_[-3]],
                            "output" : [day_[3]] }
            self.data[key] = dictionnaire__ 
        
        file_name__ = os.path.join(os.path.normpath(data_dir), f"DATA_.json")
        with open(file_name__,'w') as fd:
            fd.write(json.dumps(self.data))

    def transformDataWeather(self,data_dir:str): # Aproximation des données 
        file = os.path.join(os.path.normpath(data_dir), f"\DATA_.json")
        with open(file,'r') as fs:
            data = json.load(fs)
        for key in self.data.keys():
            try:
                file_name = os.path.join(os.path.normpath(data_dir), f"{key}_WEATHER.json")
                with open(file_name,'r') as fs:
                    data__ = json.load(fs)
                    try:
                        time_index = data__["table"]["columnNames"].index("time")    # On récupère les indices des données que l'ont veut extraire
                        temperature_index = data__["table"]["columnNames"].index("air_temperature")
                        pressure_index = data__["table"]["columnNames"].index("barometric_pressure")
                        wind_speed_index = data__["table"]["columnNames"].index("wind_speed")
                        wind_direction_index = data__["table"]["columnNames"].index("wind_direction")
                        
                        for day_ in data__["table"]["rows"]:
                            if day_[time_index] in data[key].keys():
                                if (day_[temperature_index] != None) and (day_[pressure_index]!=None) and (day_[wind_speed_index]!=None) and (day_[wind_direction_index]!=None):
                                    data[key][day_[time_index]]["input"].append(day_[temperature_index]) # On ajoute la temperature
                                    data[key][day_[time_index]]["input"].append(day_[pressure_index]) # On ajoute la pressure
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
                                        data[key][day_[time_index]]["input"].append(1013.25) # On ajoute la pressure moyenne --------------------- Aproximation des données 
                                        data[key][day_[time_index]]["input"].append(day_[wind_speed_index]) # On ajoute la vitesse du vent 
                                        data[key][day_[time_index]]["input"].append(day_[wind_direction_index]) # On ajoute la direction du vent
                                    else:
                                        del data[key][day_[time_index]]
                        except:
                            print(f"Error --- Données capricieuses {key}")
            except:
                print(f"Error --- Pas de données météo pour la station {key}")
        """
        for station_name in data: # Supressure des données incomplètes (on a les données pour les vagues mais pas pour la tempértaure)
            for date in data[station_name]:
                if len(data[station_name][date]["input"]) == 2:
                    del data[station_name][date]
        """
        for station, dates in data.items():
            dates_to_remove = [date for date, values in dates.items() if len(values["input"]) != 6]
            for date in dates_to_remove:
                del dates[date]
        file_name__ = os.path.join(os.path.normpath(data_dir), f"DATA_1.json")
        with open(file_name__,'w') as fd:
            fd.write(json.dumps(data))

class MeteoFrance :
    def __init__(self) :
        self.link = "https://donneespubliques.meteofrance.fr/?fond=donnee_libre&prefixe=Txt%2FMarine%2FArchive%2Fmarine&extension=csv.gz&date="
        self.years = [str(nb) for nb in range(1996,2025)]
        self.months = ["01","02","03","04","05","06","07","08","09","10","11","12"]
        self.dictionaryKeys = {"numer_sta":["Station name",None],"date":["Date (UTC)","AAAAMMDDHHMISS"],"t":["Temperature","K"],
            "lat":["Latitude","DD"], "lon":["Longitude","DD"], "dd":["Direction wind","Degre °"], "ff":["Speed wind","m/s (meters)"],
            "pmer":["pressure","Pa"], "Hw1Hw1":["Height waves","m (meters)"]} 

    def getData(self):
        """I have yet to automate the data recovery process.
        \nBut you can do it manually with the method 'self.link' and add the date.
        \nI give you here an example : https://donneespubliques.meteofrance.fr/?fond=donnee_libre&prefixe=Txt%2FMarine%2FArchive%2Fmarine&extension=csv.gz&date=199601
        """
        pass

    def decompressGZ(self,gz_file_path, dir_path):
        """Unzip a .gz file and extract its contents to the destination folder."""

        if not os.path.exists(dir_path):
            os.makedirs(dir_path)

        file_name = os.path.basename(gz_file_path).replace('.gz', '')
        extracted_file_path = os.path.join(dir_path, file_name)
        
        with gzip.open(gz_file_path, 'rb') as f_in:
            with open(extracted_file_path, 'wb') as f_out:
                shutil.copyfileobj(f_in, f_out)
        
        print(f"Fichier décompressé et extrait vers : {extracted_file_path}")

    def transformData(self,dir_data_path:str):
        """This function extract the data we needed from the data of Meteo France and save it in a JSON.\n
        You just need to give the path of the dir where you saved the .csv files."""
        DataMeteoFrance = {}
        filesCSV = []

        for root, dirs, files in os.walk(dir_data_path):
            for file in files:
                if file.endswith(".csv"):
                    filesCSV.append(os.path.join(root, file))
        print(filesCSV)

        for path in filesCSV:
            try:
                with open(path, 'r') as fs:
                    reader = csv.DictReader(fs)
                    for line in reader:
                        for key in line:
                            data = line[key].split(';')
                            stationName = data[0] ; date = data[1]
                            
                            inputs = [float(data[nb]) for nb in [3, 2, 4, 9, 8, 7] if (data[nb]!="mq")]
                            if len(inputs) == 6 :
                                inputs[2] -= 273.15 ; inputs[3] /= 100   # Convertion d'unités
                                if data[14]=="mq":
                                    output = [float(data[11])]
                                else:
                                    output = [float(data[14])]
                                if stationName in DataMeteoFrance.keys() :
                                    dico_station = DataMeteoFrance[stationName]
                                    dico_station[date] = {"input":inputs,"output":output}
                                    DataMeteoFrance[stationName] = dico_station
                                else:
                                    dico_station = {}
                                    dico_station[date] = {"input":inputs,"output":output}
                                    DataMeteoFrance[stationName] = dico_station
            except Exception as error:
                print(f"ERROR -- transfromData() : {error} -- {os.path.basename(path)}")
        print(DataMeteoFrance)
        with open(os.path.join(dir_data_path, "DATA_MeteoFrance.json"), "w") as fd :
            json.dump(DataMeteoFrance,fd)

class SeaDataNet :
    """This class doesn't work !
    \nYou just could download the data with the link in the function 'getData()'."""
    def __init__(self):
        self.name = "SeaDataNet"

    def getData(self):
        """I have yet to automate the data recovery process.
        \nBut you can do it manually with the website.
        \nI give you here the link : https://www.seadatanet.org/ """
        pass

    def transformData(self,data_dir):
        """This function transforms data from specific text files to JSON file.
        \nIt works with the data files that i download, but it can easily don't work with other kind of data."""
        DATA = []
        files_path = [os.path.join(data_dir, file) for file in os.listdir(data_dir) if file.endswith('.txt') and ('BSWAVE' not in file)]
        print(len(files_path))

        for path in files_path:
            data__ = [0,None,None]
            liste = []
            try:
                with open(path,'r') as fs:
                    for line in fs:
                        liste.append(line)
                for line in liste[26:]:
                    data__[0] += 1
                    elements = line.split('\t')
                    try:
                        if data__[0] == 1:
                            longitude, latitude = float(elements[4]), float(elements[5])
                            data__[1] = longitude ; data__[2] = latitude
                            temperature, pressure = float(elements[31]), float(elements[25])
                            windSpeed, windDirection = float(elements[17]), float(elements[15])
                            if elements[29].startswith('.') : waveHeight = float('0'+elements[29])
                            else : waveHeight = float('0'+elements[29])
                        else:
                            longitude, latitude = data__[1], data__[2]
                            temperature, pressure = float(elements[31]), float(elements[25])
                            windSpeed, windDirection = float(elements[17]), float(elements[15])
                            waveHeight = float(elements[29])
                            if elements[29].startswith('.') : waveHeight = float('0'+elements[29])
                            else : waveHeight = float('0'+elements[29])
                        DATA.append([longitude,latitude,temperature,pressure,windSpeed,windDirection,waveHeight])
                    except Exception as error:
                        print(f"\nERROR -- transformData() : {error} \n{elements}\n{path}")
            except Exception as error :
                print(f"\nERROR -- transformData() : {error}\nfile : {path}")

        with open(os.path.join(data_dir,'DATA_SeaDataNetLight_aproximations.json'), 'w') as fd:
            json.dump(DATA,fd)

class NDBC:
    def __init__(self):
        self.linkStationsName = "https://www.ndbc.noaa.gov/historical_data.shtml#stdmet"
        self.linkSearchStation = "https://www.ndbc.noaa.gov/station_page.php?station="
        self.linkData = "https://www.ndbc.noaa.gov/view_text_file.php?filename=STATIONhDATE.txt.gz&dir=data/historical/stdmet/"
    
    def getStationsName(self,path_out:str):
        """This function recover all the stations name and save these in a JSON file.
        \nIt's a simple 'scraping text' function."""
        try:
            response = requests.get(self.linkStationsName)
            response.raise_for_status()

            soup = BeautifulSoup(response.content, 'html.parser')
            stdmet_a = soup.find('a', id='stdmet')
            stdmet_li = stdmet_a.find_parent('li') if stdmet_a else None
            ul = stdmet_li.find('ul') if stdmet_li else None
            
            if ul:
                items = []
                for li in ul.find_all('li'):
                    a_tag = li.find('a')
                    if a_tag:
                        items.append((a_tag.get_text(strip=True), li.get_text(strip=True)))

                data = {}
                for tuple_ in items:
                    Station_Date = tuple_[1].split(':')
                    StationName = Station_Date[0]
                    Dates = Station_Date[1]
                    listeDates = []
                    while Dates != '':
                        listeDates.append(Dates[:4])
                        Dates = Dates[4:]
                    data[StationName] = listeDates
                with open(path_out,'w') as fd:
                    json.dump(data,fd)
                return items
            else:
                print("Aucune balise <ul> trouvée sous 'stdmet'") ; return []
        except requests.exceptions.RequestException as e:
            print(f"Erreur lors de l'accès à la page web : {e}") ; return []
    
    def getLocation(self,stationName,element_tag='b',class_name=None):
        """This function recover the Latitude and the longitude from the web page of the station.
        \nIt's a simple 'scraping text' function."""
        url = self.linkSearchStation + stationName
        try:
            response = requests.get(url)
            response.raise_for_status()

            soup = BeautifulSoup(response.content, 'html.parser')

            if class_name:
                elements = soup.find_all(element_tag, class_=class_name)
            else:
                elements = soup.find_all(element_tag)

            elements_ = [element.get_text(strip=True) for element in elements]
            for elem in elements_ :
                if re.match(r"(\d+)\.(\d+) (\w)", elem) != None:
                    return elem
            return ""
        except requests.exceptions.RequestException as e:
            print(f"Erreur lors de l'accès à la page web : {e}") ; return ""
        
    def getData(self,path_stationsName,path_dir:str):
        """This function take the data from the Stations Name file and use it to get the data of these stations.
        \nYou need to use the 'getStationsName()' method before using this function.
        \n/!\ Please before using the function make sure there isn't any JSON file in your 'path_dir'."""
        # Extracting the longitude and latitude of the web page
        def extract_coordinates(coordinate_string):
            def convert_to_dd(coordinate, direction):
                degrees = float(coordinate)
                if direction in ['S', 'W']:
                    degrees *= -1
                return degrees
            if coordinate_string == "" :
                return None, None
            parts = coordinate_string.split()
            latitude = convert_to_dd(parts[0], parts[1])
            longitude = convert_to_dd(parts[2], parts[3])
            return float(longitude), float(latitude)
        
        with open(path_stationsName,'r') as fs:
            SND = json.load(fs) # variable containing the dictionary of station names and their measurement dates 
        
        for stationName in SND :
            data_station = []
            longitude, latitude = extract_coordinates(self.getLocation(stationName))
            if (longitude!=None) and (latitude!=None):
                for year in SND[stationName] :
                    try:
                        url = f"https://www.ndbc.noaa.gov/view_text_file.php?filename={stationName.lower()}h{year}.txt.gz&dir=data/historical/stdmet/"
                        response = requests.get(url)
                        texte = response.text.split('\n')
                        for line in texte[2:-1]:
                            elements = [elem for elem in line.split() if (elem!='')]
                            temperature = float(elements[13]) ; pressure = float(elements[12])
                            windSpeed = float(elements[6]) ; windDirection = float(elements[5])
                            waveHeight = float(elements[8])
                            if (waveHeight!=99.00) and (waveHeight!=0.00): # deleting the incoherent data 
                                data_station.append([longitude,latitude,temperature,pressure,windSpeed,windDirection,waveHeight])
                    except Exception as error:
                        print(f"\nERROR -- getData() : {error} -- Station : {stationName}\nCode erreur serveur : {response.status_code}\n")
                if data_station != []:
                    with open(os.path.join(path_dir,f"{stationName}.json"), 'w') as fd:
                        json.dump(data_station,fd)
            else:
                print(f"ERROR -- Impossible de récupérer les coordonnées de la station : {stationName}") 

    def fusionData(input_dir, output_dir, num_output_files=8):
        """There is a lot of data and files from the NDBC. 
        \nThis function split all the data collect and save in 400+ files in less files."""

        json_files = [(os.path.join(input_dir, f), os.path.getsize(os.path.join(input_dir, f))) 
                    for f in os.listdir(input_dir) if f.endswith('.json')]
        json_files.sort(key=lambda x: x[1], reverse=True)
        
        groups = [[] for _ in range(num_output_files)]
        group_sizes = [0] * num_output_files

        for file, size in json_files:
            min_group_idx = group_sizes.index(min(group_sizes))
            groups[min_group_idx].append(file)
            group_sizes[min_group_idx] += size
        
        for i, group in enumerate(groups):
            merged_data = []
            output_file = os.path.join(output_dir, f"DATA_NDBCLight_{i+1}.json")
            
            for file in group:
                with open(file, 'r') as infile:
                    data = json.load(infile)
                    merged_data.extend(data)

            with open(output_file, 'w') as outfile:
                json.dump(merged_data, outfile)
            print(f"File : {output_file} created successfully.")

class lightData:
    def __init__(self):
        self.format = "[{'input':[0,1,2,3,4,5],'output':[0]}]"

    def transformData(self,data_path):
        """This function transform data like this :
        {*station_name* : { *date* : {"input" : [*longitude*, *latitude*,*temperature*, *pressure*, *vitesse vent*, *degré vent*], "output" : [*hauteur des vagues*] } }
        -----  to this format : [[*longitude*, *latitude*, *temperature*, *pressure*, *vitesse vent*, *degré vent*,*hauteur des vagues*]]"""

        with open(data_path,'r') as fs:
            data = json.load(fs)

        data__ = []
        for station, dates in data.items():
            for date, data in dates.items():
                data__.append(data["input"]+data["output"])

        filename = f"{data_path[:-5]}Light.json"
        with open(filename,'w') as fd:
            json.dump(data__,fd)   

    def verifyData(self,data_path)->bool:
        """This function verify if the data is in the correct form 
        \nand return a boolean, *True* if it's correct and *False* if there is a problem."""
        with open(data_path,'r') as fs:
            data = json.load(fs)
        
        Error = [0,0]
        for liste in data:
            if len(liste)!=7: Error[0] += 1
            for elem in liste: 
                if isinstance(elem,float) != True : Error[1] += 1
        print(f"\n{data_path}\nError len of liste : {Error[0]} -- Error type of element : {Error[1]}")
        if Error[0] == Error[1] == 0 : return True
        else : return False
        

"""
NCEI NOAA -- solution pour récupérer du data sur les vagues :
lien vers la documentation : https://www.ncei.noaa.gov/support/access-data-service-api-user-documentation
requête : https://www.ncei.noaa.gov/access/services/data/v1?dataset=daily-summaries&dataTypes=DP01,DP05,DP10,DSND,DSNW,DT00,DT32,DX32,DX70,DX90,SNOW,PRCP&stations=ASN00084027&startDate=1960-01-01&endDate=1970-12-31&includeAttributes=true&format=json

Obtention des données sur les vagues : 
lien du site : https://www.neracoos.org/erddap/tabledap/B01_accelerometer_all.html
Requête à mettre dans le naviguateur - vagues : http://www.neracoos.org/erddap/tabledap/B01_accelerometer_all.json?station%2Ctime%2Cmooring_site_desc%2Csignificant_wave_height%2Csignificant_wave_height_qc%2Cdominant_wave_period%2Cdominant_wave_period_qc%2Clongitude%2Clatitude%2Cdepth&time%3E=2000-07-13T00%3A00%3A00Z&time%3C=2024-07-20T07%3A00%3A00Z
Sélection d'une collection : https://data.noaa.gov/onestop/collections?q=temperature

Liens HTTP de la page internet pour télécharger les fichiers :
WAVES : https://www.neracoos.org/erddap/tabledap/C05_accelerometer_all.html
DEGRES : https://www.neracoos.org/erddap/tabledap/A01_met_all.html
DEGRES 2 : https://www.neracoos.org/erddap/tabledap/J04_met_all.html 

Météo France - Solution pour récupérer du data sur les vagues :
lien vers la page de présentation : https://donneespubliques.meteofrance.fr/?fond=produit&id_produit=95&id_rubrique=32 
Requête pour obtenir des données : https://donneespubliques.meteofrance.fr/?fond=donnee_libre&prefixe=Txt%2FMarine%2FArchive%2Fmarine&extension=csv.gz&date=199601
lien vers le pdf décrivant les champs : https://donneespubliques.meteofrance.fr/client/document/doc_parametres_ship_169.pdf

NDBC - Solution pour récupérer du data sur les vagues :
lien de la page pour rechercher les stations : https://www.ndbc.noaa.gov/historical_data.shtml#swdir
lien de la description des variables : https://www.ndbc.noaa.gov/faq/measdes.shtml
Exemple d'une station : https://www.ndbc.noaa.gov/station_page.php?station=32012
Requête pour obtenir les données : https://www.ndbc.noaa.gov/view_text_file.php?filename=32012h2007.txt.gz&dir=data/historical/stdmet/ 
"""