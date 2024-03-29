#include "tetris.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <unistd.h>

void main() {
    init();
    int game = create_game();
    printf("Created game id: %d", game);
    while (1==1) {
        int* vec = draw(game);
        int counter = 0;
        for (int i = 0; i < HEIGHT*WIDTH; i++) {
            if (counter % 10 == 0) {
                printf("\n");
            }
            if (vec[i] <= 0) {
                printf(" ");
            } else {
                printf("O");
            }
            counter++;
        }
        printf("\n---------------------\n");
        tick(game);
        usleep(1000);
    }
    destroy_game(game);
}
