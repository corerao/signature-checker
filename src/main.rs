// #![windows_subsystem = "windows"]
use iced::{
    Element, Font, Task,
    widget::{Button, Column, Container, ProgressBar, Row, TextInput, text},
};
// read this page for example
// https://jl710.github.io/iced-guide/runtime/blocking_code/blocking_code.html
#[derive(Default, Debug)]
struct App {
    input_path: String,
    is_running: bool,
    check_result: Option<bool>,
    progress: f32,
    status_message: String,
}

#[derive(Clone, Debug)]
enum Message {
    InputPathChanged(String),
    StartCheck,
    CheckCompleted(Result<bool, String>),
    UpdateProgress(f32),
}

impl App {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::InputPathChanged(path) => {
                self.input_path = path;
                Task::none()
            }

            Message::StartCheck => {
                if self.input_path.is_empty() {
                    self.status_message = "请输入检查路径".to_string();
                    return Task::none();
                }

                self.is_running = true;
                self.check_result = None;
                self.progress = 0.0;
                self.status_message = "正在检查...".to_string();

                // 启动异步任务
                let path = self.input_path.clone();
                Task::perform(
                    async move {
                        // 模拟异步命令执行
                        real_signature_check(&path).await
                    },
                    Message::CheckCompleted,
                )
            }

            Message::CheckCompleted(result) => {
                self.is_running = false;
                match result {
                    Ok(success) => {
                        self.check_result = Some(success);
                        self.status_message = if success {
                            "检查成功！".to_string()
                        } else {
                            "检查失败！".to_string()
                        };
                    }
                    Err(err) => {
                        self.status_message = format!("错误: {}", err);
                    }
                }
                Task::none()
            }

            Message::UpdateProgress(progress) => {
                self.progress = progress;
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let input = TextInput::new("输入检查路径", &self.input_path)
            .on_input(Message::InputPathChanged)
            .padding(10)
            .size(16);

        let start_button = Button::new(text("开始检查"))
            .on_press_maybe(if !self.is_running {
                Some(Message::StartCheck)
            } else {
                None
            })
            .padding(10);

        let mut content = Column::new().spacing(20).padding(20);

        content = content.push(Row::new().spacing(10).push(text("检查路径:")).push(input));

        content = content.push(start_button);

        if self.is_running {
            content = content.push(ProgressBar::new(0.0..=100.0, self.progress).height(20));
        }

        if let Some(result) = self.check_result {
            let status_text = if result {
                text("✓ 检查成功").color(iced::Color::from_rgb(0.0, 0.5, 0.0))
            } else {
                text("✗ 检查失败").color(iced::Color::from_rgb(0.5, 0.0, 0.0))
            };
            content = content.push(status_text);
        }

        content = content.push(text(&self.status_message));

        Container::new(content)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .into()
    }
}

// 异步执行检查的函数
async fn check_signature_async(path: &str) -> Result<bool, String> {
    // 模拟异步执行外部命令
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    // 实际执行命令的示例（使用 tokio::process::Command）
    match tokio::process::Command::new("cmd")
        .args(["/C", "echo", "Checking", path])
        .output()
        .await
    {
        Ok(output) => {
            if output.status.success() {
                // 解析命令输出，这里简单返回成功
                Ok(true)
            } else {
                Err("命令执行失败".to_string())
            }
        }
        Err(e) => Err(format!("执行错误: {}", e)),
    }
}

// 实际的文件签名检查函数示例
async fn real_signature_check(path: &str) -> Result<bool, String> {
    use tokio::fs;
    use tokio::process::Command;

    // 检查文件是否存在
    if !fs::metadata(path).await.is_ok() {
        return Err("文件不存在".to_string());
    }

    // 执行实际的签名检查命令（Windows 示例）
    let output = Command::new("powershell")
        .args([
            "-Command",
            &format!(
                "Get-AuthenticodeSignature '{}' | Select-Object Status",
                path
            ),
        ])
        .output()
        .await
        .map_err(|e| format!("执行 PowerShell 失败: {}", e))?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        // 解析输出判断签名状态
        Ok(stdout.contains("Valid"))
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(format!("检查失败: {}", stderr))
    }
}

fn main() -> iced::Result {
    iced::application("signature-checker", App::update, App::view)
        .default_font(Font::with_name("微软雅黑"))
        .run()
}
