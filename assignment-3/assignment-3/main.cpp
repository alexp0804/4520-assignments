#include <iostream>
#include <vector>
#include <random>
#include <algorithm>
#include <chrono>
#include <thread>
#include <mutex>
#include <assert.h>
#include <set>
#include <queue>
#include <iomanip>
#include <limits>

using namespace std;
using namespace std::chrono;

#define N_PRESENTS 500000
#define N_WORKERS 4

typedef struct Node {
    Node *next;
    int val;
    mutex lock;
} Node;

vector<int> pile(N_PRESENTS);
int next_idx = 0;
mutex next_idx_mutex;

set<int> added;
mutex set_mutex;

Node *head = new Node();

int acquire_next_idx() {
    int idx;

    next_idx_mutex.lock();
    idx = next_idx++;
    next_idx_mutex.unlock();

    return idx;
}

// Adds node in the sorted position
void add(int present) {
    head->lock.lock();

    Node *prev = head, *curr = prev->next;
    Node *new_node = new Node();
    new_node->val = present;

    // List is empty
    if (!curr) {
        head->next = new_node;
        head->lock.unlock();
        return;
    }

    curr->lock.lock();
    while (curr->val < present) {
        prev->lock.unlock();
        prev = curr;
        curr = curr->next;
        
        // End of list reached
        if (!curr) {
            prev->next = new_node;
            prev->lock.unlock();
            return; 
        }

        curr->lock.lock();
    }
    // Update links
    new_node->next = curr;
    prev->next = new_node;

    // Unlock
    curr->lock.unlock();
    prev->lock.unlock();
}

// Removes front of linked list
void remove() {
    // Empty list
    if (!head->next)
        return;

    Node *first = head->next;
    Node *second = first->next;

    // Lock nodes
    head->lock.lock();
    first->lock.lock();
    if (second)
        second->lock.lock();

    // Change links
    head->next = second;

    // Unlock nodes
    head->lock.unlock();
    first->lock.unlock();
    if (second)
        second->lock.unlock();
}

// Returns true if present is in the list
bool scan(int present) {
    head->lock.lock();

    Node *prev = head;
    Node *curr = head->next;

    while (curr && curr->val != present) {
        prev->lock.unlock();
        prev = curr;
        curr = curr->next;

        if (curr)
            curr->lock.lock();
    }

    head->lock.unlock();

    return (curr && curr->val == present);
}

void work() {
    while (1) {
        int idx = acquire_next_idx();

        // Still have presents to add 
        if (idx < N_PRESENTS)
            add(pile[idx]);

        // Write thank you note
        remove();

        // No more thank you notes and we added all presents. Done!
        if (head->next == nullptr && idx >= N_PRESENTS)
            break;
    }
}

void problem_1()
{
    // Problem 1
    auto start = high_resolution_clock::now();

    // Fill and shuffle pile
    iota(pile.begin(), pile.end(), 1);
    auto rng = default_random_engine {};
    shuffle(pile.begin(), pile.end(), rng);

    // Spawn workers
    vector<thread> handles;
    for (int i = 0; i < N_WORKERS; i++)
        handles.push_back(thread(work));

    // Wait for workers to be done
    for (thread & t : handles)
        if (t.joinable())
            t.join();

    // Nothing should be left in the list
    assert(head->next == nullptr);
    auto stop = high_resolution_clock::now();

    auto duration = duration_cast<milliseconds>(stop - start);
    cout << "Problem 1 took: " << duration.count() << " milliseconds" << endl;
}

#define N_THREADS 8
#define N_MINUTES 60
#define MIN_TEMP -100
#define MAX_TEMP 70

float temp_data[N_MINUTES][N_THREADS];

float random_number() {
    random_device rd;
    mt19937 gen(rd());
    uniform_real_distribution<> distr(MIN_TEMP, MAX_TEMP);

    return distr(gen);
}

void read_temp_data(int worker_id) {
    for (int i = 0; i < N_MINUTES; i++)
        temp_data[i][worker_id] = random_number();
}

void write_report(int hour) {
    vector<float> readings;

    for (int i = 0; i < N_MINUTES; i++)
        for (int j = 0; j < N_THREADS; j++)
            readings.push_back(temp_data[i][j]);            

    sort(readings.begin(), readings.end());

    cout << fixed << setprecision(2);
    cout << "Hour " << hour << " report" << endl;

    cout << "\tTop: ";
    for (int i = readings.size() - 1; i >= readings.size() - 6; i--)
        cout << readings[i] << " ";

    cout << endl << "\tBottom: ";
    for (int i = 0; i < 5; i++)
        cout << readings[i] << " ";
    cout << endl;

    float max_diff = numeric_limits<float>::min();
    for (int i = 0; i < N_MINUTES - 10; i++) {

        float *f_from = temp_data[i], *f_to = temp_data[i] + N_THREADS,
              *s_from = temp_data[i+10], *s_to = temp_data[i+10] + N_THREADS;

        max_diff = max({
                       max_diff,
                       abs(*min_element(f_from, f_to) - *max_element(s_from, s_to)),
                       abs(*max_element(f_from, f_to) - *min_element(s_from, s_to))
                   });
    }

    cout << "\tGreatest 10-min difference: " << max_diff << endl << endl;
}

void problem_2(int hours) {
    for (int hour = 0; hour < hours; hour++) {
        vector<thread> sensors;
        for (int i = 0; i < N_THREADS; i++)
            sensors.push_back(thread(read_temp_data, i));

        for (thread & t : sensors)
            if (t.joinable())
                t.join();

        // Create report
        write_report(hour+1); 
    }
}

int main()
{
    problem_1();
    cout << endl;
    problem_2(3);
}
