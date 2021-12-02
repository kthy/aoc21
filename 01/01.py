from dataclasses import dataclass
from typing import Sequence

INFINITY = float("inf")


@dataclass
class SlidingWindow:
    old: int
    mid: int
    new: int

    def slide(self, new_new: int) -> None:
        self.old = self.mid
        self.mid = self.new
        self.new = new_new

    def sum(self) -> int:
        w = self.window()
        return sum(w) if all(w) else INFINITY

    def window(self) -> Sequence[int]:
        return [self.old, self.mid, self.new]


if __name__ == "__main__":
    counter = 0
    sliding_counter = 0
    last_depth = INFINITY
    sliding_window = SlidingWindow(INFINITY, INFINITY, INFINITY)
    last_sliding_depth = INFINITY
    with open("./input.txt") as depths:
        for line in depths.readlines():
            depth = int(line)
            if depth > last_depth:
                counter += 1
            sliding_window.slide(depth)
            sliding_depth = sliding_window.sum()
            if sliding_depth > last_sliding_depth:
                sliding_counter += 1
            last_depth = depth
            last_sliding_depth = sliding_depth

    print(f"{counter} measurements are larger than the previous measurement.")
    print(f"{sliding_counter} three-measurement sliding window sums are larger than the previous sum.")
