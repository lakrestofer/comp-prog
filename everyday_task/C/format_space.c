#include <stdio.h>

main() {
  int c; // input character
  
  while ((c = getchar()) != EOF) {

    if (c == ' ') {
      // whait until
      while ((c = getchar()) == ' ');
      putchar(' '); // print the one space      
    } 

    putchar(c);
  }
}