# 🧮 Calculator Project (Slint + Rust)
Giới Thiệu
Đây là dự án ứng dụng máy tính cơ bản được phát triển bằng ngôn ngữ Rust và sử dụng framework giao diện người dùng Slint. Dự án này minh họa cách tích hợp giữa mã giao diện người dùng Slint (trong file .slint) và logic xử lý nghiệp vụ bằng Rust.

## 🛠️ Hướng Dẫn Build và Chạy Ứng Dụng
Để chạy ứng dụng này, bạn cần cài đặt Rust và sử dụng công cụ quản lý dự án Cargo.
1. Yêu cầu Tiên quyếtRust: Cài đặt Rust theo hướng dẫn getting-started guide.Hệ điều hành: Hỗ trợ Windows, macOS, Linux (hoặc nền tảng được Slint hỗ trợ).
2. Các Bước Thực hiện
cargo build => Biên dịch toàn bộ project, bao gồm cả mã Rust và mã Slint UI, tạo ra file thực thi (binary) trong thư mục target/debug/.
cargo run => Biên dịch (nếu cần) và chạy trực tiếp ứng dụng. Đây là cách nhanh nhất để khởi động ứng dụng.

## 🏗️ Tổng quan Kiến trúc

Project sử dụng kiến trúc phân tách rõ ràng giữa giao diện người dùng và logic nghiệp vụ.
1. Giao diện người dùng (Frontend - .slint):
   - File giao diện chính là ui/app-window.slint (hoặc tương tự).
   - Sử dụng cú pháp Slint Markup Language để định nghĩa layout (bố cục), các widget (nút bấm, màn hình hiển thị), và các callbacks (hàm được gọi khi có sự kiện, ví dụ: nhấn nút).
2. Logic nghiệp vụ (Backend - .rs):
   - Mã Rust trong src/main.rs chứa logic tính toán cốt lõi.
   - Rust chịu trách nhiệm khởi tạo giao diện Slint, liên kết các callbacks (như handle_button_press) được định nghĩa trong .slint với các hàm Rust tương ứng, và cập nhật các thuộc tính (properties) trên UI (ví dụ: hiển thị kết quả) sau khi tính toán.

Don't forget to edit this readme to replace it by yours, and edit the `name =` field in `Cargo.toml` to match the name of yourproject.
## 🖼️ Ảnh chụp Ứng dụng
<img width="316" height="476" alt="image" src="https://github.com/user-attachments/assets/62db24fc-c860-43d2-9d02-ccd3f293eef3" />




