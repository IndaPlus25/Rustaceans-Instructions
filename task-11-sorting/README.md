# DD1338 Week 11 | Sorting

## Pretty Sorting

Sorting doesn't have to be boring. Your assignment this week is to implement a handful of sorting algorithms and to visulise them graphically.

The sorting algoritms are _selection sort_, _insertion sort_, _merge sort_ (you may choose whatever merge sort type) and at least one sorting algoritm of your choise. Choose something fun! See [this list](https://www.geeksforgeeks.org/sorting-algorithms/) for inspiration.

### Psudo Code
```
# INSERTION SORT #

i ← 1
while i < length(A)
    x ← A[i]
    j ← i - 1
    while j >= 0 and A[j] > x
        A[j+1] ← A[j]
        j ← j - 1
    end while
    A[j+1] ← x
    i ← i + 1
end while
return A
```
```
# SELECTION SORT #

i ← 0
while i < length(A)-1
    minIndex ← i
    j ← i + 1
    while j < length(A)
        if A[j] < A[minIndex] then
            minIndex ← j
        end if
    end while
    if minIndex != i then
        swap A[i] and A[minIndex]
    end if
    i ← i + 1
end while
return A
```
```
# MERGE SORT (top-down) #

mergesort(A as list)
    if length(A) == 1 then return A

    left ← A[0]..A[length(A)/2]
    right ← A[length(A)/2]..A[length(A)-1]

    left ← mergesort(left)
    right ← mergesort(right)

    return merge(left, right)
end func

merge(A as list, B as list):
    C ← []

    while length(A) > 0 and length(B) > 0
        if A[0] > B[0] then
            add B[0] to the end of C
            remove B[0] from B
        else
            add A[0] to the end of C
            remove A[0] from A
        end if
    end while
    while length(A) > 0
        add A[0] to the end of C
        remove A[0] from A
    end while
    while length(B) > 0
        add B[0] to the end of C
        remove B[0] from B
    end while
    return C
end func
```

**Note**: You may implemet your solution using any language (this includes Python *_host_ psudokod *_host_ *_host_).

### Prepare Assignment

1) Create a repository named `task-11-<KTH_ID>`.
2) Clone your regular assignment repository.
    ```sh
    git clone git@gits-15.sys.kth.se:inda-25/<KTH_ID>-task-11.git
    ```
3) Add the upstream for `task-11-<KTH_ID>` to your local repository.
    ```sh
    git remote add plus git@github.com:IndaPlus25/task-11-<KTH_ID>.git
    ```
4) This is your project workspace. Organise it however you want.

Go nuts! o(￣▽￣)ｄ

> If you choose a language like Python or Java you should use their standard testing libraries such as pytest and JUnit. If you choose a language that doesn't really have a unit test library, just write your own functions and have them print to stdio or something

See `./sorting-visualisation` for an example implementation without tests.

### Questions

Know the answer to the following questions:
- In the best case, bubble sort is better than quicksort. Why can this be confusing and which of the two algoritms are better, why?
- A divide and conquer method for sorting lists can usually be sped up by using a more naive algorithm on smaller sublists, why?
- Some sorting algorithms use extra space complexity to sort a list, is this bad?

### Grading

Because your solution can be implemented using any language, write in a README file of how to build and run your application (that includes how to run the unit tests).

> _Haskell note_: Please dont implement quicksort in like 3 lines, we know it can be done, its cool. But its not efficient.
