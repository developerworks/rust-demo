#include <stdio.h>
#include <time.h>

int main () {
  struct tm sometime;  /* time broken out in detail */
  char buffer[80];
  int utc;

  sometime.tm_sec = 1;
  sometime.tm_min = 1;
  sometime.tm_hour = 1;
  sometime.tm_mday = 1;
  sometime.tm_mon = 1;
  sometime.tm_year = 1;
  sometime.tm_hour = 1;
  sometime.tm_wday = 1;
  sometime.tm_yday = 1;

  printf("Date and time: %s\n", asctime(&sometime));

  utc = mktime(&sometime);
  if( utc < 0 ) {
    fprintf(stderr, "Error: unable to make time using mktime\n");
  } else {
    printf("The integer value returned: %d\n", utc);
    strftime(buffer, sizeof(buffer), "%c", &sometime);
    printf("A more readable version: %s\n", buffer);
  }

  return 0;
}