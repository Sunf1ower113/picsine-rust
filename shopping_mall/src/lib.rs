pub use crate::floor::store::employee::Employee;
pub use crate::floor::store::Store;
pub use crate::guard::Guard;
pub use crate::mall::*;
pub use mall::{*,floor::store};

pub mod mall;


pub fn biggest_store(m: Mall) -> Store {
    let mut biggest_store = Store {
        name: "".to_string(),
        square_meters: 0,
        employees: vec![]
    };
    for floor in m.floors {
        for store in floor.stores {
            if store.square_meters > biggest_store.square_meters {
                biggest_store = store;
            }
        }
    }
    biggest_store
}

pub fn highest_paid_employee(m: Mall) -> Vec<Employee> {
    let mut res: Vec<Employee> = vec![];
    let mut new_employee = Employee {
        name: "".to_string(),
        age: 0,
        working_hours: (0, 0),
        salary: 0.0
    };
    for floor in m.floors {
        for store in floor.stores {
            for employee in store.employees {
                if employee.salary > new_employee.salary {
                    new_employee = employee;
                    res.clear();
                    res.push(new_employee.clone());
                } else if employee.salary == new_employee.salary {
                    new_employee = employee;
                    res.push(new_employee.clone());
                }
            }
        }
    }
    res
}

pub fn nbr_of_employees(m: Mall) -> usize {
    let mut nbr: usize = 0;
    for floor in m.floors {
        for store in floor.stores {
            nbr += store.employees.len();
        }
    }
    nbr + m.guards.len()
}

pub fn check_for_securities(m: &mut Mall, g: Vec<Guard>) {
    let mut square: usize = 0;
    for floor in &m.floors {
        square += floor.size_limit as usize;
    }
    let nbr = square / 200;
    let mut i = 0;
    while nbr > m.guards.len() {
        m.hire_guard(g.get(i).unwrap().clone());
        i += 1;
    }
}
pub fn cut_or_raise(m: &mut Mall) {
    for floor in &mut m.floors {
        for store in &mut floor.stores {
            for employee in &mut store.employees {
                let delta = employee.salary * 0.1;
                if employee.working_hours.1 - employee.working_hours.0 > 10 {
                    employee.raise(delta);
                } else {
                    employee.cut(delta);
                }
            }
        }
    }
}