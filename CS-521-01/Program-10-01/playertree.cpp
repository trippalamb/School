#include "PlayerTree.h"

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
// Player* Node::get_data(){
//     return this->data;
// }

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

    Player* player = nullptr;
    bool go_on = true;

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

    double sum = 0.0;
    if(this->is_empty()){ return 0.0;}

    this->move_to_root();
    
    sum = this->calc_batting_average_inner();

    return sum / double(this->size);
}

/**
 * the inner private helper method for `calc_batting_average`
 */
double PlayerTree::calc_batting_average_inner(){
    
    Player* player = nullptr;
    double sum = 0.0;

    Node* current = this->current;
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
    return this->current->data;
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
    this->remove_current(destroy);

    while(this->has_right()){
        this->move_to_right();
        this->remove_current(destroy);
    }

}

/**
 * standard to string method. returns a string representation of the player list.
 * 
 * @param reverse `true` means that the data will be written in reverse list order. 
 *                `false` means that the data will be written in list order. 
 *                defaults to `false`
 */
string PlayerTree::to_string(bool reverse){

    Player* player = nullptr;
    string s = "";

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

string PlayerTree::to_string_inOrder(){
    string str = "";
    Player* player = nullptr;
    Node* current = this->current;

    if(current->has_left()){
        this->current = current->get_left();
        str += this->to_string_inOrder();
    }

    str += current->data->to_string() + "\n";

    if(current->has_right()){
        this->current = current->get_right();
        str += this->to_string_inOrder();
    }

    return str;
}

string PlayerTree::to_string_reverseOrder(){
    string str = "";
    Player* player = nullptr;
    Node* current = this->current;

    if(current->has_right()){
        this->current = current->get_right();
        str += this->to_string_reverseOrder();
    }

    str += current->data->to_string() + "\n";

    if(current->has_left()){
        this->current = current->get_left();
        str += this->to_string_reverseOrder();
    }

    return str;
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

/**
 * standard destructor for player list
 */
PlayerTree::~PlayerTree()
{
    this->remove_all(true);
}