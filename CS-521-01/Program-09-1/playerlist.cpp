#include "PlayerList.h"

Node::Node():
    data(nullptr),
    prev(nullptr),
    next(nullptr)
{
    //do nothing else
}

Node::Node(Player* item):Node() {
    this->data = item;
}

void Node::set_prev(Node* node){
    this->prev = node;
}

void Node::set_next(Node* node){
    this->next = node;
}

Node* Node::get_prev(){
    return this->prev;
}

Node* Node::get_next(){
    return this->next;
}

Player* Node::get_data(){
    return this->data;
}

bool Node::has_prev(){
    return this->prev != nullptr;
}

bool Node::has_next(){
    return this->next != nullptr;
}

void Node::clean(){
    this->data = nullptr;
    this->prev = nullptr;
    this->next = nullptr;
}

void Node::destroy(){
    delete this->data;
    this->clean();
}

Node::~Node() {
    this->destroy();
}


PlayerList::PlayerList():
    head(nullptr),
    tail(nullptr),
    current(nullptr)
{
    this->size = 0;
}

PlayerList::PlayerList(istream& stream): PlayerList(){

    Player* player = nullptr;
    bool go_on = true;

    while(go_on) {
        player = new Player(stream);
        this->add_alphabetical(player);
        go_on = player->is_initialized();
    };
}

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
}

void PlayerList::add_alphabetical_1(Player* item){
    Player* other = this->head->get_data();

    if(other->get_sort_name() > item->get_sort_name()){
        this->current = this->head = new Node(item);
    }
    else{
        this->current = this->tail = new Node(item);
    }
}

void PlayerList::add_alphabetical_2plus(Player* item){
    Player* current = this->current->get_data();

    if(current->get_sort_name() < item->get_sort_name()){
        this->add_alphabetical_2plus_next(item);
    }
    else{
        this->add_alphabetical_2plus_prev(item);
    }
}

void PlayerList::add_alphabetical_2plus_next(Player* item){

    while(this->has_next()){
        this->move_to_next();
        if(this->current->get_data()->get_sort_name() > item->get_sort_name()){
            break;
        }
    }

    this->insert_before(item);
}

void PlayerList::add_alphabetical_2plus_prev(Player* item){

    while(this->has_prev()){
        this->move_to_prev();
        if(this->current->get_data()->get_sort_name() < item->get_sort_name()){
            break;
        }
    }

    this->insert_after(item);
}

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

double PlayerList::calc_batting_average(){
    
    Player* player = nullptr;
    double sum = 0.0;

    if(this->is_empty()){ return 0.0;}

    this->current = this->head;
    player = this->get_current();
    sum = player->get_batting_average();

    while(this->has_next()){

        player = this->get_next();
        sum += player->get_batting_average();

    }

    return sum / double(this->size);
}

bool PlayerList::has_prev(){
    if(this->is_empty()){ return false;}
    return this->current->has_prev();
};

bool PlayerList::has_next(){
    if(this->is_empty()){ return false;}
    return this->current->has_next();
};

bool PlayerList::is_empty(){
    return this->size == 0;
};

int PlayerList::get_size(){
    return this->size;
};

void PlayerList::move_to_head(){
    this->current = this->head;
};

void PlayerList::move_to_tail(){
    this->current = this->tail;
};

void PlayerList::move_to_next(){
    this->current = this->current->get_next();};

void PlayerList::move_to_prev(){
    this->current = this->current->get_prev();
};

Player* PlayerList::get_next(){
    if(this->has_next()){
        this->move_to_next();
        return this->get_current();
    }
    else{
        return nullptr;
    }
};

Player* PlayerList::get_prev(){
    if(this->has_prev()){
        this->move_to_prev();
        return this->get_current();
    }
    else{
        return nullptr;
    }
};

Player* PlayerList::get_current(){
    return this->current->get_data();
}


void PlayerList::remove_current(bool destroy){
    if(destroy) {
        this->current->destroy();
    }
    else{
        this->current->clean();
    }
}

void PlayerList::remove_all(bool destroy){
    
    if(this->is_empty()){return;}

    this->move_to_head();
    this->remove_current(destroy);

    while(this->has_next()){
        this->move_to_next();
        this->remove_current(destroy);
    }

}

string PlayerList::to_string(bool reverse){

    Player* player = nullptr;
    string s = "";

    if(this->is_empty()){return "\n";}

    player = this->get_current();
    s = player->to_string() + "\n";

    if(reverse){
        while(this->has_prev()){
            player = this->get_prev();
            s += player->to_string() + "\n";
        }
    }
    else{
        while(this->has_next()){
            player = this->get_next();
            s += player->to_string() + "\n";
        }
    }

    return s;
}


PlayerList::~PlayerList()
{
    this->remove_all(true);
}