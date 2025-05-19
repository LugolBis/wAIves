import sys
import json
import plotly.graph_objects as go

def show_map(longitudes: list[float], latitudes: list[float]):
    """
    Simple use of plotly to create an interactive map of the weather stations
    """ 
    fig = go.Figure(go.Scattergeo(
        lon = longitudes,
        lat = latitudes,
        mode = 'markers',
        marker = dict(
            size = 20,
            color = 'blue'
        )
    ))

    fig.update_geos(scope='world')
    fig.update_layout(title='Carte des stations météorologique du dataset')
    fig.show()

if __name__ == '__main__':
    file_path = sys.argv[1]
    state = True
    coords_x = []
    coords_y = []

    with open(file_path, "r") as fs:
        coordinates = json.load(fs)

    for coord in coordinates:
        if state:
            coords_x.append(coord)
            state = False
        else:
            coords_y.append(coord)
            state = True

    show_map(coords_x,coords_y)