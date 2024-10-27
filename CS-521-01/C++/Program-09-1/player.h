#ifndef PLAYER_H
#define PLAYER_H

#include <string>

using namespace std;

const int STAT_LENGTH = 8;

string build_sort_name(string first, string last);

class Player {
private:

    bool initialized;

    string name_first;        // player first name
    string name_last;         // player last name
    string name_sort;         // name for sorting

    int stats[STAT_LENGTH];   // storing all of the base statistics data here is a requirement, also adds ease of iteration options

    //it is more readable to access the variables by name
    int& n_plate_appearances; // number of plate appearances, &stats[0]
    int& n_at_bats;           // number of at bats, &stats[1]
    int& n_singles;           // number of singles, &stats[2]
    int& n_doubles;           // number of doubles, &stats[3]
    int& n_triples;           // number of triples, &stats[4]
    int& n_home_runs;         // number of home runs, &stats[5]
    int& n_walks;             // number of walks, &stats[6]
    int& n_hit_by_pitch;      // number of hit by pitch, &stats[7]

    double batting_average;  // [%] batting average
    double on_base;          // [%] on base
    double slugging;         // [%] slugging
    double ops;              // [%] on base plus slugging

    void set_sort_name();    // sets the sort name from first and last name

public:

    Player();
    Player(istream& stream);
    Player(const string names[2], const int stats[STAT_LENGTH]);

    Player& operator=(const Player& other);

    void set_all(const string names[2], const int stats[STAT_LENGTH]);
    void calc_statistics();
    string to_string();
    string to_string_name();
    bool is_initialized();
    double get_batting_average();
    string get_sort_name();
};

#endif // PLAYER_H