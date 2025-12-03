import time

import numpy as np


def part_one():
    with open("input.txt", "r") as fp:
        lines = fp.readlines()

    pos = [50]
    for line in lines:
        sign = 1 if line[0] == "R" else -1
        shift = int(line[1:-1])
        pos.append(sign * shift)

    cumsum_pos = np.cumsum(pos)

    print("Stops at zero:", sum(cumsum_pos % 100 == 0))


def part_two():
    with open("input.txt", "r") as fp:
        lines = fp.readlines()

    # shifts = [50]
    pos = [50]
    at_zero_pos = 0
    within_full_shifts = 0
    through_zero = 0
    for line in lines:
        sign = 1 if line[0] == "R" else -1
        shift = int(line[1:-1])
        dividend = shift // 100
        remainder = shift % 100

        within_full_shifts += dividend

        possible_new_position = pos[-1] + (sign * remainder)
        new_position = possible_new_position % 100

        if pos[-1] != 0:
            if new_position == 0:
                at_zero_pos += 1
            elif not new_position == possible_new_position:
                through_zero += 1

        # shifts.append(sign * shift)
        pos.append(new_position)

    overall_zeros = at_zero_pos + within_full_shifts + through_zero

    # shifts = np.array(shifts)

    # div, rem = np.divmod(shifts, 100)
    #
    # no_full_shifts_pos = rem - 100 * (div < 0)

    # full_shifts = np.sum(np.abs(div)) - np.sum(div < 0)
    #
    # passed_through_zero = sum(
    #     np.not_equal(
    #         np.diff(np.cumsum(no_full_shifts_pos)),
    #         np.diff(np.remainder(np.cumsum(no_full_shifts_pos), 100)),
    #     )
    # )
    # print("Passed through zero within a step:", full_shifts)
    # print("Passed through zero from one step to another:", passed_through_zero)
    #
    # print("Overall:", full_shifts + passed_through_zero)

    print(f"Overall zeros: {overall_zeros}")


def main():
    print("Day 1 solutions:\n# First part")
    start_time = time.time()
    part_one()
    print(f"Part one took {(time.time() - start_time) * 1e3:.6f} miliseconds\n")

    print("# Second part")
    start_time = time.time()
    part_two()
    print(f"Part two took {(time.time() - start_time) * 1e3:.6f} miliseconds")


if __name__ == "__main__":
    main()
