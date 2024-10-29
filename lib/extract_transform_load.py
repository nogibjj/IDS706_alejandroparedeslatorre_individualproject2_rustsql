import requests
import pandas as pd
import sqlite3


class ETL:
    """Extracts data from an API, transforms it, and loads it into a pandas DataFrame."""

    def __init__(self):
        self.df = pd.DataFrame()

    def extract(self, url="http://universities.hipolabs.com/search?country=United+States"):
        response = requests.get(url)
        if response.status_code == 200:
            data = response.json()
            self.df = pd.DataFrame(data)
            print(self.df.head())
            print("Data extracted successfully.")
        else:
            print(f"Failed to fetch data. Status code: {response.status_code}")
        
    def transform(self):
        print("Transforming data...")
        pass

    def load(self):
        # Establish a connection to the SQLite database
        conn = sqlite3.connect("./src/data/UniversitiesDB.db")
        
        # Convert lists and dicts to strings
        self.df = self.df.map(lambda x: str(x) if isinstance(x, (list, dict)) else x)
        
        # Create a new table with an auto-incrementing primary key
        conn.execute("""
            CREATE TABLE IF NOT EXISTS Universities (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT,
                country TEXT,
                alpha_two_code TEXT,
                state_province TEXT,
                domains TEXT,
                web_pages TEXT
            )
        """)
        
        # Insert data from the DataFrame into the new table
        for _, row in self.df.iterrows():
            conn.execute("""
                INSERT INTO Universities (name, country, alpha_two_code, state_province, domains, web_pages)
                VALUES (?, ?, ?, ?, ?, ?)
            """, (row['name'], row['country'], row['alpha_two_code'], row['state-province'], row['domains'], row['web_pages'])
            )
        
        # Commit the transaction
        conn.commit()
        
        # Close the connection
        conn.close()
        print("Data loaded into UniversitiesDB.db")

        # Reopen the connection to verify the data
        conn = sqlite3.connect("./src/data/UniversitiesDB.db")
        query = "SELECT * FROM Universities"
        result = pd.read_sql_query(query, conn)
        print(result.head())
        conn.close()


if __name__ == "__main__":
    e_t_l = ETL()
    e_t_l.extract()
    e_t_l.transform()
    e_t_l.load()



#stage maker labs
#dukecontainers
