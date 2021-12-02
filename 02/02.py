from dataclasses import dataclass


@dataclass
class FirstSub:
    horizontal_position: int
    depth: int

    def helm(self, cmd: str, units: str) -> None:
        cmds = {
            "forward": self._fwd,
            "down": self._down,
            "up": self._up,
        }
        cmds[cmd](int(units))

    def _fwd(self, units: int) -> None:
        self.horizontal_position += units

    def _down(self, units: int) -> None:
        self.depth += units

    def _up(self, units: int) -> None:
        self.depth -= units

    def __str__(self) -> str:
        return str(self.depth * self.horizontal_position)


@dataclass
class SecondSub:
    aim: int
    horizontal_position: int
    depth: int

    def helm(self, cmd: str, units: str) -> None:
        cmds = {
            "forward": self._fwd,
            "down": self._down,
            "up": self._up,
        }
        cmds[cmd](int(units))

    def _fwd(self, units: int) -> None:
        self.horizontal_position += units
        self.depth += self.aim * units

    def _down(self, units: int) -> None:
        self.aim += units

    def _up(self, units: int) -> None:
        self.aim -= units

    def __str__(self) -> str:
        return str(self.depth * self.horizontal_position)


if __name__ == "__main__":
    with open("./input.txt") as commands:
        first_sub = FirstSub(0,0)
        second_sub = SecondSub(0,0,0)
        for line in commands.readlines():
            cmd, units = line.split(' ')
            first_sub.helm(cmd, units)
            second_sub.helm(cmd, units)

    print(first_sub)
    print(second_sub)
