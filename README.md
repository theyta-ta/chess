calling it a day here.

currently implementing checks (and therefore also checkmate) and castling.
all i really need to do is 

havent added castling, checks, checkmate, algebraic notation input,
en-passant, 50-move rule, or ex/importing FEN/PGN yet.

that is probably the order ill add them in though.

theres also probably lotsa bugs :D



i am personally currently counting (S)LOC with 
`cat src/* | sed -r '/^\s*.\s*$/d' | sed -r '/^\s*\/\//d' | wc -l`
if anyone has any other simple ways of counting LOC in the commandline w/out installing any packages, 
please share them!

(if you cant/dont want to read regex + shell, 
the command counts how many lines are in the src/ directory that are 
>1 char long 
and do not start with `//`.

obviously this still counts block comments, but oh well!
i will pipe the sed into awk to remove them... once i can be bothered.
that will probably be added to the command soon. when im bored in a lecture probably.
