import time
from crud_operations import crud_operations

def time_read_function():
    CRUD = crud_operations()
    
    start_time = time.time()
    data = CRUD.read()
    end_time = time.time()
    
    print(f"data_head: {data.head()}")  # Adjusted for YAML format
    print(f"python_read_time: {end_time - start_time:.6f}")

if __name__ == "__main__":
    time_read_function()