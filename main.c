#include <stdio.h>

int main (int argc,char** argv) {
	int i;
	
	for (i = 0; i< argc ;i++) 
	{
		printf("hello!%s\n", argv[i]);
	}
	return 0;
}
