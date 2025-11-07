%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
% DD1337 Programmering | Task 9
% - Calculating Dart Scores Example
% 
% See: https://open.kattis.com/problems/calculatingdartscores
% Author: Viola Söderlund <violaso@kth.se>, Tobias Hansson <tohans@kth.se>
% Last edited: 2021-06-07
%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%

[kattio].

main :-
    read_int(X),
    solve(X).

solve(X) :-
    between(0, 20, A),
    between(0, 20, B),
    between(0, 20, C),
    between(0, 3, Ax),
    between(0, 3, Bx),
    between(0, 3, Cx),
    X is (A*Ax + B*Bx + C*Cx),

    write_with_mult(Ax, A),
    write_with_mult(Bx, B),
    write_with_mult(Cx, C),!.

solve(_) :- write('impossible').

write_with_mult(0, _).
write_with_mult(1, Score) :- write('single '), write(Score), nl.
write_with_mult(2, Score) :- write('double '), write(Score), nl.
write_with_mult(3, Score) :- write('triple '), write(Score), nl.
