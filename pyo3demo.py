import pyo3impl
from uuid import uuid4

pyo3impl.create_database_sync()
pyo3impl.insert_data_sync("2", "HELLO WORLD")
value = pyo3impl.get_value_by_id_sync("1")
print(f"The value was {value}")
