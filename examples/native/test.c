#include "tetris.h"
#include <stdio.h>

void main() {
    init();

    int id = create_game();
    tick(id);
    draw(id);
    hard_drop(id);
    store(id);
    left(id);
    right(id);
    down(id);
    rotate_right(id);
    rotate_left(id);
    destroy_game(id);
}
