/* * this sample demonstrates recognition of a verb/not a verb. */ 
%{
/*
*Definition Section
*/
#include<stdio.h>
#include<string.h>
%}

/*Rule Section*/
%%
[\t ]+ /* ignore whitespace */ ;
is |
am |
are |
were |
was |
be |
being |
been |
do |
does |
did |
will |
would |
should |
can |
could |
has |
have |
had |
go { printf("%s: is a verb\n", yytext); }
[a-zA-Z]+ { printf("%s: is not a verb\n", yytext); }
.|\n { ECHO; /* normal default anyway */ }
%%

int yywrap(void){}

int main()
{
// The function that starts the analysis    
  yylex();

  return 0;
}