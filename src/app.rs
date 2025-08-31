use crate::fl;
// use crate::i18n::LANGUAGE_LOADER;
use iced::{
    widget::{column, container, progress_bar, row, text, text_input, Button}, Alignment::Center, Element, Length::Fill, Task
};
#[derive(Default, Debug)]
pub struct App {
    input_path: String,
    status: AppStatus,
}

#[derive(Default, Debug)]
pub enum AppStatus {
    #[default]
    Pending,
    Running(f32),
    Finished(String),
}

#[derive(Clone, Debug)]
pub enum Message {
    /// 用户在输入框里修改路径
    InputPathChanged(String),

    /// 点击“开始检查”
    StartCheck,

    /// 检查进度更新（运行中用）
    ProgressUpdated(f32),

    /// 检查完成，带结果信息
    Finished(String),

    /// 用户点击“重新开始”
    Reset,
}

impl App {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        todo!()
        // match message {
        //     Message::InputPathChanged(path) => {
        //         self.input_path = path;
        //         Task::none()
        //     }

        //     Message::StartCheck => {
        //         if self.input_path.is_empty() {
        //             self.status_message = "请输入检查路径".to_string();
        //             return Task::none();
        //         }

        //         self.is_running = true;
        //         self.check_result = None;
        //         self.progress = 0.0;
        //         self.status_message = "正在检查...".to_string();

        //         // 启动异步任务
        //         let path = self.input_path.clone();
        //         Task::perform(
        //             async move {
        //                 // 模拟异步命令执行
        //                 real_signature_check(&path).await
        //             },
        //             Message::CheckCompleted,
        //         )
        //     }

        //     Message::CheckCompleted(result) => {
        //         self.is_running = false;
        //         match result {
        //             Ok(success) => {
        //                 self.check_result = Some(success);
        //                 self.status_message = if success {
        //                     "检查成功！".to_string()
        //                 } else {
        //                     "检查失败！".to_string()
        //                 };
        //             }
        //             Err(err) => {
        //                 self.status_message = format!("错误: {}", err);
        //             }
        //         }
        //         Task::none()
        //     }

        //     Message::UpdateProgress(progress) => {
        //         self.progress = progress;
        //         Task::none()
        //     }
        // }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let content = match &self.status {
            // 等待状态：输入框 + 按钮
            AppStatus::Pending => container(column![
                row![
                    text(fl!("check_path")),
                    text_input(&fl!("check_path_placeholder"), &self.input_path)
                        .on_input_maybe(Some(Message::InputPathChanged)),
                ]
                .spacing(10).align_y(Center),
                Button::new(text(fl!("start_check")))
                    .on_press_maybe(Some(Message::StartCheck))
                    .padding(10),
            ].align_x(Center).spacing(10)),
            // 运行中：进度条
            AppStatus::Running(prog) => container(
                column![
                    text(fl!("checking")),
                    progress_bar(0.0..=100.0, *prog).height(20)
                ]
                .spacing(20)
                .padding(20),
            ),
            // 完成：显示结果
            AppStatus::Finished(msg) => container(
                column![
                    text(fl!("check_finished")),
                    text(msg).size(16),
                    Button::new(text(fl!("restart")))
                        .on_press(Message::Reset)
                        .padding(10)
                ]
                .spacing(20)
                .padding(20),
            ),
        };
        content.width(Fill).padding(10).center(Fill).into()
    }
}

#[test]
fn lan() {
    println!("{}", fl!("check_path_placeholder"));
}
