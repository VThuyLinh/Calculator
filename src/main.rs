mod calc;

// Import các struct và enum từ module calc
use calc::{Op, CalculatorState};

// Import các định nghĩa UI từ ui.slint
slint::include_modules!();

// Hàm main
fn main() -> Result<(), slint::PlatformError> {
    let ui = Calculator::new()?;
    let mut state = CalculatorState::new(); 
    
    // Thiết lập giá trị ban đầu
    ui.set_default(state.display_text().into());

    // Đăng ký callback cho click nút
    ui.on_handle_button_clicked({
        let ui_handle = ui.as_weak();
        move |input| { 
            let ui = ui_handle.unwrap(); 

            // ----------------------------------------------------
            // LOGIC XỬ LÝ NÚT NHẤN 
            // ----------------------------------------------------
            match input.as_str() {
                "C" => state.clear(),
                "=" => state.evaluate(),
                "±" => state.toggle_sign(),
                "←" => state.backspace(),
                "." => state.input_decimal(),
                
                // Nút toán tử
                "+" => state.set_op(Op::Add),
                "-" => state.set_op(Op::Sub),
                "x" => state.set_op(Op::Mul),
                "/" => state.set_op(Op::Div),

                // Lệnh Memory 
                "M_CLEAR" => state.memory_clear(),
                "M_RECALL" => state.memory_recall(),
                "M_ADD" => state.memory_add(),
                "M_SUBTRACT" => state.memory_subtract(),
                
                // Nút chữ số
                digit @ _ if digit.len() == 1 && digit.chars().next().unwrap().is_digit(10) => {
                    state.input_digit(digit)
                }
                _ => {}
            }
            
            // Cập nhật giá trị 
            ui.set_default(state.display_text().into()); 
        }
    });

    // Đăng ký callback cho chuyển đổi Theme
    ui.on_request_toggle_theme({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            // Lấy giá trị hiện tại và đảo ngược nó
            let current_mode = ui.get_is_dark_mode();
            ui.set_is_dark_mode(!current_mode);
        }
    });
    
    ui.run()
}


