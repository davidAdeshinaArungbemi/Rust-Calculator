// use std::collections::HashMap;
use std::io;

fn remove_whitespace(mut expression: String) -> String {
    expression.retain(|c| !c.is_whitespace());

    return expression.to_string();
}

fn is_an_operator(ch: char) -> bool {
    match ch {
        '(' | ')' | '^' | '*' | '/' | '+' => return true,
        _ => return false, //@ means nothing
    }
}

fn is_a_number(ch: char) -> bool {
    match ch {
        '0'..='9' => return true,
        '-' => return true,
        _ => return false,
    }
}

fn check_for_invalids(expr: &String) {
    for ch in expr.chars() {
        if !(is_a_number(ch) || is_an_operator(ch)) {
            panic!("Invalid expression!");
        }
    }
}

fn parenthesis_corrector(expr: String) -> String {
    let mut expr: String = expr.clone();
    for i in 0..expr.len() {
        if (is_an_operator(expr.chars().nth(i).unwrap()) == false && i != expr.len() - 1)
            || expr.chars().nth(i).unwrap() == ')'
        {
            if expr.chars().nth(i + 1).unwrap() == '(' {
                expr.insert(i + 1, '*');
            }
        }
    }

    if expr.chars().nth(0).unwrap() == '+' {
        expr.insert(0, '0');
    }

    return expr;
}

fn negative_sign_corrector(mut expr: String) -> String {
    for i in 0..expr.len() {
        if i == 0 {
            continue;
        }
        if expr.chars().nth(i).unwrap() == '-' {
            if is_an_operator(expr.chars().nth(i - 1).unwrap()) {
                continue;
            }
            //code adds + signs where necessary when we have something like -2-2, which becomes -2+-2
            expr.insert(i, '+');
        }
    }
    println!("{}", expr);
    return expr;
}

fn select_operator_character(expr: &String) -> (char, usize) {
    if expr.contains('(') {
        return ('(', expr.find('(').unwrap());
    } else if expr.contains('^') {
        return ('^', expr.find('^').unwrap());
    } else if expr.contains('*') {
        return ('*', expr.find('*').unwrap());
    } else if expr.contains('/') {
        return ('/', expr.find('/').unwrap());
    } else {
        return ('+', expr.find('+').unwrap());
    }
}

fn get_operands(sub_expr: String, ch: char) -> (f64, f64) {
    let index = sub_expr.find(ch).unwrap();
    // if index_e > sub_expr.len() + 1 {
    //     index_e = index_e - sub_expr.len() + 1;
    // }
    let op_1: String = sub_expr.chars().take(index).collect();
    let op_2: String = sub_expr
        .chars()
        .skip(index + 1)
        .take(sub_expr.len() - index + 1)
        .collect();

    let op_1 = op_1.parse::<f64>().unwrap();
    let op_2 = op_2.parse::<f64>().unwrap();
    return (op_1, op_2);
}

fn get_sub_expr(expr: &String, index: usize) -> (String, usize, usize) {
    let mut index_s: usize = 0;
    let mut index_e: usize = expr.len();

    let mut new_expr: String = String::new();
    for i in (0..index).rev() {
        let ch = expr.chars().nth(i).unwrap();
        if is_an_operator(ch) {
            index_s = i + 1;
            break;
        }
        new_expr.push(ch);
    }

    new_expr = new_expr.chars().rev().collect();

    new_expr.push(expr.chars().nth(index).unwrap());

    for i in (index + 1)..expr.len() {
        let ch = expr.chars().nth(i).unwrap();
        if is_an_operator(ch) {
            index_e = i;
            break;
        }
        new_expr.push(ch);
    }

    return (new_expr, index_s, index_e);
}

fn update_expression(expr: String, result: f64, s_index: usize, e_index: usize) -> String {
    let before_result: String = expr.chars().take(s_index).collect();
    let after_result: String = expr
        .chars()
        .skip(e_index)
        .take(expr.len() - e_index)
        .collect();

    let string_result = result.to_string();
    let final_expr = before_result + &string_result + &after_result;

    return final_expr;
}

fn choose_operation(ch: char, expr: String, index: usize) -> String {
    match ch {
        '(' => return paren(expr, index),
        // ')' => return paren(expr, index),
        '^' => {
            let sub_expr_info = get_sub_expr(&expr, index);
            let op_tuple = get_operands(sub_expr_info.0, ch);
            let result = f64::powf(op_tuple.0, op_tuple.1);
            println!("{}", result);
            let updated_expr = update_expression(expr, result, sub_expr_info.1, sub_expr_info.2);
            return updated_expr;
        }
        '*' => {
            let sub_expr_info = get_sub_expr(&expr, index);
            let op_tuple = get_operands(sub_expr_info.0, ch);
            let result = op_tuple.0 * op_tuple.1;
            let updated_expr = update_expression(expr, result, sub_expr_info.1, sub_expr_info.2);
            return updated_expr;
        }
        '/' => {
            let sub_expr_info = get_sub_expr(&expr, index);
            let op_tuple = get_operands(sub_expr_info.0, ch);
            let result = op_tuple.0 / op_tuple.1;
            let updated_expr = update_expression(expr, result, sub_expr_info.1, sub_expr_info.2);
            return updated_expr;
        }
        '+' => {
            let sub_expr_info = get_sub_expr(&expr, index);
            let op_tuple = get_operands(sub_expr_info.0, ch);
            let result = op_tuple.0 + op_tuple.1;
            let updated_expr = update_expression(expr, result, sub_expr_info.1, sub_expr_info.2);
            return updated_expr;
        }
        _ => return expr.clone(),
    }
}

fn paren(expr: String, index: usize) -> String {
    let index_2 = expr.find(')');
    if index_2.is_none() {
        panic!("No closing brackets found");
    } else {
        let mut before_expr: String = String::new();
        let mut after_expr: String = String::new();
        let mut sub_expr: String = expr
            .chars()
            .skip(index + 1)
            .take(index_2.unwrap() - (index) - 1)
            .collect();

        sub_expr = begin_operation(sub_expr);

        if index != 0 {
            before_expr = expr.chars().take(index).collect();
            sub_expr.insert_str(0, &before_expr);
        }
        if index_2.unwrap() != expr.len() - 1 {
            after_expr = expr
                .chars()
                .skip(index_2.unwrap() + 1)
                .take(expr.len() - index_2.unwrap())
                .collect();
            sub_expr.insert_str(sub_expr.len(), &after_expr);
        }
        return sub_expr;
    }
}

fn begin_operation(expr: String) -> String {
    match expr.parse::<f64>() {
        //check if its just a number
        Ok(_) => return expr,
        Err(_) => {
            let expr = negative_sign_corrector(expr);
            let char_info = select_operator_character(&expr);
            let reduced_expression = choose_operation(char_info.0, expr, char_info.1);
            return begin_operation(reduced_expression);
        }
    };
}

fn main() {
    let mut expression = String::new();
    let _ = io::stdin().read_line(&mut expression);
    let expression = remove_whitespace(expression);
    let expression = parenthesis_corrector(expression);
    check_for_invalids(&expression);
    println!("{}", begin_operation(expression));
}
