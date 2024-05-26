#[derive(Debug)]
enum Status {
    Pending,
    Started,
    Completed,
}

#[derive(Debug)]
struct ToDo {
    task: String,
    status: Status,
}

struct ToDoManager {
    todos: Vec<ToDo>,
}

impl ToDoManager {
    fn new() -> ToDoManager {
        ToDoManager {
            todos: Vec::new(),
        }
    }

    fn add_to_do(&mut self, todo: ToDo) {
        self.todos.push(todo);
    }

    fn mark_started(&mut self, index: usize) -> Result<(), &'static str> {
        if let Some(todo) = self.todos.get_mut(index) {
            todo.status = Status::Started;
            Ok(())
        } else {
            Err("Index out of bounds")
        }
    }

    fn mark_completed(&mut self, index: usize) -> Result<(), &'static str> {
        if let Some(todo) = self.todos.get_mut(index) {
            todo.status = Status::Completed;
            Ok(())
        } else {
            Err("Index out of bounds")
        }
    }

    fn get_to_do(&self, index: usize) -> Option<&ToDo> {
        self.todos.get(index)
    }
}

fn main() {
    let mut manager = ToDoManager::new();
    let todo1 = ToDo {
        task: "Task1".to_string(),
        status: Status::Pending,
    };

    manager.add_to_do(todo1);

    match manager.mark_completed(0) {
        Ok(_) => println!("Todo marked as completed"),
        Err(err) => println!("Error: {}", err),
    }

    if let Some(cur_todo) = manager.get_to_do(0) {
        println!("Task: {}, Status: {:?}", cur_todo.task, cur_todo.status);
    } else {
        println!("Todo not found");
    }
}
