from uuid import uuid4
from sqlite3 import connect, Cursor
import time

FILE_NAME = "femtorust.db"

def get_value_by_id(cursor: Cursor, id: str) -> str:
    cursor.execute("SELECT value FROM data WHERE id=?", (id,))
    retrieved_sequence = cursor.fetchall()
    assert len(retrieved_sequence) == 1, "Oops, too few or too many results"
    return retrieved_sequence[0][0]

def insert_data(cursor: Cursor, id: str, value: str) -> None:
    cursor.execute("INSERT INTO data (id, value) VALUES (?, ?)", (id, value))

def build_database(cursor: Cursor) -> None:
    cursor.execute("""CREATE TABLE IF NOT EXISTS data (
                        id TEXT PRIMARY KEY, 
                        value TEXT NOT NULL, 
                        created TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP);""")

# Create database and table
DATABASE = connect(FILE_NAME, isolation_level=None)
CURSOR = DATABASE.cursor()
build_database(CURSOR)

# Start the timer
start_time = time.time()

# Insert 1000 rows
for i in range(1, 1001):
    insert_data(CURSOR, str(i), f"HELLO WORLD {i}")

# Retrieve and print 1000 rows
for i in range(1, 1001):
    value = get_value_by_id(CURSOR, str(i))
    print(f"The value for ID {i} was {value}")

# Close the database connection
DATABASE.close()

# Stop the timer and calculate elapsed time
end_time = time.time()
elapsed_time = end_time - start_time

print(f"Total execution time: {elapsed_time} seconds")
