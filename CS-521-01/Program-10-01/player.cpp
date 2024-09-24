#include "Player.h"
#include <iostream>
#include <sstream>
#include <string>
#include <iomanip>

using namespace std;

/**
 * Default constructor for player.
 */
Player::Player():
    n_plate_appearances(this->stats[0]),
    n_at_bats(this->stats[1]),
    n_singles(this->stats[2]),
    n_doubles(this->stats[3]),
    n_triples(this->stats[4]),
    n_home_runs(this->stats[5]),
    n_walks(this->stats[6]),
    n_hit_by_pitch(this->stats[7])
{

    this->name_first = "unknown";
    this->name_last = "unknown";
    this->set_sort_name();

    for (int i = 0; i < STAT_LENGTH; i++){
        this->stats[i] = 0;
    }

    this->batting_average = 0.0;
    this->on_base = 0.0;
    this->slugging = 0.0;
    this->ops = 0.0;

    this->initialized = false;
}

/**
 * Construct from stream.
 * 
 * @param stream input stream to specific data format file
 */
Player::Player(istream& stream): Player() {


    string line; //individual line from input
    string names[2]; // array to hold first and last name
    int stats[STAT_LENGTH]; // individual primary stats from input, see header information for further details

    getline(stream, line);

    if (!line.empty()) {
        istringstream iss(line);

        iss >> names[0] >> names[1];

        for (int i = 0; i < STAT_LENGTH; ++i) {
            iss >> stats[i];
        }

        this->set_all(names, stats);
    }
    
}

/**
 * Construct from values.
 * 
 * @param names player first and last names
 * @param stats specific stat ordering to set player stats
 */
Player::Player(const string names[2], const int stats[STAT_LENGTH]):Player() {

    this->set_all(names, stats);

}

/**
 * Assignment override.
 * 
 * @param other the player data to be assigned to this player data
 */
Player& Player::operator=(const Player& other) {
    if (this != &other) {

        this->name_first = other.name_first;
        this->name_last = other.name_last;
        this->name_sort = other.name_sort;
        
        for (int i = 0; i < STAT_LENGTH; i++) {
            this->stats[i] = other.stats[i];
        }
        
        this->batting_average = other.batting_average;
        this->on_base = other.on_base;
        this->slugging = other.slugging;
        this->ops = other.ops;
        
        this->initialized = other.initialized;
        
    }
    return *this;
}

/**
 * sets a name variable to be used in alphabetical sorting. Requires `name_first` and `name_last` to already be set
 */
void Player::set_sort_name(){
    this->name_sort = build_sort_name(this->name_first, this->name_last);
}



/**
 * Sets all the values for player. This exists to merge functionality of all constructors.
 * 
 * @param names player first and last names
 * @param stats specific stat ordering to set player stats
 */
void Player::set_all(const string names[2], const int stats[STAT_LENGTH]){
    this->name_first = names[0];
    this->name_last = names[1];
    this->set_sort_name();

    for(int i = 0; i < STAT_LENGTH; i++){
        this->stats[i] = stats[i];
    }

    this->initialized = true;

    this->calc_statistics();
}

/**
 * Sets all the derived statistics for player. Requires `stats` to already be set.
 */
void Player::calc_statistics(){

    double n_hits = this->n_singles + this->n_doubles + this->n_triples + this->n_home_runs; //total number of hits

    this->batting_average = n_hits / double(this->n_at_bats);

    this->on_base = (n_hits + double(this->n_walks + this->n_hit_by_pitch))/double(this->n_plate_appearances);

    this->slugging = double(
            this->n_singles + 
        2 * this->n_doubles + 
        3 * this->n_triples + 
        4 * this->n_home_runs
    )/double(this->n_at_bats);

    this->ops = this->on_base + this->slugging;

}

/**
 * Standard to String method. Converts the player object to a string.
 */
string Player::to_string(){

    ostringstream oss; // temporary output stream

    oss << fixed << setprecision(3);
    oss << setw(20) << right << (this->name_last + ", " + this->name_first) << " : ";
    oss << setw(9) << this->batting_average << setw(9) << this->ops;

    return oss.str();
}

/**
 * Returns `true` if the player was constructed successfully. Useful for determining if the stream
 * had valid data.
 */
bool Player::is_initialized(){
    return this->initialized;
}

/**
 * standard getter for `batting_average`.
 */
double Player::get_batting_average(){
    return this->batting_average;
}

/**
 * getter for `name_sort`
 */
string Player::get_sort_name(){
    return this->name_sort;
}

string build_sort_name(string first, string last){
    string full = last + first;
    for(int i = 0; i < full.length(); i++){
        full[i] = tolower(full[i]);
    }
    return full;
}