fn main() {
  let mut todo_list = create_todo_list();

  let todo = create_todo(String::from("Bonjour"));

  todo_list.todos.push(todo);

  let fisrt_todo = &mut todo_list.todos[0];

  fisrt_todo.set_content(String::from("Hello"));
  
  println!("Status is: {}", fisrt_todo.status);
  
  fisrt_todo.set_status(true);
  
  println!("Status is: {}", fisrt_todo.status);

  println!("{:?}", todo_list)
}

#[derive(Debug)]
struct Todo {
  content: String,
  status: bool,
}

#[derive(Debug)]
struct TodoList {
  todos: Vec<Todo>,
}

impl Todo {
  fn set_content(&mut self, content: String) -> &str {
    self.content = content;
    return &self.content;
  }

  fn set_status(&mut self, status: bool) -> &bool {
    self.status = status;
    return &self.status
  }
}

fn create_todo_list() -> TodoList {
  TodoList { todos: vec![] }
}

fn create_todo(content: String) -> Todo {
  let todo = Todo {
    content,
    status: false
  };

  todo
}
