file = open("breathe.txt", "w+")
file.write("loop_forever\n")
for i in range(0, 255):
    file.write(f"color:{i};{i};{i}\nsleep:10\n")
for i in reversed(range(0, 255)):
    file.write(f"color:{i};{i};{i}\nsleep:10\n")
file.close()

file = open("wave.txt", "w+")
file.write("loop_forever\n")
for i in range(0, 255):
    file.write(f"color:{i};{i};{i}, region:left\nsleep:5\n")

for i in reversed(range(0, 255)):
    file.write(f"color:{i};{i};{i}, region:left\nsleep:5\n")
    file.write(f"color:{255-i};{255-i};{255-i}, region:middle\nsleep:5\n")

for i in reversed(range(0, 255)):
    file.write(f"color:{i};{i};{i}, region:middle\nsleep:5\n")
    file.write(f"color:{255-i};{255-i};{255-i}, region:right\nsleep:5\n")

for i in reversed(range(0, 255)):
    file.write(f"color:{i};{i};{i}, region:right\nsleep:5\n")
file.close()
