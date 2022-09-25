#!/bin/sh

echo -e "loop_forever\n" > breathe.txt
for i in {0..255}
do
	echo -e "color:$i;$i;$i\nsleep:10" >> breathe.txt
done
for i in {255..0}
do
	echo -e "color:$i;$i;$i\nsleep:10" >> breathe.txt
done


echo -e "loop_forever\n" > wave.txt
for i in {0..255}
do
	echo -e "color:$i;$i;$i,region:left\nsleep:5" >> wave.txt
done

for i in {255..0}
do
    echo -e "color:$i;$i;$i,region:left\nsleep:5" >> wave.txt
    echo -e "color:$(expr 255 - $i);$(expr 255 - $i);$(expr 255 - $i),region:middle\nsleep:5" >> wave.txt
done

for i in {255..0}
do
    echo -e "color:$i;$i;$i,region:middle\nsleep:5" >> wave.txt
    echo -e "color:$(expr 255 - $i);$(expr 255 - $i);$(expr 255 - $i),region:right\nsleep:5" >> wave.txt
done

for i in {255..0}
do
    echo -e "color:$i;$i;$i,region:right\nsleep:5" >> wave.txt
done


echo -e "loop_forever\n" > rgb-wave.txt
for i in {0..255}
do
    echo -e "color:$i;0;0,region:left\nsleep:5" >> rgb-wave.txt
done

for i in {255..0}
do
    echo -e "color:$i;0;0,region:left\nsleep:5" >> rgb-wave.txt
    echo -e "color:0;$(expr 255 - $i);0,region:middle\nsleep:5" >> rgb-wave.txt
done

for i in {255..0}
do
    echo -e "color:0;$i;0,region:middle\nsleep:5" >> rgb-wave.txt
    echo -e "color:0;0;$(expr 255 - $i),region:right\nsleep:5" >> rgb-wave.txt
done

for i in {255..0}
do
    echo -e "color:0;0;$i,region:right\nsleep:5" >> rgb-wave.txt
done
