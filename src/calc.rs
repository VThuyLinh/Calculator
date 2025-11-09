
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

// Khai báo CalculatorState
#[derive(Debug, Clone)]
pub struct CalculatorState {
    pub current: String,
    pub stored: f64,
    pub op: Option<Op>,
    pub last_operand: Option<f64>,
    pub last_op: Option<Op>, 
    pub error: bool,
    pub should_clear_current: bool,
    pub memory: f64, 
}

impl CalculatorState {
    /// Khởi tạo máy tính.
    pub fn new() -> Self {
        CalculatorState {
            current: "0".to_string(),
            stored: 0.0,
            op: None,
            last_operand: None,
            last_op: None,
            error: false,
            should_clear_current: false,
            memory: 0.0, 
        }
    }

    // Định dạng số float
    fn format_f64(value: f64) -> String {
        let mut s = format!("{:.12}", value)
            .trim_end_matches('0')
            .trim_end_matches('.')
            .to_string();
        
        if s == "-0" {
            s = "0".to_string();
        }
        s
    }

    /// Chuỗi để hiển thị trên UI
    pub fn display_text(&self) -> String {
        if self.error {
            return "Error".to_string();
        }
        self.current.clone()
    }
    
    fn current_value(&self) -> f64 {
        self.current.parse::<f64>().unwrap_or(0.0)
    }

    // --- Logic Memory ---
    pub fn memory_clear(&mut self) {
        self.memory = 0.0;
        self.error = false;
    }

    pub fn memory_recall(&mut self) {
        if self.error {
            return;
        }
        self.current = Self::format_f64(self.memory);
        self.should_clear_current = true;
    }

    pub fn memory_add(&mut self) {
        if self.error {
            return;
        }
        self.memory += self.current_value();
        self.should_clear_current = true;
    }

    pub fn memory_subtract(&mut self) {
        if self.error {
            return;
        }
        self.memory -= self.current_value();
        self.should_clear_current = true;
    }
    

    /// Đầu vào là một chữ số
    pub fn input_digit(&mut self, digit: &str) {
        if self.error {
            return;
        }

        if self.should_clear_current {
            self.current = "0".to_string();
            self.should_clear_current = false;
        }

        if self.current == "0" {
            self.current = digit.to_string();
        } else {
            if self.current.len() < 15 {
                self.current.push_str(digit);
            }
        }
    }

    /// Đầu vào là số thập phân
    pub fn input_decimal(&mut self) {
        if self.error {
            return;
        }

        if self.should_clear_current {
            self.current = "0".to_string();
            self.should_clear_current = false;
        }

        if !self.current.contains('.') {
            self.current.push('.');
        }
    }

    /// Phép toán đang chờ xử lý (+, -, x, /)
    pub fn set_op(&mut self, new_op: Op) {
        if self.error {
            return;
        }

        // Nếu đang có phép toán và đã nhập toán hạng 2, tính toán trung gian.
        if self.op.is_some() && !self.should_clear_current {
            self.evaluate_internal(false); 
        } 
        // Nếu đang có phép toán nhưng chưa nhập toán hạng 2 chỉ thay thế phép toán.
        else if self.op.is_some() && self.should_clear_current {
            self.op = Some(new_op);
            return;
        }

        // Nếu chưa có phép toán lấy current làm stored (hoặc đã tính toán xong ở bước 1).
        if self.op.is_none() {
             if let Ok(value) = self.current.parse::<f64>() {
                 self.stored = value;
             }
        }
        
        self.op = Some(new_op);
        self.should_clear_current = true;
        self.last_operand = None; 
        self.last_op = None;
    }

    fn calculate(&self, a: f64, b: f64, op: Op) -> Result<f64, String> {
        match op {
            Op::Add => Ok(a + b),
            Op::Sub => Ok(a - b),
            Op::Mul => Ok(a * b),
            Op::Div => {
                if b == 0.0 {
                    Err("Division by zero".to_string())
                } else {
                    Ok(a / b)
                }
            }
        }
    }

    /// Logic chung (dùng cho `=`, hoặc tính toán trung gian).
    fn evaluate_internal(&mut self, is_final_eval: bool) {
        if self.error {
            return;
        }
        
        let op_to_use;
        
        let second_operand: f64; 
        
        let mut first_operand = self.stored;
        
        // Lặp lại phép toán (ví dụ: 5 + 2 = = -> 9)
        if is_final_eval && self.op.is_none() && self.last_operand.is_some() && self.last_op.is_some() {
            op_to_use = self.last_op.unwrap();
            
            if let Ok(current_val) = self.current.parse::<f64>() {
                first_operand = current_val;
            } else {
                return;
            }
            
            second_operand = self.last_operand.unwrap(); 

        } 
        // Tính toán thông thường
        else if self.op.is_some() {
            op_to_use = self.op.unwrap();
            
            if let Ok(current_val) = self.current.parse::<f64>() {
                second_operand = current_val;
            } else {
                return; 
            }
            
            // Lưu trữ toán hạng 2 và phép toán cho lần lặp lại sau (Chỉ khi là = cuối cùng)
            if is_final_eval {
                self.last_operand = Some(second_operand);
                self.last_op = Some(op_to_use);
            }

        } else {
            return;
        }

        // TÍNH TOÁN CHUNG
        match self.calculate(first_operand, second_operand, op_to_use) {
            Ok(result) => {
                self.current = Self::format_f64(result);
                
                
                if is_final_eval {
                    self.op = None;
                } else {
                    self.stored = result;
                }
                
                self.should_clear_current = true;
            }
            Err(_) => {
                self.error = true;
                self.current = "Error".to_string();
                self.op = None;
            }
        }
    }

