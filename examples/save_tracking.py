import math
import numpy as np
import py_rust_maps


def main() -> None:
    N = 1_000_000  # iterations

    # Standard Map
    k = -1
    ti = 0.15  # angle [rad]
    pi = 0  # momentum [rad]

    # Pendulum
    t0 = 45 * math.pi / 180  # angle [rad]
    p0 = 0  # velocity  [rad/s]
    w = 2 * math.pi * 0.5  # frequency [Hz]
    dt = 1e-2  # time step [s]
    total_time = dt * N

    x, y = py_rust_maps.standard_map_tracking(ti, pi, k, N)
    x = np.array(x, dtype=np.float64)
    y = np.array(y, dtype=np.float64)
    np.savetxt("standard_tracking_theta.csv", x, delimiter=",")
    np.savetxt("standard_tracking_p.csv", y, delimiter=",")

    x, y = py_rust_maps.pendulum_tracking(t0, p0, w, dt, N)
    x = np.array(x, dtype=np.float64)
    y = np.array(y, dtype=np.float64)
    np.savetxt("pendulum.csv", np.column_stack((x, y)).T, delimiter=",")

    return


if __name__ == "__main__":
    main()
