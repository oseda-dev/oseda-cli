#include "utils/utils.h"
#include <stdio.h>

int main(int argc, char *argv[]) {
	printf("%s\n", argv[0]);

	do_thing_from_other_file();

    return 0;
}
