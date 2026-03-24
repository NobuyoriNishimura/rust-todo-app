# rust-todo-app
A todo-app built with Rust.

## Setup
This API uses `Docker Compose`. Please ensure that you have `Docker` installed on your PC.

### 1. Get the source code
First, execute the following comannds to get the source code in this repository, or download and unzip `ZIP` file via the green `CODE` button at top-right of this page.
```bash
git clone https://github.com/NobuyoriNishimura/rust-todo-app.git
cd rust-todo-app
```

### 2. Start the service
Then, start the service by spinning up containers for backend or database. Once you see the message `The server is ready.`, this API is available.
```bash
docker compose up --build
```

## Usage examples
### 1. Add ToDo
Add a new ToDo into a database with following `curl` command. 

**Expected responses**
- Success: Returns `Added!`
- Failure: Returns `Error: the TODO isn't added.`

**Parameters**
- `content`: The task description.
- `deadline`: The completion date. The format must be `YYYY-MM-DD`.

**Command**
```bash
curl -X POST -H "Content-Type: application/json" -d '{"content": "test!!", "deadline": "2026-04-01"}' http://localhost:3000/api/todo/add
```

### 2. Delete ToDo
Delete the specific ToDo from database with following `curl` command. 

**Expected responses**
- Success: Returns `Delete the ToDo.`
- Failure: Returns `Error: the ToDo isn't deleted.`

**Parameters**
- `id`: The unique identifier of the ToDo to be deleted. Signify it at the end of the URL.

**Command**
```bash
# The ToDo which id equals "1" is deleted.
curl -X DELETE http://localhost:3000/api/todo/delete/1
```

### 3. Check ToDo
Retrieve the list of remaining ToDos from the database with following `curl` command. 

**Expected responses**
- Success (Empty): `[]`
- Success (With data): 
```json
[
    {
        "id":2,
        "content":"test!!",
        "created_at":"2026-03-23",
        "deadline":"2026-04-01",
        "done":0
    },
    .
    .
    .
]
```
- Failure: 
```json
[
    {
        "id":-1,
        "content":"Error",
        "created_at":null,
        "deadline":null,
        "done":null
    }
]
```

**Parameters**
- No parameters are required.

**Command**
```bash
curl http://localhost:3000/api/todo/check
```