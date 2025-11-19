# Stardust

A simple and elegant movie watchlist application built with Rust, featuring a dark and crimson theme.

## Features

- Add and remove movies from your personal watchlist
- Beautiful dark and crimson themed interface
- Built with Rust, Axum, and PostgreSQL
- Handles duplicate movies gracefully
- Responsive design

## Tech Stack

- **Backend**: Rust, Axum web framework
- **Database**: PostgreSQL with UUID primary keys
- **Frontend**: Askama templates with custom CSS
- **Styling**: Dark gradient background with crimson accents
- **ORM**: deadpool-postgres for database connection pooling

## Prerequisites

- Rust (latest stable)
- PostgreSQL
- Cargo

## Setup

1. **Clone the repository**
   ```bash
   git clone <repository-url>
   cd stardust
   ```

2. **Set up the database**
   ```sql
   CREATE DATABASE stardust;
   CREATE TABLE movie (
       id UUID PRIMARY KEY UNIQUE DEFAULT (gen_random_uuid()),
       title TEXT NOT NULL UNIQUE
   );
   ```

3. **Configure database connection**
   
   Edit the database configuration in `src/main.rs`:
   ```rust
   let mut cfg = Config::new();
   cfg.dbname = Some("stardust".to_string());
   cfg.host = Some("localhost".to_string());
   cfg.port = Some(5432);
   cfg.user = Some("postgres".to_string());
   // Add password if needed: cfg.password = Some("your_password".to_string());
   ```

4. **Install dependencies**
   ```bash
   cargo build
   ```

5. **Run the application**
   ```bash
   cargo run
   ```

The server will start on `http://localhost:3000`.

## Usage

- **Add movies**: Type a movie title in the input field and click "Add Movie"
- **Remove movies**: Click the "Remove" button next to any movie in your watchlist
- **Duplicate handling**: If you try to add a movie that's already in your watchlist, you'll see a friendly message

## API Endpoints

- `GET /` - Main page with movie list
- `POST /add` - Add a new movie
- `POST /delete` - Remove a movie

## Database Schema

```sql
CREATE TABLE movie (
    id UUID PRIMARY KEY UNIQUE DEFAULT (gen_random_uuid()),
    title TEXT NOT NULL UNIQUE
);
```

## Configuration

The application uses PostgreSQL connection pooling via deadpool-postgres with the following default configuration:
- Host: localhost
- Port: 5432
- Database: stardust
- User: postgres
- No TLS encryption
- Fast connection recycling