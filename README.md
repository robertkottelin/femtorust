# FEMTORUST

This repository contains a Rust project that demonstrates the integration of Rust code into a Python application using PyO3. The `pyo3impl` shared object (`pyo3impl.so`) is the compiled result of the PyO3 implementation, allowing Rust functions to be called from Python. This shared library is the bridge between Rust and Python in this project, enabling the execution of high-performance Rust code within a Python environment.

### Building

To build the shared object file, navigate to the `pyo3impl` directory and run:

    cargo build --release

This will compile the Rust code into a shared object file located at target/release/libpyo3impl.so.

### Usage

To use the compiled libpyo3impl.so in a Python program, rename it to pyo3impl.so and ensure it is in the same directory as your Python script or in a directory listed in your PYTHONPATH. You can then import the module in Python as follows:

    import pyo3impl

From there, you can invoke the Rust-implemented functions directly in Python.
Functionality

The shared library includes the following functionalities:

    create_database_sync(): Synchronously creates a SQLite database with a predefined schema.
    insert_data_sync(id, value): Synchronously inserts data into the database.
    get_value_by_id_sync(id): Synchronously retrieves a value from the database by ID.

Dependencies

    Rust
    PyO3
    SQLx for Rust
    SQLite
