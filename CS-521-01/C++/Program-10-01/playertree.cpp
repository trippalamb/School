#include "PlayerTree.h"
#include "math.h"


/**
 * Default Node constructor.
 */
Node::Node():
    data(nullptr),
    left(nullptr),
    right(nullptr)
{
    //do nothing else
}

/**
 * Standard player constructor.
 * 
 * @param item the player data
 */
Node::Node(Player* item):Node() {
    this->data = item;
}

/**
 * adds the new player the the node tree, based on name comparison
 * 
 * @param item the player data
 */
void Node::add(Player* item){

    if(this->data == nullptr){
        this->data = item;
    }
    else if(item->get_sort_name() <this->data->get_sort_name()){
        if(this->has_left()){this->left->add(item);}
        else{this->set_left(new Node(item));}
    }
    else{
        if(this->has_right()){this->right->add(item);}
        else{this->set_right(new Node(item));}
    }
}

/**
 * Standard leftious node setter.
 * 
 * @param node the node to be set to leftious
 */
void Node::set_left(Node* node){
    this->left = node;
}

/**
 * Standard right node setter.
 * 
 * @param node the node to be set to right
 */
void Node::set_right(Node* node){
    this->right = node;
}

/**
 * Standard getter for `left`.
 */
Node* Node::get_left(){
    return this->left;
}

/**
 * Standard getter for `right`.
 */
Node* Node::get_right(){
    return this->right;
}

/**
 * Standard getter for `data`.
 */
Player* Node::get_data(){
    return this->data;
}

/**
 * returns `true` if there is a leftious node.
 */
bool Node::has_left(){
    return this->left != nullptr;
}

/**
 * returns `true` if there is a right node.
 */
bool Node::has_right(){
    return this->right != nullptr;
}

/**
 * returns `true` if there is only a single child node.
 */
bool Node::has_only_one_child(){
    return this->has_left() != this->has_right();
}

/**
 * Search recursively for child node with given name.
 * 
 * @param name the name key to use
 * @param parent a reference to the parent of the returned node
 */
Node* Node::search(string name, Node*& parent){

    if(name == this->get_data()->get_sort_name()){
        return this;
    }
    else if (name < this->get_data()->get_sort_name() ){
        parent = this;
        if(this->has_left()){ return this->get_left()->search(name, parent);}
        else{ return nullptr; }
    }
    else{
        parent = this;
        if(this->has_right()){ return this->get_right()->search(name, parent);}
        else { return nullptr; }
    }
}

/**
 * Replace the matching immediate child node
 * 
 * @param old_child the child node to replace
 * @param new_child the new node to replace
 */
void Node::replace_child(Node* old_child, Node* new_child) {
    if (this->left == old_child) {
        this->left = new_child;
    } else if (this->right == old_child) {
        this->right = new_child;
    }
}

/**
 * returns left node if avaialble and the right if left is null
 */
Node* Node::get_single_child() {
    return (this->left != nullptr) ? this->left : this->right;
}

/**
 * returns true if the node has no children
 */
bool Node::is_leaf() {
    return (this->left == nullptr && this->right == nullptr);
}

/**
 * searches recursively for the minimum node and returns a pointer to it
 */
Node* Node::find_min() {
    if (left == nullptr) {
        return this;
    }
    return left->find_min();
}

/**
 * prints the node tree.
 */
string Node::to_string_tree(string indent){
    string output = indent + "|-" + this->data->to_string_name() + "\n";
    string indent_new = indent + "  ";
    if(this->has_left()){
        output += indent_new + this->left->to_string_tree(indent_new);
    }

    if(this->has_right()){
        output += indent_new + this->right->to_string_tree(indent_new);
    }

    return output;
}

/**
 * cleans out the data from node without deletion.
 */
void Node::clean(){
    this->data = nullptr;
    this->left = nullptr;
    this->right = nullptr;
}

/**
 * cleans out the data from node with deletion.
 */
void Node::destroy(){
    delete this->data;
    this->clean();
}

/**
 * Standard destructor. Wraps `destroy`.
 */
Node::~Node() {
    this->destroy();
}

/**
 * Default PlayerList constructor.
 */
PlayerTree::PlayerTree():
    root(nullptr),
    current(nullptr)
{
    this->size = 0;
}

/**
 * Construct from stream.
 * 
 * @param strean the input stream. Which is a specificly formatted list of player data
 */
PlayerTree::PlayerTree(istream& stream): PlayerTree(){

    Player* player = nullptr;  //pointer to relevant player data
    bool go_on = true; //determines when while loop will end

    while(go_on) {
        player = new Player(stream);
        
        if((go_on = player->is_initialized())) {
            this->add_alphabetical(player);
        }
        else{
            delete player;
        }
    };

}

/**
 * Generic method to add a player to the player list in alphabetical order.
 * 
 * @param item the player object to add
 */
void PlayerTree::add_alphabetical(Player* item){
    

    if(this->is_empty()){
        this->root = this->current = new Node(item);
    }
    else{
        this->current = this->root;
        this->current->add(item);
    }

    this->size++;
}

