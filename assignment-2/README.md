
# Problem 1

This problem is equivalent to the 100-Prisoner problem with the lightbulb in the room. The cupcake and its presence is equivalent
to the lightbulb and it being on or off. Just like how every guest must eat a cupcake, in the 100-Prisoner problem every prisoner must visit the room.

Thus, the solution is as follows:

* Pick one guest to be the leader. 
    * The leader will request a new cupcake if one is not present, and keep count of how many times they requested a cupcake.
    * If the leader has not had a cupcake, they will request one, eat it, and request another.

* All other guests will either:
    * Eat the cupcake, if they have not eaten a cupcake, and one is present.
    * Otherwise, leave the maze and try again later.

So, only one person can eat a cupcake every time the leader requests a cupcake. So, once the leader requests N cupcakes and all N cupcakes get eaten, everyone has had a cupcake.

## Proof of Correctness, Efficiency, and Evaluation

My program simulates the guests entering the maze at random by having each thread represent a guest, which continuously loops while attempting to check if the cupcake is present or not. Because the cupcake is gaurded by a lock, only one guest may be at the cupcake at once.

To achieve mutual exclusion in this problem, the simplest and most efficient way is to use a lock on the cupcake.

The leader thread keeps track of how many times it has requested a cupcake. Once that count exceeds $N$, the leader sends a signal to stop all other threads from entering the maze again.

I keep a counter that is incremented when a guest eats a cupcake for the first time. This is for validating at the end, if the counter is equal to the number of guests, all guests ate a cupcake.

I tested my solution with varying sizes of $N$. The counter was equal to the number of guests for all $N$ that I tested.

| N      | Average Runtime (over 100 runs)|
| ------ | ----------- |
| 50     | 19.20 ms    |
| 100    | 59.56 ms   |
| 250    | 174.83 ms   |

#
# Problem 2

## Solution #1

| Pros | Cons |
| ------ | ----------- |
| Guests can do what they want if they have no interest in seeing the crystal.| Guests can overcrowd door and cause chaos in determining who can go next. This would mean each guest has no idea of how long it will take to see the crystal.     |
| Minimal amount of time wasted from one guest leaving the room to another guest entering.

## Solution #2

| Pros | Cons |
| --- | --- |
| Guests can do what they want if they have no interest in seeing the crystal. | Guests must spend time flipping the sign when they enter and leave. | 
| Guests would not crowd the door because they would see if the room was open or closed at a glance | Two or more guests might see an "AVAILABLE" sign at the same time and arrive at the door at the same time, leading to a conflict. |

## Solution #3
| Pros | Cons |
| --- | --- |
| Guests waste no time between leaving the room and the next guest entering, because of the queue. | Guests cannot roam the party while waiting for the room. 
| Each guest will have somewhat of an idea of how long it will take to see the room based on the length of the queue. |

## Recommended Solution

Solution #3 is the optimal solution, because it maximizes the number of people who can visit the room by wasting no time inbetween a guest leaving and a guest entering the room. The trade-off is that guests cannot roam the party, but if they aren't interested in visiting the room, they simply do not have to get in line.

## Proof of Correctness, Efficiency, and Evaluation

My program uses a queue that is shared between all the quests that decide who gets to enter the room. Once a guest is finished visiting the room, it selects the next guest who now enters the room. The guest also decides if it wants to reenter the queue, or go back to the party. The process continues until no guests are left in the queue.

This is efficient because it allows for no wasted time inbetween a guest leaving the room and another guest entering the room.

I tested my solution with varying sizes of $N$, with a %$70$ chance to enter the room initially, and a %$20$ chance to reenter the room after visiting, allowing them to visit any number of times.

| N      | Average Runtime (over 500 runs)|
| ------ | ---------|
| 50     | 6.82 ms  |
| 100    | 28.38 ms  |
| 250    | 146.95 ms |

# Compiling and Running
Navigate to the ``assignment-2`` directory and run with `cargo run --release`.
You must use `cargo` to build this, as it relies on a dependency for the `rand`.

[Guide to install Cargo or Rust, if it is not installed.](https://doc.rust-lang.org/book/ch01-01-installation.html#installation)
