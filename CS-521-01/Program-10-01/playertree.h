// LinkedList.h
#ifndef BINARY_TREE_H
#define BINARY_TREE_H

#include <stdexcept>
#include <vector>
#include <iomanip>
#include <iostream>

#include "Player.h"

struct Node {
private:
    Node* right;   // points to the right node if there is one
    Node* left;   // points to the left node if there is one

public:
    Player* data; // holds the player data
    Node();
    Node(Player* item);
    void add(Player* item);
    void set_left(Node* node);
    void set_right(Node* node);
    Node* get_left();
    Node* get_right();
    Node* search(string name, Node*& parent);
    Player* get_data();
    bool has_left();
    bool has_right();
    bool has_only_one_child();
    Node* find_min();
    void replace_child(Node* old_child, Node* new_child);
    Node* get_single_child();
    bool is_leaf();
    string to_string_tree(string indent);
    void clean();
    void destroy();
    ~Node();
};

class PlayerTree {
private:

    Node* root;     // points to the root node of the tree
    Node* current;  // points to the current node in the list
    size_t size;    // the current size of the tree

    void move_to_root();
    void move_to_right();
    void move_to_left();
    void insert_right(Player* item);
    void insert_left(Player* item);

    double calc_batting_average_inner();
    string to_string_inOrder();
    string to_string_reverseOrder();
    Player* get_right();
    Player* get_left();
    Player* get_current();
    void remove_all_inner(bool destroy = false);
    void remove_leaf_node(Node* to_remove, Node*& parent);
    void remove_node_with_one_child(Node* to_remove, Node*& parent);
    void remove_node_with_two_children(Node* to_remove, Node*& parent);

public:
    PlayerTree();
    PlayerTree(istream& stream);
    ~PlayerTree();

    void add_alphabetical(Player* item);

    bool has_right();
    bool has_left();
    bool is_empty();
    int get_size();

    double calc_batting_average();

    void remove_current(bool destroy = false);
    void remove_all(bool destroy = false);
    bool remove_by_name(string name_first, string name_last, bool destroy=false);
    void clear();
    string to_string(bool reverse = false);
    string to_string_tree();
    void build_report(ostream& stream);

};

#endif // BINARY_TREE_H