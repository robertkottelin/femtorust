from uuid import uuid4
from sqlite3 import connect

FILE_NAME = f"database_{str(uuid4())[0:5]}.db"

from sqlite3 import Cursor

def get_value_by_id(cursor: Cursor, id: str) -> str:

    cursor.execute("""SELECT value from data where id=?""", (id,))

    retrieved_sequence = cursor.fetchall()

    assert len(retrieved_sequence) == 1, "oops too few or too many results"

    return retrieved_sequence[0]

def build_database(cursor: Cursor) -> None:

    cursor.execute("""create table if not exists data(
                    id text primary key, 
                    value text not null, 
                    created timestamp not null default current_timestamp);""")

    cursor.execute("""INSERT INTO data(id,value) VALUES(?,?);""",
                   ("1", "HELLO WORLD"))


DATABASE = connect(FILE_NAME, isolation_level=None)

CURSOR = DATABASE.cursor()

build_database(CURSOR)

ID = "1"

value = get_value_by_id(id=ID, cursor=CURSOR)

print(f"the value was {value[0]}")

