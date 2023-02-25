#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#define WIDTH 10
#define HEIGHT 20

typedef struct Game {
  char data[112];
} Game;

Game new(int width, int height);

int* draw(Game* game);
void tick(Game* game);
void hard_drop(Game* game);
void store(Game* game);
void left(Game* game);
void right(Game* game);
void down(Game* game);
void rotate_right(Game* game);
void rotate_left(Game* game);