#include "PlayerList.h"

/**
 * Default Node constructor.
 */
Node::Node():
    data(nullptr),
    prev(nullptr),
    next(nullptr)
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
 * Standard previous node setter.
 * 
 * @param node the node to be set to previous
 */
void Node::set_prev(Node* node){
    this->prev = node;
}

/**
 * Standard next node setter.
 * 
 * @param node the node to be set to next
 */
void Node::set_next(Node* node){
    this->next = node;
}

/**
 * Standard getter for `prev`.
 */
Node* Node::get_prev(){
    return this->prev;
}

/**
 * Standard getter for `next`.
 */
Node* Node::get_next(){
    return this->next;
}

/**
 * Standard getter for `data`.
 */
Player* Node::get_data(){
    return this->data;
}

/**
 * returns `true` if there is a previous node.
 */
bool Node::has_prev(){
    return this->prev != nullptr;
}

/**
 * returns `true` if there is a next node.
 */
bool Node::has_next(){
    return this->next != nullptr;
}

/**
 * cleans out the data from node without deletion.
 */
void Node::clean(){
    this->data = nullptr;
    this->prev = nullptr;
    this->next = nullptr;
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
PlayerList::PlayerList():
    head(nullptr),
    tail(nullptr),
    current(nullptr)
{
    this->size = 0;
}

/**
 * Construct from stream.
 * 
 * @param strean the input stream. Which is a specificly formatted list of player data
 */
PlayerList::PlayerList(istream& stream): PlayerList(){

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
void PlayerList::add_alphabetical(Player* item){
    

    if(this->is_empty()){
        this->head = this->tail = this->current = new Node(item);
    }
    else if(this->size == 1){
        this->add_alphabetical_1(item);
    }
    else{
        this->add_alphabetical_2plus(item);
    }

    this->size++;
}

/**
 * adds a player to the player list in alphabetical order for the special case there is only 1 current entries.
 * 
 * @param item the player object to add
 */
void PlayerList::add_alphabetical_1(Player* item){
    Player* other = this->head->get_data();

    if(other->get_sort_name() > item->get_sort_name()){
        this->current = this->head = new Node(item);
    }
    else{
        this->current = this->tail = new Node(item);
    }
    this->head->set_next(this->tail);
    this->tail->set_prev(this->head);
}

/**
 * adds a player to the player list in alphabetical order when there are 2 or more entries.
 * 
 * @param item the player object to add
 */
void PlayerList::add_alphabetical_2plus(Player* item){
    Player* current = this->current->get_data();

    if(current->get_sort_name() < item->get_sort_name()){
        this->add_alphabetical_2plus_next(item);
    }
    else{
        this->add_alphabetical_2plus_prev(item);
    }
}

/**
 * adds a player to the player list in alphabetical order when there are 2 or more entries and
 * the name is alphabetically after `current`.
 * 
 * @param item the player object to add
 */
void PlayerList::add_alphabetical_2plus_next(Player* item){

    while(this->has_next()){
        this->move_to_next();
        if(this->current->get_data()->get_sort_name() > item->get_sort_name()){
            this->insert_before(item);
            return;
        }
    }

    this->insert_after(item);

}

/**
 * adds a player to the player list in alphabetical order when there are 2 or more entries and
 * the name is alphabetically prior to `current`.
 * 
 * @param item the player object to add
 */
void PlayerList::add_alphabetical_2plus_prev(Player* item){

    while(this->has_prev()){
        this->move_to_prev();
        if(this->current->get_data()->get_sort_name() < item->get_sort_name()){
            this->insert_after(item);
            return;
        }
    }

    this->insert_before(item);
}

/**
 * inserts a new player to the list after `current`
 * 
 * @param item the player object to add
 */
void PlayerList::insert_after(Player* item){

    bool is_tail = !this->current->has_next();
    Node* node = new Node(item);
    Node* next = nullptr;

    if(is_tail){
        this->current->set_next(node);
        this->tail = node;
        node->set_prev(this->current);
    }
    else{
        next = this->current->get_next();
        this->current->set_next(node);
        next->set_prev(node);
        node->set_next(next);
        node->set_prev(this->current);
    }
    
    this->move_to_next();
}

/**
 * inserts a new player to the list prior to `current`
 * 
 * @param item the player object to add
 */
void PlayerList::insert_before(Player* item){

    bool is_head = !this->current->has_prev();
    Node* node = new Node(item);
    Node* prev = nullptr;

    if(is_head){
        this->current->set_prev(node);
        this->head = node;
        node->set_next(this->current);
    }
    else{
        prev = this->current->get_prev();
        this->current->set_prev(node);
        prev->set_next(node);
        node->set_prev(prev);
        node->set_next(this->current);
    }
    
    this->move_to_prev();

}

/**
 * calculates the batting average for the entire list.
 */
double PlayerList::calc_batting_average(){
    
    Player* player = nullptr;
    double sum = 0.0;

    if(this->is_empty()){ return 0.0;}

    this->move_to_head();
    player = this->get_current();
    sum = player->get_batting_average();

    while(this->has_next()){

        player = this->get_next();
        sum += player->get_batting_average();

    }

    return sum / double(this->size);
}

/**
 * returns `true` if there is a node prior to `current`.
 */
bool PlayerList::has_prev(){
    if(this->is_empty()){ return false;}
    return this->current->has_prev();
};

/**
 * returns `true` if there is a node after `current`.
 */
bool PlayerList::has_next(){
    if(this->is_empty()){ return false;}
    return this->current->has_next();
};

/**
 * returns `true` if the list is empty
 */
bool PlayerList::is_empty(){
    return this->size == 0;
};

/**
 * standard getter for `size`
 */
int PlayerList::get_size(){
    return this->size;
};

/**
 * sets `current` to `head`
 */
void PlayerList::move_to_head(){
    this->current = this->head;
};

/**
 * sets `current` to `tail`
 */
void PlayerList::move_to_tail(){
    this->current = this->tail;
};

/**
 * sets `current` to `current->next`
 */
void PlayerList::move_to_next(){
    this->current = this->current->get_next();
};

/**
 * sets `current` to `current->prev`
 */
void PlayerList::move_to_prev(){
    this->current = this->current->get_prev();
};

/**
 * returns a reference to the next player object
 */
Player* PlayerList::get_next(){
    if(this->has_next()){
        this->move_to_next();
        return this->get_current();
    }
    else{
        return nullptr;
    }
};

/**
 * returns a reference to the previous player object
 */
Player* PlayerList::get_prev(){
    if(this->has_prev()){
        this->move_to_prev();
        return this->get_current();
    }
    else{
        return nullptr;
    }
};

/**
 * returns a reference to the current player object
 */
Player* PlayerList::get_current(){
    return this->current->get_data();
}

/**
 * removes `current` from the list
 * 
 * @param destroy `true` means that the data in the current node will be destroyed. 
 *                `false` means that the references will be removed, but data will remain
 *                defaults to `false`.
 */
void PlayerList::remove_current(bool destroy){
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
void PlayerList::remove_all(bool destroy){
    
    if(this->is_empty()){return;}

    this->move_to_head();
    this->remove_current(destroy);

    while(this->has_next()){
        this->move_to_next();
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
string PlayerList::to_string(bool reverse){

    Player* player = nullptr;
    string s = "";

    if(this->is_empty()){return "\n";}

    if(reverse){
        this->move_to_tail();
        player = this->get_current();
        s = player->to_string() + "\n";
        while(this->has_prev()){
            player = this->get_prev();
            s += player->to_string() + "\n";
        }
    }
    else{
        this->move_to_head();
        player = this->get_current();
        s = player->to_string() + "\n";
        while(this->has_next()){
            player = this->get_next();
            s += player->to_string() + "\n";
        }
    }

    return s;
}

/**
 * builds and writes the player report to a given output stream.
 * 
 * @param stream the output stream to write the text to
 */
void PlayerList::build_report(ostream& stream){

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
PlayerList::~PlayerList()
{
    this->remove_all(true);
}