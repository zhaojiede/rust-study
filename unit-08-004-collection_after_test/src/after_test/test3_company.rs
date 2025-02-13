use std::collections::HashMap;

/*
使用哈希 map 和 vector，创建一个文本接口来允许用户向公司的部门中增加员工的名字。
例如，“Add Sally to Engineering” 或 “Add Amir to Sales”。
接着让用户获取一个部门的所有员工的列表，或者公司每个部门的所有员工按照字典序排列的列表。
*/
fn add_employee_get_member_list(add_str: &str) -> Vec<String> {
    add_employee(add_str);
    get_member_list()
}

fn add_employee(add_str: &str) {
    let mut company = HashMap::new();
    let (name, department) = get_department_name(add_str);
    company.entry(department).or_insert(Vec::new().push(name));

}

fn get_department_name(add_str: &str) -> (String, String) {
    add_str.split_once(" to ").unwrap()
}