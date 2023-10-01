use std::{rc::Rc, cell::RefCell};

use serde_json::Map;

use crate::{callstack::{CallStack, Thread}, choice::Choice, object::RTObject, container::Container, json_write_state, json_read};

#[derive(Clone)]
pub struct Flow {
    pub name: String,
    pub callstack: Rc<RefCell<CallStack>>,
    pub output_stream: Vec<Rc<dyn RTObject>>,
    pub current_choices: Vec<Rc<Choice>>
}

impl Flow {
    pub fn new(name: &str, main_content_container: Rc<Container>) -> Flow {
        Flow { 
            name: name.to_string(),
            callstack: Rc::new(RefCell::new(CallStack::new(main_content_container))),
            output_stream: Vec::new(),
            current_choices: Vec::new()
        }
    }

    pub fn from_json(name: &str, main_content_container: Rc<Container>, j_obj: &Map<String, serde_json::Value>) -> Result<Flow, String> {
        let mut flow = Self { 
            name: name.to_string(),
            callstack: Rc::new(RefCell::new(CallStack::new(main_content_container.clone()))),
            output_stream: json_read::jarray_to_runtime_obj_list(j_obj.get("outputStream").ok_or("outputStream not found.")?.as_array().unwrap(), false)?,
            current_choices: json_read::jarray_to_runtime_obj_list(j_obj.get("currentChoices").ok_or("currentChoices not found.")?.as_array().unwrap(), false)?.iter().map(|o| o.clone().into_any().downcast::<Choice>().unwrap()).collect::<Vec<Rc<Choice>>>(),
        };

        flow.callstack.borrow_mut().load_json(&main_content_container, j_obj.get("callstack").ok_or("loading callstack")?.as_object().unwrap())?;
        let j_choice_threads = j_obj.get("choiceThreads").ok_or("loading choice threads")?;

        flow.load_flow_choice_threads(j_choice_threads, main_content_container)?;

        Ok(flow)
    }

    pub(crate) fn write_json(&self) -> serde_json::Value {
        let mut flow: Map<String, serde_json::Value> = Map::new();

        flow.insert("callstack".to_owned(), self.callstack.borrow().write_json());
        flow.insert("outputStream".to_owned(), json_write_state::write_list_rt_objs(&self.output_stream));
        
        // choiceThreads: optional
        // Has to come BEFORE the choices themselves are written out
        // since the originalThreadIndex of each choice needs to be set
        let mut has_choice_threads = false;
        let mut jct: Map<String, serde_json::Value> = Map::new();
        for c in self.current_choices.iter() {
            // c.original_thread_index = c.get_thread_at_generation().unwrap().thread_index;
            let original_thread_index = match c.get_thread_at_generation() {
                Some(t) => Some(t.thread_index),
                None => None,
            }.unwrap();

            if self.callstack.borrow().get_thread_with_index(original_thread_index).is_none() {
                if !has_choice_threads {
                    has_choice_threads = true;
                }

                jct.insert(original_thread_index.to_string(), c.get_thread_at_generation().unwrap().write_json());
            }
        }

        if has_choice_threads {
            flow.insert("choiceThreads".to_owned(), serde_json::Value::Object(jct));
        }

        let mut c_array: Vec<serde_json::Value> = Vec::new();
        for c in self.current_choices.iter() {
            c_array.push(json_write_state::write_choice(c));
        }

        flow.insert("currentChoices".to_owned(), serde_json::Value::Array(c_array));

        serde_json::Value::Object(flow)
    }

    pub fn load_flow_choice_threads(&mut self, j_choice_threads: &serde_json::Value, main_content_container: Rc<Container>) -> Result<(), String>{
        for choice in self.current_choices.iter_mut() {
            self.callstack.borrow().get_thread_with_index(choice.original_thread_index).map(|o| choice.set_thread_at_generation(o.copy())).or_else(|| {
                let j_saved_choice_thread =
                        j_choice_threads.get(&choice.original_thread_index.to_string()).ok_or("loading choice threads").unwrap();
                choice.set_thread_at_generation(Thread::from_json(&main_content_container, j_saved_choice_thread.as_object().unwrap()).unwrap());
                Some(())
            }).unwrap();
        }

        Ok(())
    }
}