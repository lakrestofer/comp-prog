#include <stdio.h>

int main() {
  int c; // the chars
  int nc, nwhite, nother; // counters
  int ndigit[10]; // counter of numbers
  
  // init variables
  nc = nwhite = nother = 0;
  for (int i = 0; i < 10; ++i) ndigit[i] = 0;
  
  while ((c = getchar()) != EOF) {
    if (c >= '0' && c <= '9') ++ndigit[c-'0'];
    else if (c == ' ' || c  == '\n' || c =='\t') ++nwhite;
    else ++nother;
  }
  
  printf("n_chars: %d\n", nc);
  printf("n_white: %d\n", nwhite);
  printf("n_other: %d\n", nother);

  printf("n_numbers: [");
  for (int i = 0; i < 10; ++i) printf("%d,", ndigit[i]);
  printf("]\n");
  return 0;
}
