#include<stdio.h>

int main (void) {
	char ch;
	FILE *fp;
	fp = fopen("memo.txt","r");
	if ( fp == NULL ) {
		perror("file not found");
		return 1;
	}
	printf("ILC v0.1 \n");
	while ( (ch = fgetc(fp)) != EOF) {
		printf("%c", ch);
	}
	printf("task end.");
	fclose(fp);
	return 0;
}
