%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
% DD1337 Programming | Task 9 
% - Example
% 
% Author: Isak Larsson
% Modified by: Viola Söderlund
% Last edited: 2025-11-07
%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%

% Facts
human(isak).
asse(viola).
friend(isak, viola).
friend(viola, tobbe).

% Predicate
friend_of_a_friend(A, B):-
    friend(A, X),
    friend(X, B).

% Predicate
last([H], Element):-
    Element = H.
last([_|T], Element):-
    last(T, Element).

% Predicate
% Is true if Element is the Nth element of the list
% 1-indexing
nth(1, [Element], Element).
nth(1, [H|_], Element) :-
    Element = H.
nth(N, [_|T], Element) :-
    N1 is N - 1
    nth(N1, T, Element).