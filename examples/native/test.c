#include "tetris.h"
#include <stdio.h>

int extern get_random() {
    return 0;
}

void main() {
    Game game = new(WIDTH, HEIGHT);
    printf("Created game in: %p\n", &game);
    draw(&game);
    tick(&game);
    hard_drop(&game);
    store(&game);
    left(&game);
    right(&game);
    down(&game);
    rotate_right(&game);
    rotate_left(&game);
    printf("Called all functions. Done");
}
