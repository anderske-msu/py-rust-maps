import math
import matplotlib.pyplot as plt
import py_rust_maps as rust


def plot_map(x: list, y: list, title: str, x_label: str, y_label: str, fn: str) -> None:
    fig, ax = plt.subplots()

    ax.plot(x, y, linestyle=None, marker="o")

    ax.set_title(title)
    ax.set_xlabel(x_label)
    ax.set_ylabel(y_label)

    fig.savefig(f"{fn}.png", dpi=300)

    return


def main() -> None:
    N = 1000  # 1000 iterations

    # Standard Map
    k = -1
    ti = 0.1  # 0.1 radians
    pi = 0  # 0 momentum

    # Pendulum
    t0 = 45 * math.pi / 180  # 45 degrees
    p0 = 0  # 0 velocity
    w = 2 * math.pi * 0.5  # 0.5 Hz
    dt = 1e-2  # 10 ms
    total_time = dt * N  # 10 s

    x, y = rust.standard_map_tracking(ti, pi, k, N)
    plot_map(x, y, "Standard Map", r"$\theta$", "p", "standard-map")

    x, y = rust.pendulum_tracking(t0, p0, w, dt, N)
    plot_map(
        x,
        y,
        f"Pendulum: {total_time}s (step {dt}s)",
        r"$\theta$",
        r"$\dot{\theta}$",
        "pendulum",
    )

    return


if __name__ == "__main__":
    main()
