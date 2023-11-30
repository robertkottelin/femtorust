import pyo3impl
import time

# Start the timer
start_time = time.time()

# Create the database
pyo3impl.create_database_sync()

# Insert 1000 rows
for i in range(1, 1001):
    pyo3impl.insert_data_sync(str(i), f"HELLO WORLD {i}")

# Retrieve and print 1000 rows
for i in range(1, 1001):
    value = pyo3impl.get_value_by_id_sync(str(i))
    # print(f"The value for ID {i} was {value}")

# Stop the timer and calculate elapsed time
end_time = time.time()
elapsed_time = end_time - start_time

print(f"Total execution time: {elapsed_time} seconds")
