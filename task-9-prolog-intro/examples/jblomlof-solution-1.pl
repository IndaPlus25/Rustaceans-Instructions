%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
% DD1337 Programmering | Task 9 
% - Solution Example
%
% Originally: DD1337 HT-22 Programming, Task 10
% Author: Jonathan Blomlöf <jblomlof@kth.se>
%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%

% This is very slow. My a* algorithm checks stupid paths and can overlap very easily.
% However i cant be bothered to fix it so i wont.
% On the the other hand its mostly just slow when a group is dead, otherwise quite fast for large boards.
% To fix it I think I need some memory to see paths i have already taken. Because the problem rn is that it checks
% so many tree paths which are basically the same.




% Taken and modified from https://gist.github.com/MuffinTheMan/7806903
% Get an element from a 2-dimensional list at (Row,Column)
% using 1-based indexing.
nth1_2d(Row, Column, List, Element) :-
    nth1(Row, List, SubList),
    nth1(Column, SubList, Element).

% Reads a file and retrieves the Board from it.
load_board(BoardFileName, Board):-
    see(BoardFileName),     % Loads the input-file
    read(Board),            % Reads the first Prolog-term from the file
    seen.                   % Closes the io-stream

% Able to run check_alive easier given we use a .txt file for board.
run_pr(Row,Column, FileName):- % with fle name
load_board(FileName, Board),
check_alive(Row, Column, Board).

% Checks whether the group of stones connected to
% the stone located at (Row, Column) is alive or dead.
check_alive(Row, Column, Board):-
    nth1_2d(Row, Column, Board, Stone),
    \+(Stone = e), % input validition i guess
    
    exit_exists(Row, Column, Board, Stone).

% Searches all possible branching that can happen and if they are pathes to e or not.
exit_exists(Row, Column, Board, Colour):-
    Row > 0,
    Column > 0,
    len_of_list(Board, MaxRow),
    MaxRow1 is MaxRow + 1,
    Row < MaxRow1,
    len_of_column_helper(Board, FirstRow),
    len_of_list(FirstRow, MaxColumn),
    MaxColumn1 is MaxColumn + 1,
    Column < MaxColumn1,
    nth1_2d(Row,Column, Board, Stone), % Thanks to this thread i know how to branch https://stackoverflow.com/questions/1775651/whats-the-operator-in-prolog-and-how-can-i-use-it
    (
    Stone = Colour -> 
    set_element_to_x(Row, Column, Board, NewBoard),
    Row1 is Row + 1,
    Row2 is Row -1,
    Column1 is Column + 1,
    Column2 is Column - 1,
        (
        exit_exists(Row1, Column, NewBoard, Colour);
        exit_exists(Row2, Column, NewBoard, Colour);
        exit_exists(Row, Column1, NewBoard, Colour);
        exit_exists(Row, Column2, NewBoard, Colour)
        )
    ;Stone = e -> !
    ).

set_element_to_x(1, Column, [ToReplace|CurrentBoard], [SubBoard|CurrentBoard]):-
    set_element_in_list_to_x(Column, ToReplace, SubBoard).

set_element_to_x(Row, Column, [A|CurrentBoard], [A|NewBoard]):-
    Row1 is Row-1,
    set_element_to_x(Row1, Column, CurrentBoard, NewBoard).

set_element_in_list_to_x(1, [_|L], [x|L]).
set_element_in_list_to_x(IndexPos, [A|TheList], [A|NewList]):-
    IndexPos1 is IndexPos-1,
    set_element_in_list_to_x(IndexPos1, TheList, NewList).

len_of_column_helper([A|_], A).
len_of_list([],0). % "inspired by" https://www.geeksforgeeks.org/lists-in-prolog/#:~:text=Operations%20on%20Prolog%20Lists%3A%201%201.%20Find%20the,an%20element%20X%20from%20a%20list%20L%20
len_of_list([_|L], N):-
    len_of_list(L, N1),
    N is N1 + 1.