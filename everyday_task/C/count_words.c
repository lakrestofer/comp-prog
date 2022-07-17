#include <stdio.h>

#define IN 1
#define OUT 0

int main() {
  int c, nl, nw, nc, state;
  
  state = OUT;
  nl = nw = nc = 0;
  while ((c = getchar()) != EOF) {
    ++nc;
    if (c == '\n') ++nl;
    if (c == ' ' || c == '\n' || c == '\t') state = OUT;
    else if (state == OUT) {
      state = IN;
      ++nw;
    }
  }
  
  printf("\n");
  printf("n_newlines: %d\n", nl);
  printf("n_words: %d\n", nw);
  printf("n_chars: %d\n", nc);
  return 0;
}
