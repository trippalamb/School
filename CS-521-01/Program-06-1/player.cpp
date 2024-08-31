#include "Player.h"
#include <iostream>
#include <sstream>
#include <string>
#include <iomanip>

using namespace std;

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

    for (int i = 0; i < STAT_LENGTH; i++){
        this->stats[i] = 0;
    }

    this->batting_average = 0.0;
    this->on_base = 0.0;
    this->slugging = 0.0;
    this->ops = 0.0;

    this->initialized = false;
}

Player::Player(istream& stream): Player() {


    string line;
    string names[2];
    int stats[STAT_LENGTH];

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

Player::Player(const string names[2], const int stats[STAT_LENGTH]):Player() {

    this->set_all(names, stats);

}

Player& Player::operator=(const Player& other) {
    if (this != &other) {

        name_first = other.name_first;
        name_last = other.name_last;
        
        for (int i = 0; i < STAT_LENGTH; i++) {
            stats[i] = other.stats[i];
        }
        
        batting_average = other.batting_average;
        on_base = other.on_base;
        slugging = other.slugging;
        ops = other.ops;
        
        initialized = other.initialized;
        
    }
    return *this;
}

void Player::set_all(const string names[2], const int stats[STAT_LENGTH]){
    this->name_first = names[0];
    this->name_last = names[1];

    for(int i = 0; i < STAT_LENGTH; i++){
        this->stats[i] = stats[i];
    }

    this->initialized = true;

    this->calc_statistics();
}

void Player::calc_statistics(){

    double n_hits = this->n_singles + this->n_doubles + this->n_triples + this->n_home_runs;

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

string Player::to_string(){

    ostringstream oss;

    oss << fixed << setprecision(3);
    oss << setw(20) << right << (this->name_last + ", " + this->name_first) << " : ";
    oss << setw(9) << this->batting_average << setw(9) << this->ops;

    return oss.str();
}

bool Player::is_initialized(){
    return this->initialized;
}

double Player::get_batting_average(){
    return this->batting_average;
}