# Problem 1

I used a concurrent linked list that uses the fine-grained synchronization implementation described in the book.

This implementation is efficient because it locks only the nodes necessary for insertion, deletion, or scanning.
It gaurantees that the necessary nodes for insertion and deletion are not being modified by any other thread while one thread is modifying it. Thus, there will never be leftover nodes in the list at the end of the day.

I tested my code with a single worker as well as four workers, and they successful send "Thank You" notes to all gift givers.

# Problem 2

I used a matrix for storing temperature information, where each row corresponds to a certain minute within the hour and each column corresponds to one of the eight sensors.

The report is generated at the end of the hour by sorting all values to find the min 5 and max 5. This is perfectly reasonable rather than updating the max and min as time progresses because the number of floating points we sort is just $8 \times 60$.

I tested the report generation with multiple hours of data and it correctly finds the low and high values and the largest 10 minute difference.

# Compilation Instructions

The code is set up to run $500000$ gifts with $4$ workers on problem $1$, and to run for $3$ hours on problem $2$.

C++17 is required.

To compile:
`g++ -std=c++17 -pthread main.cpp`

To run:
`./a.out`