# add_user.py
import sqlite3
import hashlib
import getpass
import sys
from typing import Tuple, Optional

def connect_db(db_name: str = "library.db") -> sqlite3.Connection:
    """
    Create a connection to the database.
    
    Args:
        db_name (str): Name of the database file
        
    Returns:
        sqlite3.Connection: Database connection object
    """
    return sqlite3.connect(db_name)

def upsert_user(username: str, password: str, db_name: str = "library.db") -> Tuple[bool, Optional[str]]:
    """
    Add a new user to the database.
    
    Args:
        username (str): Username to add
        password (str): Password for the new user
        db_name (str): Name of the database file
        
    Returns:
        Tuple[bool, Optional[str]]: (Success status, Error message if any)
    """
    try:
        with connect_db(db_name) as conn:
            cursor = conn.cursor()
            
            # Create users table if it doesn't exist
            cursor.execute('''
                CREATE TABLE IF NOT EXISTS users (
                    username TEXT PRIMARY KEY,
                    password TEXT NOT NULL
                )
            ''')
            
            # Hash the password
            hashed_password = hashlib.sha256(password.encode()).hexdigest()
            
            cursor.execute("SELECT username FROM users WHERE username = ?", (username,))
            user_exists = cursor.fetchone() is not None
            
            if user_exists:
                # Update existing user's password
                cursor.execute(
                    "UPDATE users SET password = ? WHERE username = ?",
                    (hashed_password, username)
                )
                return True, None, True
            else:
                # Insert new user
                cursor.execute(
                    "INSERT INTO users (username, password) VALUES (?, ?)",
                    (username, hashed_password)
                )
                return True, None, False
            
    except Exception as e:
        return False, f"Unexpected error: {str(e)}"

def main():
    """Command line interface for adding a user."""
    # Get database name (optional)
    db_name = "library.db"
    if len(sys.argv) > 1:
        db_name = sys.argv[1]
    
    # Get username
    username = input("Enter new username: ").strip()
    if not username:
        print("Error: Username cannot be empty")
        return
    
    # Get password (hidden input)
    password = getpass.getpass("Enter password: ")
    
    if not password:
        print("Error: Password cannot be empty")
        return
    
    # Add/update the user
    success, error, was_updated = upsert_user(username, password, db_name)
    
    if success:
        action = "updated" if was_updated else "added"
        print(f"User '{username}' {action} successfully")
    else:
        print(f"Failed to modify user: {error}")

if __name__ == "__main__":
    main()