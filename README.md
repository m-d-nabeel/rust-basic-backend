## Overview

This project has two parts:
- **Client UI**: The frontend interface for users.
- **Server Auth**: Backend for handling authentication.

Currently, only the client UI and server authentication are finished. More features will be added over time.

## Project Structure

```
.
├── client                # Frontend code
│   ├── index.html
│   ├── package.json
│   ├── src               # UI components
│   └── ...
└── server                # Backend code
    ├── Cargo.toml
    ├── db_init_query
    │   └── query_init.sql  # Initial database setup script
    ├── docker-compose.yml  # Docker setup
    ├── run_ins.sh          # Server initialization script
    ├── src                 # Server code
    └── ...
```

## Completed

1. **Client UI**: Basic user interface.
2. **Server Auth**: Basic user login and registration.

## To Be Done

- More API features
- UI improvements
- Database enhancements

## Running the Project

### Start the Server

1. Go to the `server` directory:
   ```bash
   cd server
   ```
2. Run the server setup script:
   ```bash
   ./run_ins.sh
   ```

### Start the Client

1. Go to the `client` directory:
   ```bash
   cd client
   ```
2. Install dependencies and start the client:
   ```bash
   npm install
   npm run dev
   ```

---

More updates will come as time allows.