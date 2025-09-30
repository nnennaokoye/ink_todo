#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod ink_todo_list {
    use ink::storage::Mapping;
    use ink::prelude::string::String;

    #[derive(Debug, Clone, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub struct Todo {
        id: u32,
        title: String,
        description: String,
        completed: bool,
        created_at: AccountId,
    }


    #[ink(storage)]
    pub struct InkTodoList {
        todos: Mapping<u32, Todo>,
        todo_count: u32,
        owner: AccountId,
    }

    
    #[ink(event)]
    pub struct TodoCreated {
        #[ink(topic)]
        pub id: u32,
    }

    #[ink(event)]
    pub struct TodoUpdated {
        #[ink(topic)]
        pub id: u32,
    }

    #[ink(event)]
    pub struct TodoDeleted {
        #[ink(topic)]
        pub id: u32,
    }

    #[ink(event)]
    pub struct TodoCompleted {
        #[ink(topic)]
        pub id: u32,
    }


    impl Default for InkTodoList {
        fn default() -> Self {
            Self::new()
        }
    }

    impl InkTodoList {
        
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                todos: Mapping::default(),
                todo_count: 0,
                owner: Self::env().caller(),
            }
        }

       

        
        #[ink(message)]
        pub fn create_todo(&mut self, title: String, description: String) {
            let todo = Todo {
                id: self.todo_count,
                title,
                description,
                completed: false,
                created_at: self.env().caller(),
            };
            self.todos.insert(self.todo_count, &todo);
            self.env().emit_event(TodoCreated { id: self.todo_count });
            self.todo_count = self.todo_count.saturating_add(1);
        }

        #[ink(message)]
        pub fn update_todo(&mut self, id: u32, title: String, description: String) {
            if let Some(mut todo) = self.todos.get(id) {
                todo.title = title;
                todo.description = description;
                self.todos.insert(id, &todo);
                self.env().emit_event(TodoUpdated { id });
            }
        }

        #[ink(message)]
        pub fn delete_todo(&mut self, id: u32) {
            if self.todos.get(id).is_some() {
                self.todos.remove(id);
                self.env().emit_event(TodoDeleted { id });
            }
        }
       
        

        #[ink(message)]
        pub fn complete_todo(&mut self, id: u32) {
            if let Some(mut todo) = self.todos.get(id) {
                todo.completed = true;
                self.todos.insert(id, &todo);
                self.env().emit_event(TodoCompleted { id });
            }
        }

        #[ink(message)]
        pub fn get_todo(&self, id: u32) -> Option<Todo> {
            self.todos.get(id)
        }

        #[ink(message)]
        pub fn get_todo_count(&self) -> u32 {
            self.todo_count
        }

        #[ink(message)]
        pub fn get_owner(&self) -> AccountId {
            self.owner
        }
    }
}