/**
 * calculates the batting average for the entire list.
 */
double PlayerTree::calc_batting_average(){

    double sum = 0.0; //holds the sum of individual batting averages
    if(this->is_empty()){ return 0.0;}

    this->move_to_root();
    
    sum = this->calc_batting_average_inner();

    return sum / double(this->size);
}

/**
 * the inner private helper method for `calc_batting_average`
 */
double PlayerTree::calc_batting_average_inner(){
    
    Player* player = nullptr; //pointer to relevant player data
    double sum = 0.0; //holds the sum of individual batting averages

    Node* current = this->current; //holds reference to original current node
    
    player = this->get_current();
    sum += player->get_batting_average();

    if(current->has_left()){
        this->current = current->get_left();
        sum += this->calc_batting_average_inner();
    }

    if(current->has_right()){
        this->current = current->get_right();
        sum += this->calc_batting_average_inner();
    }

    return sum;
}

/**
 * returns `true` if there is a node prior to `current`.
 */
bool PlayerTree::has_left(){
    if(this->is_empty()){ return false;}
    return this->current->has_left();
};

/**
 * returns `true` if there is a node after `current`.
 */
bool PlayerTree::has_right(){
    if(this->is_empty()){ return false;}
    return this->current->has_right();
};

/**
 * returns `true` if the list is empty
 */
bool PlayerTree::is_empty(){
    return this->size == 0;
};

/**
 * standard getter for `size`
 */
int PlayerTree::get_size(){
    return this->size;
};

/**
 * sets `current` to `head`
 */
void PlayerTree::move_to_root(){
    this->current = this->root;
};

/**
 * sets `current` to `current->right`
 */
void PlayerTree::move_to_right(){
    this->current = this->current->get_right();
};

/**
 * sets `current` to `current->left`
 */
void PlayerTree::move_to_left(){
    this->current = this->current->get_left();
};

/**
 * returns a reference to the right player object
 */
Player* PlayerTree::get_right(){
    if(this->has_right()){
        this->move_to_right();
        return this->get_current();
    }
    else{
        return nullptr;
    }
};

/**
 * returns a reference to the leftious player object
 */
Player* PlayerTree::get_left(){
    if(this->has_left()){
        this->move_to_left();
        return this->get_current();
    }
    else{
        return nullptr;
    }
};

/**
 * returns a reference to the current player object
 */
Player* PlayerTree::get_current(){
    return this->current->get_data();
}

/**
 * removes `current` from the list
 * 
 * @param destroy `true` means that the data in the current node will be destroyed. 
 *                `false` means that the references will be removed, but data will remain
 *                defaults to `false`.
 */
void PlayerTree::remove_current(bool destroy){
    if(destroy) {
        this->current->destroy();
    }
    else{
        this->current->clean();
    }
    delete this->current;
    this->size--;
}

/**
 * removes all nodes from the list
 * 
 * @param destroy `true` means that the data in the nodes will be destroyed. 
 *                `false` means that the references will be removed, but data will remain
 *                defaults to `false`
 */
void PlayerTree::remove_all(bool destroy){
    
    if(this->is_empty()){return;}

    this->move_to_root();
    this->remove_all_inner(destroy);
}

/**
 * inner helper method for `remove_all` nodes from the list
 * 
 * @param destroy `true` means that the data in the nodes will be destroyed. 
 *                `false` means that the references will be removed, but data will remain
 *                defaults to `false`
 */
void PlayerTree::remove_all_inner(bool destroy){
    
    Node* current = this->current; //saves reference to original current node

    if(current->has_left()){
        this->current = current->get_left();
        this->remove_all_inner(destroy);
    }


    if(current->has_right()){
        this->current = current->get_right();
        this->remove_all_inner(destroy);
    }

    this->current = current;
    this->remove_current(destroy);

}

/**
 * safely removes a node matching the first and last name arguments from the binary tree
 * 
 * @param name_first the first name of the player to remove. Capitalization doesn't matter 
 * @param name_last the last name of the player to remove. Capitalization doesn't matter 
 * @param destroy `true` means that the data in the nodes will be destroyed. 
 *                `false` means that the references will be removed, but data will remain
 *                defaults to `false`
 */
bool PlayerTree::remove_by_name(string name_first, string name_last, bool destroy){

    string name_sort = build_sort_name(name_first, name_last); //converts first and last name to the node key
    Node* parent = nullptr; //holds a pointer to the parent of the returned node
    Node* to_move = nullptr; //holds a pointer to the child of current to move
    Node* to_remove = nullptr; //holds a pointer to the child intended to be removed
    bool replace_left = false; //tells which branch of parent to replace

    this->current = this->root;
    to_remove = this->current->search(name_sort, parent);

    if(to_remove == nullptr){
        return false;
    }
    else if (to_remove->is_leaf()) {
        this->remove_leaf_node(to_remove, parent);
    }
    else if (to_remove->has_only_one_child()) {
        this->remove_node_with_one_child(to_remove, parent);
    }
    else {
        this->remove_node_with_two_children(to_remove, parent);
    }

    this->current = to_remove;
    this->remove_current(destroy);
    return true;

}

