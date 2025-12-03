help:
    just --list

# Run the main file for a given day (day1, day2, etc.) in python
python day:
    cd {{day}} && uv run main.py

# Run the main file for a given day (day1, day2, etc.) in rust
rust day:
    cd {{day}} && cargo run main
