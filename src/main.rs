// src/main.rs

// 1. Khai báo các module cần thiết
// Giữ các imports cần thiết và an toàn nhất

mod calc;

// 2. Import các struct và enum từ module calc
use calc::{Op, CalculatorState};

// 3. Import các định nghĩa UI từ ui.slint
slint::include_modules!();

// 4. Hàm main
fn main() -> Result<(), slint::PlatformError> {
    // Khởi tạo giao diện UI
    let ui = Calculator::new()?;
    
    // Tạo một Weak Handle (con trỏ yếu) của UI để truyền vào closure.
    let ui_handle = ui.as_weak(); 
    
    // Khởi tạo trạng thái logic của máy tính
    let mut state = CalculatorState::new(); 
    
    // Thiết lập giá trị hiển thị ban đầu
    ui.set_default(state.display_text().into());

    // Đăng ký callback cho sự kiện click nút
    ui.on_handle_button_clicked(move |input| { 
        let ui = ui_handle.unwrap(); 

        // ----------------------------------------------------
        // LOGIC XỬ LÝ NÚT NHẤN (đã hoàn thiện)
        // ----------------------------------------------------
        match input.as_str() {
            "C" => state.clear(),
            "=" => state.evaluate(),
            "±" => state.toggle_sign(),
            "←" => state.backspace(),
            "." => state.input_decimal(),
            
            // Các nút toán tử
            "+" => state.set_op(Op::Add),
            "-" => state.set_op(Op::Sub),
            "x" => state.set_op(Op::Mul),
            "/" => state.set_op(Op::Div),

            // BONUS: Xử lý các lệnh Memory 
            "M_CLEAR" => state.memory_clear(),
            "M_RECALL" => state.memory_recall(),
            "M_ADD" => state.memory_add(),
            "M_SUBTRACT" => state.memory_subtract(),
            
            // Các nút chữ số
            digit @ _ if digit.len() == 1 && digit.chars().next().unwrap().is_digit(10) => {
                state.input_digit(digit)
            }
            _ => {}
        }
        
        // Cập nhật giá trị hiển thị trên UI
        ui.set_default(state.display_text().into()); 
    });
    
    ui.run()
}



// // src/main.rs

// // 1. Khai báo các module cần thiết
// use slint::{Model, Weak}; // Đảm bảo import `Weak`
// mod calc;

// // 2. Import các struct và enum từ module calc
// use calc::{Op, CalculatorState};

// // 3. Import các định nghĩa UI từ ui.slint
// slint::include_modules!();

// // 4. Hàm main
// fn main() -> Result<(), slint::PlatformError> {
//     // Khởi tạo giao diện UI
//     let ui = Calculator::new()?;
    
//     // **SỬA LỖI QUAN TRỌNG:**
//     // Tạo một Weak Handle (con trỏ yếu) của UI để truyền vào closure.
//     // Điều này tránh việc di chuyển quyền sở hữu (move) của `ui` gốc.
//     let ui_handle = ui.as_weak(); // Dòng 14: Biến này sẽ được dùng
    
//     // Khởi tạo trạng thái logic của máy tính
//     // **LƯU Ý:** state phải là `mut` và phải được di chuyển (move) vào closure
//     // vì closure sẽ cần sửa đổi (mutate) nó.
//     let mut state = CalculatorState::new(); 
    
//     // Thiết lập giá trị hiển thị ban đầu
//     ui.set_default(state.display_text().into());

//     // Dòng 18: Đăng ký callback cho sự kiện click nút
//     // **SỬA LỖI QUAN TRỌNG:**
//     // Dùng `move` để di chuyển `ui_handle` (Weak<Calculator>) và `state` (CalculatorState) vào closure.
//     ui.on_handle_button_clicked(move |input| { 
//         // Lấy lại Strong handle từ Weak handle
//         let ui = ui_handle.unwrap(); 

//         // ----------------------------------------------------
//         // LOGIC XỬ LÝ NÚT NHẤN (giữ nguyên logic từ calc.rs)
//         // ----------------------------------------------------
//         match input.as_str() {
//             "C" => state.clear(),
//             "=" => state.evaluate(),
//             "±" => state.toggle_sign(),
//             "←" => state.backspace(),
//             "." => state.input_decimal(),
//             "+" => state.set_op(Op::Add),
//             "-" => state.set_op(Op::Sub),
//             "x" => state.set_op(Op::Mul),
//             "/" => state.set_op(Op::Div),
//             digit @ _ if digit.len() == 1 && digit.chars().next().unwrap().is_digit(10) => {
//                 state.input_digit(digit)
//             }
//             _ => {}
//         }
        
//         // Cập nhật giá trị hiển thị trên UI
//         ui.set_default(state.display_text().into()); 
//     });
    
    
//     ui.run()
// }