// LinkedList.h
#ifndef LINKED_LIST_H
#define LINKED_LIST_H

#include <stdexcept>
#include <vector>
#include <iomanip>
#include <iostream>

#include "Player.h"

struct Node {
private:
    Player* data; // holds the player data
    Node* next;   // points to the next node if there is one
    Node* prev;   // points to the previous node if there is one

public:
    Node();
    Node(Player* item);
    void set_prev(Node* node);
    void set_next(Node* node);
    Node* get_prev();
    Node* get_next();
    Player* get_data();
    bool has_prev();
    bool has_next();
    void clean();
    void destroy();
    ~Node();
};

class PlayerList {
private:

    Node* head;     // points to the first node in the list
    Node* tail;     // points to the last node in the list
    Node* current;  // points to the current node in the list
    size_t size;    // the current length of the list

    void add_alphabetical_1(Player* item);
    void add_alphabetical_2plus(Player* item);
    void add_alphabetical_2plus_prev(Player* item);
    void add_alphabetical_2plus_next(Player* item);

    void move_to_head();
    void move_to_tail();
    void move_to_next();
    void move_to_prev();
    void insert_after(Player* item);
    void insert_before(Player* item);

    Player* get_next();
    Player* get_prev();
    Player* get_current();

public:
    PlayerList();
    PlayerList(istream& stream);
    ~PlayerList();

    void add_alphabetical(Player* item);

    bool has_next();
    bool has_prev();
    bool is_empty();
    int get_size();

    double calc_batting_average();

    void remove_current(bool destroy = false);
    void remove_all(bool destroy = false);
    string to_string(bool reverse = false);
    void build_report(ostream& stream);

};

#endif // LINKED_LIST_H