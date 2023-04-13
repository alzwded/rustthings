scaledownc: main.c ds800.c common.h Makefile pixelio.c
	gcc -o scaledownc -O3 -DVERSION='"0.1.0"' -fopenmp `pkg-config --cflags libjpeg` main.c ds800.c pixelio.c `pkg-config --libs libjpeg` -lm

clean:
	rm -rf *.o scaledownc
