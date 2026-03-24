# rust-todo-app
A todo-app built with Rust.

## Usage examples
### 1. Add ToDo
Add a new ToDo into a database with following `curl` command. 

**Expected responses**
- Success: Returns `Added!`
- Failure: Returns `Error: the TODO isn't added.`

**Parameters**
- `content`: The task description.
- `deadline`: The completion date. The format must be `YYYY-MM-DD`.

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

```bash
# The ToDo which id equals "1" is deleted.
curl -X DELETE http://localhost:3000/api/todo/delete/1
```