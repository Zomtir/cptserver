#!/usr/bin/env python3

import argparse
import os
import re
import subprocess
from pathlib import Path

def main():
    parser = argparse.ArgumentParser(
        description="Apply schema + migrations and dump normalized schema."
    )
    parser.add_argument("from_version", type=int, help="Initial schema version")
    parser.add_argument("to_version", type=int, help="Target schema version")
    args = parser.parse_args()

    folder = Path(__file__).parent.parent / "sql"

    db_name = os.environ["CPT_TEST_DB_DATABASE"]
    db_user = os.environ["CPT_TEST_DB_USER"]
    db_password = os.environ["CPT_TEST_DB_PASSWORD"]
    db_host = os.environ.get("CPT_TEST_DB_HOST", "127.0.0.1")
    db_port = os.environ.get("CPT_TEST_DB_PORT", "3306")

    mariadb_cmd = [
        "mariadb",
        "--host", db_host,
        "--port", db_port,
        "--user", db_user,
        f"--password={db_password}",
        db_name,
    ]

    def run_sql(sql: str):
        wrapped_sql = "\n".join([
            "SET FOREIGN_KEY_CHECKS=0;",
            sql,
            "SET FOREIGN_KEY_CHECKS=1;",
        ])

        subprocess.run(
            mariadb_cmd,
            input=wrapped_sql,
            text=True,
            check=True,
        )

    def run_sql_file(path: Path):
        print(f"Applying {path.name}...")
        run_sql(path.read_text(encoding="utf-8"))

    # Get all tables
    result = subprocess.run(
        mariadb_cmd[:-1]
        + [
            "-N",
            "-e",
            f"SELECT TABLE_NAME FROM information_schema.tables WHERE TABLE_SCHEMA='{db_name}'",
            db_name,
        ],
        text=True,
        capture_output=True,
        check=True,
    )

    tables = [t for t in result.stdout.splitlines() if t]

    # Drop existing tables
    if tables:
        print(f"Dropping {len(tables)} tables...")
        sql = []
        sql.extend(f"DROP TABLE `{t}`;" for t in tables)
        run_sql("\n".join(sql))

    # Apply initial schema
    run_sql_file(folder / f"schema_{args.from_version}.sql")

    # Apply migrations
    for version in range(args.from_version + 1, args.to_version + 1):
        run_sql_file(folder / f"migrate_{version}.sql")

    # Dump normalized schema
    output = Path(f"schema_{args.from_version}_{args.to_version}.sql")

    with output.open("w", encoding="utf-8") as f:
        subprocess.run(
            [
                "mariadb-dump",
                "--host", db_host,
                "--port", db_port,
                "--user", db_user,
                f"--password={db_password}",
                "--no-data",
                "--skip-comments",
                "--skip-add-drop-table",
                db_name,
            ],
            stdout=f,
            check=True,
        )

    text = output.read_text(encoding="utf-8")
    text = re.sub(r"\)\s+ENGINE=.*?;", ");", text)
    text = re.sub(r"^\s*/\*(?:!|M!)[^\n]*$\n?", "", text, flags=re.MULTILINE)
    output.write_text(text, encoding="utf-8")

    print(f"Wrote {output}")


if __name__ == "__main__":
    main()