#include <stdio.h>

main()
{
  float fahr, celsius;
  float lower, upper, step;  
  
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
    celsius = (5.0/9.0) * (fahr-32.0);
    printf("%3.0f %6.1f\n", fahr, celsius);
    fahr += step;
  }
}
