%{
#include <stdio.h>
%}
%%
[a-z] {printf("%c",yytext[0]-32);}
\n    {printf("\n"); return 0;}
.     {printf("%c", yytext[0]);}
%%
int yywrap(void){
    return 1;
}

int main()
{
    printf("Enter the string: ");
    yylex();
    return 0;
}