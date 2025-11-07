%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
% DD1337 Programmering | Task 9 
% - Solution Example
%
% Originally: DD1337 HT-22 Programming, Task 10
% Author: Jonathan Blomlöf <jblomlof@kth.se>
%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%

%It was a very slow boi. But I did some changes. My thought process was as follow

% A way to make this faster is to terminate every search that steps on a tile we already searched for
% E.g if we taken path [[2,2], [2,3].....] (path-1)
% and we are now searching the path [[3,3], [2,3]] we can realize that our path that we are on is now doomed to fail
% if path-1 also failed. We can only search paths that path-1 already searched or search squares path-1 already tried to search.
% So lets save every path we done and compare if our newest square is in any of the paths.
% Bonus: I dont even need to keep track where in what direction i cant go anymore.
% I cant make it work (well it works but not optimized). It cant return the paths it took, well, because its just false.


% Now a Very fast boi
% Using memory to remember squares it doesnt like


% Taken and modified from https://gist.github.com/MuffinTheMan/7806903
% Get an element from a 2-dimensional list at (Row,Column)
% using 1-based indexing.
nth1_2d(Row, Column, List, Element) :-
    nth1(Row, List, SubList),
    nth1(Column, SubList, Element).

/**
 * @form load_board(BoardFileName, Board)
 * @constraints
 *     @unrestricted Board
 *     @ground BoardFileName
 * @descr Reads a file and retrieves the Board from it.
*/
load_board(BoardFileName, Board):-
    see(BoardFileName),     % Loads the input-file
    read(Board),            % Reads the first Prolog-term from the file
    seen.                   % Closes the io-stream

% Able to run check_alive easier given we use a .txt file for board.
run_pr(Row,Column, FileName):- % with file name
load_board(FileName, Board),
check_alive(Row, Column, Board).

% Checks whether the group of stones connected to
% the stone located at (Row, Column) is alive or dead.
check_alive(Row, Column, Board):-
    nth1_2d(Row, Column, Board, Stone),
    \+(Stone = e), % input validition i guess
     len_of_list(Board, MaxRow),
    MaxRow1 is MaxRow + 1,
    len_of_column_helper(Board, FirstRow),
    len_of_list(FirstRow, MaxColumn),
    MaxColumn1 is MaxColumn + 1,
    exit_exists(Row, Column, MaxRow1, MaxColumn1, Board, Stone, [], Moves),
    !,
    contains(0, Moves).

% Searches all possible branching that can happen and if they are pathes to e or not.
% Returns a List ReturnVisited that contains pairs of numbers. If there is a path to and empty cell
% There is a 0 in the ReturnVisited list. Thus we check for it at the end of check_alive().
exit_exists(Row, Column, MaxRow, MaxColumn, Board, Colour, Visited, ReturnVisited):-
      (
        \+ already_searched([Row, Column], Visited),
        Row > 0,
        Column > 0,
        Row < MaxRow,
        Column < MaxColumn,
        add_to_visited([Row, Column], Visited, NewVisited),
        nth1_2d(Row,Column, Board, Stone),
    % Thanks to this thread i know how to branch https://stackoverflow.com/questions/1775651/whats-the-operator-in-prolog-and-how-can-i-use-it
        (
        Stone = Colour -> (
            Row1 is Row + 1,
            Row2 is Row - 1,
            Column1 is Column + 1,
            Column2 is Column - 1,
                (
                exit_exists(Row1, Column, MaxRow, MaxColumn, Board, Colour, NewVisited, FirstVisited),
                exit_exists(Row2, Column, MaxRow, MaxColumn, Board, Colour, FirstVisited, SecondVisited),
                exit_exists(Row, Column1, MaxRow, MaxColumn, Board, Colour, SecondVisited, ThirdVisited),
                exit_exists(Row, Column2, MaxRow, MaxColumn, Board, Colour, ThirdVisited, ReturnVisited)
                )
            )
        ; Stone = e -> (list_concat([0], Visited, ReturnVisited), !)
        )
    )
    ;list_concat([], Visited, ReturnVisited). % something failed

% Gives length of either a row, or column. Need helper func for column.
len_of_column_helper([A|_], A).
len_of_list([],0). % "inspired by"(read copied) https://www.geeksforgeeks.org/lists-in-prolog/#:~:text=Operations%20on%20Prolog%20Lists%3A%201%201.%20Find%20the,an%20element%20X%20from%20a%20list%20L%20
len_of_list([_|L], N):-
    len_of_list(L, N1),
    N is N1 + 1.

% Adds L1 to end of L2
list_concat([],L,L). % taken from same as len_of_list
list_concat([X1|L1],L2,[X1|L3]) :- list_concat(L1,L2,L3).

%True if Element A is in List
contains(A, [A|_]).
contains(A, [_|List]):-
    contains(A, List).

% Pairs contains coord [x,y]. If X, Y is already a pair this is true.
% It doesnt work with out brackets for first arg??
% "already_searched(X, Y, [[X|Y]|_])."  <--- Doesnt work???

already_searched([X|Y], [[X|Y]|_]).
already_searched([X|Y], [_|Pairs]):-
    already_searched([X|Y], Pairs).

%Like list_concat. But I did this before i Know that was a thing
add_to_visited(Pair, Visited, [Pair|Visited]).