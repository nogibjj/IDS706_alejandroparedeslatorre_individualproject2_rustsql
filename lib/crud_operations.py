# Alejandro Paredes La Torre
import logging
import sqlite3
import pandas as pd

class crud_operations:
    """ Returns principal stats and summary statistics and prints a report """
    def __init__(self):
        logging.basicConfig(filename='query_log.log', level=logging.INFO, 
                    format='%(asctime)s - %(levelname)s - %(message)s')
        self.logger = logging.getLogger(__name__)
    
    def create(self,  name, country, alpha_two_code, state_province, domains, web_pages):
        conn = sqlite3.connect("./src/data/UniversitiesDB.db")
        c = conn.cursor()
        c.execute(
            """
            INSERT INTO Universities 
            (name, country, alpha_two_code, state_province, domains, web_pages) 
            VALUES (?, ?, ?, ?, ?, ?)
            """,
            ( name, country, alpha_two_code, state_province, domains, web_pages),
        )
        conn.commit()
        conn.close()
        pass

    def read(self):
        conn = sqlite3.connect("./src/data/UniversitiesDB.db")
        query = "SELECT * FROM Universities"
        try:
            result = pd.read_sql_query(query, conn)
            self.logger.info("Successfully read table Universities ")
        except Exception as EXP:
            self.logger.error(f"Error reading table Universities: {EXP}")
            result = None
        finally:
            conn.close()
        return result

    def update(self, record_id, name, country, alpha_two_code, state_province, domains, web_pages):
        conn = sqlite3.connect("./src/data/UniversitiesDB.db")
        c = conn.cursor()
        
        query = """
        UPDATE Universities 
        SET name=?, country=?, alpha_two_code=?, state_province=?, domains=?, web_pages=? 
        WHERE id=?
        """
        
        c.execute(query, (name, country, alpha_two_code, state_province, domains, web_pages, record_id))
        conn.commit()
        conn.close()

    def delete(self, id):
        conn = sqlite3.connect("./src/data/UniversitiesDB.db")
        c = conn.cursor()
        c.execute("DELETE FROM Universities WHERE id=?", (id,))
        conn.commit()
        conn.close()
    
    def general_query(self, query):
        conn = sqlite3.connect("./src/data/UniversitiesDB.db")
        try:
            result = pd.read_sql_query(query, conn)
            self.logger.info(f"Successfully executed query: {query}")
        except Exception as e:
            self.logger.error(f"Error executing query '{query}': {e}")
            result = None
        finally:
            conn.close()
        return result