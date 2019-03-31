/*
issue #26
https://github.com/AngoraFuzzer/Angora/issues/26

support asan
*/

#include <stdio.h>
#include <stdlib.h>

int main(int argc, char* argv[]){
	char content[1024];
	if (argc < 2){
		printf("usage: test input_sample\n");
		return -1;
	}
	FILE *file = fopen(argv[1], "r");
	size_t n_read = fread(content, 1, 1024, file);
	if (n_read < 1000){
		printf("sample too short\n");
		fclose(file);
		return -1;
	}
	if (content[0] == 'h' && content[1] == 'e'){
		if (content[2] >75){
			if (content[3] < 80)
				printf("well done!");
		}
		else if(content[2] == 74){
			//triggers a crash			
			*(content+1025*sizeof(char)) = 'r';
		}
	}
	fclose(file);
	return 0;
} 