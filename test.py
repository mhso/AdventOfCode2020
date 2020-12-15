def rotate_point(x, y, deg):
    if deg < 0:
        deg = 360 + deg

    if deg == 90:
        return (y, -x)
    if deg == 180:
        return (-x, -y)
    if deg == 270:
        return (-y, x)
    return (x, y)

x_p = 10
y_p = 4
degrees = 180

print(rotate_point(x_p, y_p, degrees))
