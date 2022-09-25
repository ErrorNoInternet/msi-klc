file = open("breathe.txt", "w+")
for i in range(0, 255):
    file.write(f"color:{i};{i};{i}\nsleep:10\n")
for i in reversed(list(range(0, 255))):
    file.write(f"color:{i};{i};{i}\nsleep:10\n")
file.close()
