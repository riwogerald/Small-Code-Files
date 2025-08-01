/* * this sample demonstrates capitalization of letters */ 
%{
/*
*Definition Section i.e. header files
*/
#include<stdio.h>
#include<string.h>
%}

/*Rule Section*/
/*yytext holds the text matched by the current token */
%%
[\t ]+ /* ignore whitespace */ ;
[a-z] { printf("%c",yytext[0]-32); } /*converts to uppercase*/
[A-Z] { printf("%c",yytext[0]+32); } /*converts to lowercase*/
.|\n { ECHO; /* normal default anyway */ }
%%

int yywrap(void){ return 1; }

int main()
{
    // The function that starts the analysis    
    yylex();
    return 0;
}