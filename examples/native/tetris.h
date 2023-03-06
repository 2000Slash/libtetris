#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#define WIDTH 10
#define HEIGHT 20

void init();

int create_game();
void tick(int id);
int* draw(int id);
void hard_drop(int id);
void store(int id);
void left(int id);
void right(int id);
void down(int id);
void rotate_right(int id);
void rotate_left(int id);
void destroy_game(int id);