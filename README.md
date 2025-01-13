# wAIves 🏄

## Objectif du projet :
L'objectif de ce projet **personnel** est de développer des modèles d'**IA** capables de prédire des conditions de Surf à partir de données météorologiques.
Il est donc ici question de Fouille de données, d'Apprentissage supervisé et de développement d'une application web.

## Étapes clées du projet :
- **Fouille de données** : Text Scraping, Analyse et Formatage de données pour constituer un large dataset (4GO)
- **Apprentissage supervisé** : Training de models à partir des librairies *Tensorflow* et *Pytorch*
- **Développement web** : Mise en place de la logique d'un server (chaque branche du projet contient une implémentation différente de cette partie) et
communication avec une API (*OpenWeatherMap*)

## Données météorologique :
Les donnés météorologiques récoltées  à l'aide de *OpenWeatherMap* (et fournies aux modèles) sont les suivantes : **longitude**, **latitude**, **temperature**, **pressure**, **wind speed**, **wind direction**
<br>
La donnée inférée par les modèles : **wave height**
<br>
<br>
Pour plus d'information regardez le dossier [Data](https://github.com/LugolBis/wAIves/tree/web/DATA)

## Models :
De nombreux models ont été entrainés avec diverses variation de leurs paramètres d'entraînement (epochs, metrics, layers et datasets).
<br>
Ces tests ont permis de faire émerger les modèles fournissant les meilleurs résultats.
<br>
<br>
Pour plus d'information regardez le dossier [Models](https://github.com/LugolBis/wAIves/tree/web/Models)

## Utilisation en ligne :
*En cours de développement*
- Github Pages 🚧
- Diminutions des problèmes de biais & réentrainement de models 🚧
- Intégration des modèles Pytorch 🚧

## Sources des données météorologique :
- NOAA
- NDBC
- Météo France
- SeaDataNet