/**
 * inner routine of `remove_by_name`. logic for leaf removal
 * 
 * @param to_remove a pointer to the node to remove 
 * @param parent pointer reference to parent of `to_remove` 
 */
void PlayerTree::remove_leaf_node(Node* to_remove, Node*& parent) {
    if (parent == nullptr) {
        this->root = nullptr;
    } else {
        parent->replace_child(to_remove, nullptr);
    }
}

/**
 * inner routine of `remove_by_name`. logic for node with only a single child removal
 * 
 * @param to_remove a pointer to the node to remove 
 * @param parent pointer reference to parent of `to_remove` 
 */
void PlayerTree::remove_node_with_one_child(Node* to_remove, Node*& parent) {
    Node* child = to_remove->get_single_child(); //holds pointer to single child of the node to be removed
    if (parent == nullptr) {
        this->root = child;
    } else {
        parent->replace_child(to_remove, child);
    }
}

/**
 * inner routine of `remove_by_name`. logic node with 2 children removal
 * 
 * @param to_remove a pointer to the node to remove 
 * @param parent pointer reference to parent of `to_remove` 
 */
void PlayerTree::remove_node_with_two_children(Node* to_remove, Node*& parent) {

    Node* node_left = to_remove->get_left(); //holds reference to left node of `to_remove`
    Node* node_right = to_remove->get_right(); //holds reference to right node of `to_remove`
    Node* node_min = node_right->find_min(); //holds reference to minimum value child node of `node_right`

    if(parent == nullptr){
        this->root = node_right;   
    }
    else{
        parent->replace_child(to_remove, node_right);
    }

    node_min->set_left(node_left);
    
}

/**
 * Wrapper for `remove_all(true)`.
 * 
 **/
void PlayerTree::clear(){
    this->remove_all(true);
}

/**
 * standard to string method. returns a string representation of the player list.
 * 
 * @param reverse `true` means that the data will be written in reverse list order. 
 *                `false` means that the data will be written in list order. 
 *                defaults to `false`
 */
string PlayerTree::to_string(bool reverse){

    Player* player = nullptr; //pointer to relevant player data
    string s = ""; //holds the eventual output string

    if(this->is_empty()){return "\n";}

    this->move_to_root();
    if(reverse){
        s = this->to_string_reverseOrder();
    }
    else{
        s = this->to_string_inOrder();
    }

    return s;
}

/**
 * inner method for `to_string`. Writes the player strings in order.
 * 
 */
string PlayerTree::to_string_inOrder(){
    string str = ""; //holds the eventual output string
    Player* player = nullptr;  //pointer to relevant player data
    Node* current = this->current;  //saves pointer to current node

    if(current->has_left()){
        this->current = current->get_left();
        str += this->to_string_inOrder();
    }

    str += current->get_data()->to_string() + "\n";

    if(current->has_right()){
        this->current = current->get_right();
        str += this->to_string_inOrder();
    }

    return str;
}

/**
 * inner method for `to_string`. Writes the player strings in reverse order.
 * 
 */
string PlayerTree::to_string_reverseOrder(){
    string str = ""; //holds the eventual output string
    Player* player = nullptr;  //pointer to relevant player data
    Node* current = this->current;  //saves pointer to current node

    if(current->has_right()){
        this->current = current->get_right();
        str += this->to_string_reverseOrder();
    }

    str += current->get_data()->to_string() + "\n";

    if(current->has_left()){
        this->current = current->get_left();
        str += this->to_string_reverseOrder();
    }

    return str;
}

/**
 * prints the node tree.
 */
string PlayerTree::to_string_tree(){
    return this->root->to_string_tree("");
}

/**
 * builds and writes the player report to a given output stream.
 * 
 * @param stream the output stream to write the text to
 */
void PlayerTree::build_report(ostream& stream){

    stream << "BASEBALL TEAM REPORT --- " << this->get_size() << " PLAYERS FOUND IN FILE" << endl;
    stream << "OVERALL BATTING AVERAGE is " << fixed << setprecision(3) << this->calc_batting_average() << endl;

    stream << "    PLAYER NAME      :    AVERAGE    OPS" << endl;
    stream << "---------------------------------------------" << endl;
    stream << this->to_string() << endl;

    stream << "For testing, list in reverse order is:" << endl;
    stream << "    PLAYER NAME      :    AVERAGE    OPS" << endl;
    stream << "---------------------------------------------" << endl;
    stream << this->to_string(true);

}


int get_depth_inner(Node* current, int depth){
    int depth_left = 0;
    int depth_right = 0;
    
    if(current != nullptr){
        depth++;
    }
    if(current->has_left()){
        depth_left = get_depth_inner(current->get_left(), depth);
    }
    if(current->has_right()){
        depth_right = get_depth_inner(current->get_right(), depth);
    }

    return max(depth, max(depth_left, depth_right));
}
int PlayerTree::get_depth(){

    return get_depth_inner(this->root, 0);
}

/**
 * standard destructor for player list
 */
PlayerTree::~PlayerTree()
{
    this->clear();
}