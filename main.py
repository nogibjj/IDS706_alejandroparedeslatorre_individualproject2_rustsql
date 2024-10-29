import click
from lib.extract_transform_load import ETL
from lib.crud_operations import crud_operations


@click.group()
def cli():
    """ETL-CRUD script"""
    pass


# ETL
@cli.command()
def extract():
    """Extract data"""
    print("Extracting data...")
    e_t_l = ETL()
    e_t_l.extract()


@cli.command()
def load():
    """Load data"""
    print("Loading data...")
    e_t_l = ETL()
    e_t_l.extract()
    e_t_l.load()


# CRUD operations
CRUD = crud_operations()


@cli.command()
@click.argument("name")
@click.argument("country")
@click.argument("alpha_two_code")
@click.argument("state_province")
@click.argument("domains")
@click.argument("web_pages")
def create(name, country, alpha_two_code, state_province, domains, web_pages):
    """Create a record"""
    CRUD.create(name, country, alpha_two_code, state_province, domains, web_pages)


@cli.command()
def read():
    """Read data"""
    data = CRUD.read()
    print(data.head())


@cli.command()
@click.argument("record_id", type=int)
@click.argument("name")
@click.argument("country")
@click.argument("alpha_two_code")
@click.argument("state_province")
@click.argument("domains")
@click.argument("web_pages")
def update(
    record_id, name, country, alpha_two_code, state_province, domains, web_pages
):
    """Update a record"""
    CRUD.update(
        record_id, name, country, alpha_two_code, state_province, domains, web_pages
    )


@cli.command()
@click.argument("record_id", type=int)
def delete(record_id):
    """Delete a record"""
    CRUD.delete(record_id)


if __name__ == "__main__":
    cli()
