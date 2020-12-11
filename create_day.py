from glob import glob

day = len(glob("advent_of_code_2020/src/days/*.rs")) + 1

with open(f"advent_of_code_2020/src/days/day{day}.rs", "w") as fp:
    fp.write(
        "extern crate util;\n"
        "\n"
        "pub fn part_one() -> () {\n"
        f"\tlet lines = util::read_input({day});\n"
        "\tlet list: Vec<&str> = lines.split(\"\\n\").collect();\n"
        "}\n"
        "\n"
        "pub fn part_two() -> () {\n"
        f"\tlet lines = util::read_input({day});\n"
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

    for line in lines[day-1:-4]:
        fp.write(line)

    fp.write("\t\t" + lines[-4].strip() + ",\n")

    fp.write(f"\t\tvec![&day{day}::part_one, &day{day}::part_two]\n")

    for line in lines[-3:]:
        fp.write(line)

print(f"Added day {day}")
