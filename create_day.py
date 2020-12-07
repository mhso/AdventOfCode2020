from sys import argv
import os

if len(argv) == 1:
    print("Specify a day")
    exit(0)

day = None
try:
    day = int(argv[1])
    if day < 1 or day > 25:
        raise ValueError
except ValueError:
    print("Invalid day")
    exit(0)

with open(f"advent_of_code_2020/src/days/day{day}.rs", "w") as fp:
    fp.write(
        "extern crate util;\n"
        "\n"
        "pub fn part_one() -> () {\n"
        "\tlet lines = util::read_input(6);\n"
        "\tlet list: Vec<&str> = lines.split(\"\\n\").collect();\n"
        "}\n"
        "\n"
        "pub fn part_two() -> () {\n"
        "\tlet lines = util::read_input(6);\n"
        "\tlet list: Vec<&str> = lines.split(\"\\n\").collect();\n"
        "}"
    )

open(f"advent_of_code_2020/input/day{day}.txt", "w").close()

day_file = f"advent_of_code_2020/src/days.rs"
lines = open(day_file, "r").readlines()
with open(day_file, "w") as fp:
    for line in lines[:day-1]:
        fp.write(line)

    fp.write(f"pub mod day{day};\n")

    for line in lines[day-1:-5]:
        fp.write(line)

    fp.write("\t\t" + lines[-4].strip() + ",\n")

    fp.write(f"\t\tvec![&day{day}::part_one, &day{day}::part_two]\n")

    for line in lines[-3:]:
        fp.write(line)
