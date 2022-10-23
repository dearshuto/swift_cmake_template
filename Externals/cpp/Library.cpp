//
//  Use this file to import your target's public headers that you would like to expose to Swift.
//

#include <stdio.h>

extern "C" void my_print(void)
{
    printf("Hello World from C\n");
}
