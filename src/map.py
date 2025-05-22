import sys
import json
import plotly.graph_objects as go

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

def show_clusters(
    longitudes_groups: list[list[float]],
    latitudes_groups: list[list[float]],
    centroids_longitude: list[float],
    centroids_latitude: list[float],
    save_path: str = None
):

    fig = go.Figure()
    colors = [
        '#FF0000',  
        '#00FF00',  
        '#0000FF',  
        '#FFA500',  
        '#800080', 
        '#FF69B4'
    ]
    
    for index, (lons, lats) in enumerate(zip(longitudes_groups, latitudes_groups)):
        fig.add_trace(go.Scattergeo(
            lon = lons,
            lat = lats,
            mode = 'markers',
            marker = dict(
                size = 10,
                color = colors[index % len(colors)]
            ),
            name = f'Cluster {index+1}'
        ))
        
    for index in range(0,len(centroids_longitude)):
        fig.add_trace(go.Scattergeo(
            lon = [centroids_longitude[index]],
            lat = [centroids_latitude[index]],
            mode = 'markers',
            marker = dict(
                size = 10,
                color = colors[index % len(colors)],
                symbol = "x-open-dot"
            ),
            name = f'Centroid {index+1}'
        ))
    
    fig.update_geos(scope='world')
    fig.update_layout(title='Carte des stations météorologiques du dataset')

    if save_path != None:
        fig.write_image(save_path)
    else:
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

    elif len(args) >= 3:
        with open(args[2], "r") as fs:
            object = json.load(fs)
        
        centroids_x = []
        centroids_y = []
        for centroid in object:
            parsed_list = centroid.split(";")
            coord_x, coord_y = float(parsed_list[1]), float(parsed_list[2])
            centroids_x.append(coord_x)
            centroids_y.append(coord_y)

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

        if len(args) == 4:
            show_clusters(coords_x_groups, coords_y_groups,centroids_x,centroids_y, args[3])
        else:
            show_clusters(coords_x_groups, coords_y_groups,centroids_x,centroids_y)