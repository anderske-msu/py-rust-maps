import math
import numpy as np
import py_rust_maps


def transpose_list(x: list) -> list:
    return np.array(x).T.tolist()


def save_3d_data(data: list, filename: str) -> None:
    data_2d = np.array(data).reshape(np.shape(data)[0], -1)
    np.savetxt(filename, data_2d, delimiter=",")
    return


def main() -> None:
    number_iterations = 10_000
    number_initial_conditions = 20
    maximum_amplitude = math.pi / 3
    filename = "standard_map_data.csv"

    # Standard Map
    # ti = 0.15
    k = -0.5 # -4 < k < 0 for stability around (0, 0)
    initial_p = 0

    data = []
    for initial_theta in np.linspace(0, maximum_amplitude, number_initial_conditions):
        theta, p = py_rust_maps.standard_map_tracking(
            initial_theta, initial_p, k, number_iterations
        )
        data.append(transpose_list([theta, p]))
        # transpose_list is needed because data should be
        # (number_initial_conditions, number_iterations, 2)

    save_3d_data(data, filename)

    return


if __name__ == "__main__":
    main()
