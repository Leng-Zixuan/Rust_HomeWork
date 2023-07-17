use std::collections::HashMap;

// 定义学生结构体
#[derive(Clone, Debug)]
struct Student {
    id: u32,
    name: String,
    age: u8,
    class_id: u32,
}

// 定义班级结构体
#[derive(Clone, Debug)]
struct Class {
    id: u32,
    name: String,
    students: Vec<Student>,
}

// 定义课程结构体
#[derive(Debug)]
struct Course {
    id: u32,
    name: String,
    classes: Vec<Class>,
}

// 定义学生管理系统结构体
struct StudentManagementSystem {
    students: HashMap<u32, Student>,
    classes: HashMap<u32, Class>,
    courses: HashMap<u32, Course>,
}

impl StudentManagementSystem {
    // 创建学生
    fn create_student(&mut self, id: u32, name: String, age: u8, class_id: u32) {
        let student = Student {
            id,
            name,
            age,
            class_id,
        };
        self.students.insert(id, student);
    }

    // 读取学生
    fn read_student(&self, id: u32) -> Option<&Student> {
        self.students.get(&id)
    }

    // 更新学生
    fn update_student(&mut self, id: u32, name: String, age: u8, class_id: u32) {
        if let Some(student) = self.students.get_mut(&id) {
            student.name = name;
            student.age = age;
            student.class_id = class_id;
        }
    }

    // 删除学生
    fn delete_student(&mut self, id: u32) {
        self.students.remove(&id);
    }

    // 创建班级
    fn create_class(&mut self, id: u32, name: String) {
        let class = Class {
            id,
            name,
            students: Vec::new(),
        };
        self.classes.insert(id, class);
    }

    // 读取班级
    fn read_class(&self, id: u32) -> Option<&Class> {
        self.classes.get(&id)
    }

    // 更新班级
    fn update_class(&mut self, id: u32, name: String) {
        if let Some(class) = self.classes.get_mut(&id) {
            class.name = name;
        }
    }

    // 删除班级
    fn delete_class(&mut self, id: u32) {
        self.classes.remove(&id);
    }

    // 创建课程
    fn create_course(&mut self, id: u32, name: String) {
        let course = Course {
            id,
            name,
            classes: Vec::new(),
        };
        self.courses.insert(id, course);
    }

    // 读取课程
    fn read_course(&self, id: u32) -> Option<&Course> {
        self.courses.get(&id)
    }

    // 更新课程
    fn update_course(&mut self, id: u32, name: String) {
        if let Some(course) = self.courses.get_mut(&id) {
            course.name = name;
        }
    }

    // 删除课程
    fn delete_course(&mut self, id: u32) {
        self.courses.remove(&id);
    }

    // 将学生添加到班级中
    fn add_student_to_class(&mut self, student_id: u32, class_id: u32) {
        if let Some(student) = self.students.get(&student_id) {
            if let Some(class) = self.classes.get_mut(&class_id) {
                class.students.push(student.clone());
            }
        }
    }

    // 将班级添加到课程中
    fn add_class_to_course(&mut self, class_id: u32, course_id: u32) {
        if let Some(class) = self.classes.get(&class_id) {
            if let Some(course) = self.courses.get_mut(&course_id) {
                course.classes.push(class.clone());
            }
        }
    }
}

fn main() {
    let mut sms = StudentManagementSystem {
        students: HashMap::new(),
        classes: HashMap::new(),
        courses: HashMap::new(),
    };

    // 创建学生
    sms.create_student(1, String::from("张三"), 18, 1);
    sms.create_student(2, String::from("李四"), 19, 1);
    sms.create_student(3, String::from("王五"), 20, 2);

    // 创建班级
    sms.create_class(1, String::from("一班"));
    sms.create_class(2, String::from("二班"));

    // 创建课程
    sms.create_course(1, String::from("数学"));
    sms.create_course(2, String::from("英语"));

    // 将学生添加到班级中
    sms.add_student_to_class(1, 1);
    sms.add_student_to_class(2, 1);
    sms.add_student_to_class(3, 2);

    // 将班级添加到课程中
    sms.add_class_to_course(1, 1);
    sms.add_class_to_course(2, 2);

    // 读取学生、班级和课程
    println!("{:?}", sms.read_student(1));
    println!("{:?}", sms.read_class(1));
    println!("{:?}", sms.read_course(1));

    // 更新学生、班级和课程
    sms.update_student(1, String::from("张三三"), 19, 2);
    sms.update_class(1, String::from("一班一班"));
    sms.update_course(1, String::from("高数"));

    // 读取更新后的学生、班级和课程
    println!("{:?}", sms.read_student(1));
    println!("{:?}", sms.read_class(1));
    println!("{:?}", sms.read_course(1));

    // 删除学生、班级和课程
    sms.delete_student(1);
    sms.delete_class(1);
    sms.delete_course(1);

    // 读取删除后的学生、班级和课程
    println!("{:?}", sms.read_student(1));
    println!("{:?}", sms.read_class(1));
    println!("{:?}", sms.read_course(1));
}
