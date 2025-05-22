import sys
import json
import plotly.graph_objects as go
import plotly.express.colors as pcq  # Alternative valide

def show_stations(longitudes: list[float], latitudes: list[float]):
    """
    Simple use of plotly to create an interactive map of the weather stations
    """ 
    fig = go.Figure(go.Scattergeo(
        lon = longitudes,
        lat = latitudes,
        mode = 'markers',
        marker = dict(
            size = 10,
            color = 'blue'
        )
    ))

    fig.update_geos(scope='world')
    fig.update_layout(title='Carte des stations météorologique du dataset')
    fig.show()

def show_clusters(longitudes_groups: list[list[float]], latitudes_groups: list[list[float]]):

    fig = go.Figure()
    colors = [
        '#FF0000',  # Rouge
        '#00FF00',  # Vert
        '#0000FF',  # Bleu
        '#FFA500',  # Orange
        '#800080',  # Violet
        '#FF69B4'   # Rose
    ]
    
    for i, (lons, lats) in enumerate(zip(longitudes_groups, latitudes_groups)):
        fig.add_trace(go.Scattergeo(
            lon = lons,
            lat = lats,
            mode = 'markers',
            marker = dict(
                size = 10,
                color = colors[i % len(colors)]
            ),
            name = f'Groupe {i+1}'
        ))
    
    fig.update_geos(scope='world')
    fig.update_layout(title='Carte des stations météorologiques du dataset')
    fig.show()

if __name__ == '__main__':
    args = sys.argv
    file_path = args[1]
    state = True
    coords_x = []
    coords_y = []

    with open(file_path, "r") as fs:
        data = json.load(fs)

    if len(args) == 2:
        for coord in data:
            if state:
                coords_x.append(coord)
                state = False
            else:
                coords_y.append(coord)
                state = True
        show_stations(coords_x,coords_y)

    elif len(args) == 3 and args[2]=="--cluster":
        coords_x_groups = []
        coords_y_groups = []

        for liste in data:
            x_group = []
            y_group = []

            for coord in liste:
                if state:
                    x_group.append(coord)
                    state = False
                else:
                    y_group.append(coord)
                    state = True

            coords_x_groups.append(x_group)
            coords_y_groups.append(y_group)

        show_clusters(coords_x_groups, coords_y_groups)