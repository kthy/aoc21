if __name__ == "__main__":
    counter = 0
    last_depth = float('inf')
    with open("./input/depths.txt") as depths:
        for line in depths.readlines():
            depth = int(line)
            if depth > last_depth:
                counter += 1
            last_depth = depth

    print(counter)