    /// Nút = 
    pub fn evaluate(&mut self) {
        self.evaluate_internal(true);
    }
    
    /// Nút xóa tất cả 
    pub fn clear(&mut self) {
        // Chỉ reset state, không clear memory!
        self.current = "0".to_string();
        self.stored = 0.0;
        self.op = None;
        self.last_operand = None;
        self.last_op = None;
        self.error = false;
        self.should_clear_current = false;
    }

    /// Nút xóa lùi
    pub fn backspace(&mut self) {
        if self.error || self.should_clear_current {
            return;
        }

        if self.current.len() > 1 {
            self.current.pop();
            if self.current.ends_with('.') {
                self.current.pop();
            }
        } 
        
    }
    /// Nút đổi dấu
    pub fn toggle_sign(&mut self) {
        if self.error {
            return;
        }
        
        if let Ok(value) = self.current.parse::<f64>() {
            let new_value = -value;
            self.current = Self::format_f64(new_value);
        }
    }
}

// --- Unit Tests ---
#[cfg(test)]
mod tests {
    use super::*;

    /// Tạo State và kiểm tra kết quả 
    fn run_test(inputs: &[&str], expected: &str) {
        let mut state = CalculatorState::new();
        run_test_with_state(&mut state, inputs, expected);
    }
    
    //Chạy test trên CÙNG MỘT state để test chuỗi thao tác liên tục (như Memory)
    fn run_test_with_state(state: &mut CalculatorState, inputs: &[&str], expected: &str) {
        for input in inputs {
            match *input {
                "C" => state.clear(),
                "=" => state.evaluate(),
                "±" => state.toggle_sign(),
                "←" => state.backspace(),
                "." => state.input_decimal(),
                "+" => state.set_op(Op::Add),
                "-" => state.set_op(Op::Sub),
                "x" => state.set_op(Op::Mul),
                "/" => state.set_op(Op::Div),
                "MC" => state.memory_clear(),
                "MR" => state.memory_recall(),
                "M+" => state.memory_add(),
                "M-" => state.memory_subtract(),
                digit @ _ if digit.len() == 1 && digit.chars().next().unwrap().is_digit(10) => {
                    state.input_digit(digit)
                }
                _ => panic!("Invalid input in test case: {}", input),
            }
        }
        assert_eq!(state.display_text(), expected, "Input: {:?}", inputs);
    }

    // Các test case
    // Test phép +
    #[test]
    fn test_case_1() {
        run_test(&["1", "2", "+", "3", "="], "15");
    }
    // Test nhân 0
    #[test]
    fn test_case_2() {
        run_test(&["5", "x", "0", "="], "0");
    }
    // Test chia 0
    #[test]
    fn test_case_3() {
        run_test(&["9", "/", "0", "="], "Error");
    }
    // Test + số thập phân
    #[test]
    fn test_case_4() {
        run_test(&["1", ".", ".", "5", "+", "2", "="], "3.5");
    }

    // Test phép + có 2 =
    #[test]
    fn test_case_5() {
        run_test(&["5", "+", "2", "=", "="], "9");
    }

    // Test xóa lùi
    #[test]
    fn test_case_6() {
        run_test(&["1", "0", "←", "←"], "1");
    }
    
    // Test giới hạn
    #[test]
    fn test_case_7() {
        let digits: Vec<&str> = (0..20).map(|_| "1").collect();
        run_test(&digits, "111111111111111"); 
    }

    // Test phép + liên tiếp
    #[test]
    fn test_chained_operations() {
        run_test(&["1", "+", "2", "+", "3", "="], "6");
    }
    
    // Test thay thế
    #[test]
    fn test_op_replacement() {
        run_test(&["1", "0", "+", "x", "-", "5", "="], "5");
    }
    
    // Test memory
    #[test]
    fn test_memory_functions() {
        let mut state = CalculatorState::new();
        
        // 1. 10 M+ 
        run_test_with_state(&mut state, &["1", "0", "M+"], "10"); 
        
        // 2. C 
        run_test_with_state(&mut state, &["C"], "0"); 
        
        // 3. MR 
        run_test_with_state(&mut state, &["MR"], "10");
        
        // 4. 5 M- 
        run_test_with_state(&mut state, &["5", "M-"], "5");
        
        // 5. MR 
        run_test_with_state(&mut state, &["MR"], "5");
        
        // 6. MC 
        state.memory_clear();
        assert_eq!(state.memory, 0.0);
    }
}



