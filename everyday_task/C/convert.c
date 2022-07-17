#include <stdio.h>

main()
{
  int fahr, celsius;
  int lower, upper, step;  
  
  printf("we first print the convertions without right-justification");
  lower = 0;
  upper = 300;
  step = 20;
  
  fahr = lower;
  while (fahr <= upper) {
    celsius = 5 * (fahr-32) / 9;
    printf("%d %d\n", fahr, celsius);
    fahr += step;
  }
  printf("Then we do!");
  lower = 0;
  upper = 300;
  step = 20;
  
  fahr = lower;
  while (fahr <= upper) {
    celsius = 5 * (fahr-32) / 9;
    printf("%3d %6d\n", fahr, celsius);
    fahr += step;
  }
}
