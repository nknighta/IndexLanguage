#include <stdio.h>

int main (void) {
	char *s;
	FILE *illoader;
	illoader = fopen("main.il", "r");
	
	printf("compiling...");
	if ( illoader == NULL ) {
		perror ("not found main.il");
		return 1;
	}

	while ( (s = fgetc(illoader) )!= EOF) {
		printf("%s", s);
	}
	fclose(illoader);
	return 0;
}
