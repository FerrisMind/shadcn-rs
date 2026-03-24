use iced::widget::{column, container, scrollable, text};
use iced::{Alignment, Background, Element, Length, Padding, Task};
use lucide_icons::LUCIDE_FONT_BYTES;

use iced_shadcn::{
    Theme, ToolContentProps, ToolHeaderProps, ToolInputProps, ToolOutputProps, ToolOutputValue,
    ToolProps, ToolUIPartState, tool, tool_header_default, tool_input, tool_output,
};

const BASIC_INPUT: &str = r#"{
  "location": "San Francisco, CA",
  "units": "celsius"
}"#;
const BASIC_OUTPUT: &str = r#"{
  "temperature": 18,
  "conditions": "Partly cloudy",
  "humidity": 65,
  "wind_speed": 12
}"#;

const STATES_PROCESSING_INPUT: &str = r#"{
  "file_path": "/src/components/Button.tsx",
  "analysis_type": "security"
}"#;
const STATES_READY_INPUT: &str = r#"{
  "prompt": "A serene mountain landscape at sunset",
  "size": "1024x1024",
  "style": "photorealistic"
}"#;
const STATES_COMPLETED_INPUT: &str = r#"{
  "location": "Tokyo, Japan",
  "units": "celsius"
}"#;
const STATES_COMPLETED_OUTPUT: &str = r#"{
  "temperature": 22,
  "conditions": "Clear sky",
  "humidity": 58,
  "wind_speed": 8,
  "forecast": "Sunny throughout the day"
}"#;
const STATES_ERROR_INPUT: &str = r#"{
  "to": "user@example.com",
  "subject": "Welcome to our platform",
  "body": "Thank you for signing up!"
}"#;
const STATES_ERROR_TEXT: &str = "SMTP Authentication failed: Invalid credentials for mail.example.com:587. Please check your email configuration.";

const COMPACT_PROCESSING_INPUT: &str = r#"{
  "query": "AI best practices",
  "max_results": 10
}"#;
const COMPACT_READY_INPUT: &str = r#"{
  "text": "Hello, world!",
  "target_language": "es"
}"#;
const COMPACT_COMPLETED_INPUT: &str = r#"{
  "numbers": [10, 20, 30, 40]
}"#;
const COMPACT_COMPLETED_OUTPUT: &str = r#"{
  "result": 100,
  "operation": "sum"
}"#;
const COMPACT_ERROR_INPUT: &str = r#"{
  "url": "https://api.example.com/data",
  "method": "GET"
}"#;
const COMPACT_ERROR_TEXT: &str = "Network timeout: Request exceeded 30 second limit";

#[derive(Debug, Clone, Copy)]
enum ToolCardId {
    Basic,
    StatesProcessing,
    StatesReady,
    StatesCompleted,
    StatesError,
    CompactProcessing,
    CompactReady,
    CompactCompleted,
    CompactError,
}

impl ToolCardId {
    const fn index(self) -> usize {
        match self {
            ToolCardId::Basic => 0,
            ToolCardId::StatesProcessing => 1,
            ToolCardId::StatesReady => 2,
            ToolCardId::StatesCompleted => 3,
            ToolCardId::StatesError => 4,
            ToolCardId::CompactProcessing => 5,
            ToolCardId::CompactReady => 6,
            ToolCardId::CompactCompleted => 7,
            ToolCardId::CompactError => 8,
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    SetOpen(ToolCardId, bool),
}

struct Example {
    theme: Theme,
    open: [bool; 9],
}

impl Default for Example {
    fn default() -> Self {
        Self {
            theme: Theme::default(),
            open: [
                true,  // Basic
                true,  // StatesProcessing
                false, // StatesReady
                true,  // StatesCompleted
                false, // StatesError
                false, // CompactProcessing
                false, // CompactReady
                false, // CompactCompleted
                false, // CompactError
            ],
        }
    }
}

impl Example {
    fn update(&mut self, message: Message) -> Task<Message> {
        let Message::SetOpen(id, value) = message;
        self.open[id.index()] = value;
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;

        let intro = column![
            text("Tool Demo").size(28),
            text("Single binary: basic + tool states + tool states compact.")
                .size(14)
                .style(|_| iced::widget::text::Style {
                    color: Some(theme.palette.muted_foreground),
                }),
        ]
        .spacing(6);

        let basic_section = column![
            text("Basic").size(20),
            self.wrap_width(
                self.tool_card(
                    ToolCardId::Basic,
                    "get_weather",
                    ToolUIPartState::OutputAvailable,
                    BASIC_INPUT,
                    Some(ToolOutputProps::new().output(ToolOutputValue::json(BASIC_OUTPUT))),
                    ToolProps::new(),
                ),
                760.0
            ),
        ]
        .spacing(10);

        let states_section = column![
            text("Tool States").size(20),
            self.state_with_label(
                "Processing - Input Streaming",
                self.tool_card(
                    ToolCardId::StatesProcessing,
                    "analyze_code",
                    ToolUIPartState::InputStreaming,
                    STATES_PROCESSING_INPUT,
                    None,
                    ToolProps::new(),
                ),
            ),
            self.state_with_label(
                "Ready - Input Available",
                self.tool_card(
                    ToolCardId::StatesReady,
                    "generate_image",
                    ToolUIPartState::InputAvailable,
                    STATES_READY_INPUT,
                    None,
                    ToolProps::new(),
                ),
            ),
            self.state_with_label(
                "Completed - Output Available",
                self.tool_card(
                    ToolCardId::StatesCompleted,
                    "get_weather",
                    ToolUIPartState::OutputAvailable,
                    STATES_COMPLETED_INPUT,
                    Some(
                        ToolOutputProps::new()
                            .output(ToolOutputValue::json(STATES_COMPLETED_OUTPUT)),
                    ),
                    ToolProps::new(),
                ),
            ),
            self.state_with_label(
                "Error - Output Error",
                self.tool_card(
                    ToolCardId::StatesError,
                    "send_email",
                    ToolUIPartState::OutputError,
                    STATES_ERROR_INPUT,
                    Some(ToolOutputProps::new().error_text(STATES_ERROR_TEXT)),
                    ToolProps::new(),
                ),
            ),
        ]
        .spacing(12);

        let compact_props = ToolProps::new().compact(true);
        let compact_section = column![
            text("Tool States Compact").size(20),
            self.wrap_width(
                self.tool_card(
                    ToolCardId::CompactProcessing,
                    "search_documents",
                    ToolUIPartState::InputStreaming,
                    COMPACT_PROCESSING_INPUT,
                    None,
                    compact_props,
                ),
                520.0
            ),
            self.wrap_width(
                self.tool_card(
                    ToolCardId::CompactReady,
                    "translate_text",
                    ToolUIPartState::InputAvailable,
                    COMPACT_READY_INPUT,
                    None,
                    compact_props,
                ),
                520.0
            ),
            self.wrap_width(
                self.tool_card(
                    ToolCardId::CompactCompleted,
                    "calculate_sum",
                    ToolUIPartState::OutputAvailable,
                    COMPACT_COMPLETED_INPUT,
                    Some(
                        ToolOutputProps::new()
                            .output(ToolOutputValue::json(COMPACT_COMPLETED_OUTPUT)),
                    ),
                    compact_props,
                ),
                520.0
            ),
            self.wrap_width(
                self.tool_card(
                    ToolCardId::CompactError,
                    "fetch_api_data",
                    ToolUIPartState::OutputError,
                    COMPACT_ERROR_INPUT,
                    Some(ToolOutputProps::new().error_text(COMPACT_ERROR_TEXT)),
                    compact_props,
                ),
                520.0
            ),
        ]
        .spacing(8)
        .align_x(Alignment::Center);

        let page = column![intro, basic_section, states_section, compact_section]
            .spacing(18)
            .width(Length::Fill)
            .max_width(980);

        container(scrollable(page))
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .center_x(Length::Fill)
            .style(move |_| iced::widget::container::Style {
                background: Some(Background::Color(theme.palette.background)),
                text_color: Some(theme.palette.foreground),
                ..iced::widget::container::Style::default()
            })
            .into()
    }

    fn state_with_label<'a>(
        &'a self,
        label: &'a str,
        card: Element<'a, Message>,
    ) -> Element<'a, Message> {
        column![
            text(label).size(14).style(|_| iced::widget::text::Style {
                color: Some(self.theme.palette.muted_foreground),
            }),
            self.wrap_width(card, 760.0),
        ]
        .spacing(6)
        .into()
    }

    fn wrap_width<'a>(&self, card: Element<'a, Message>, max_width: f32) -> Element<'a, Message> {
        container(card)
            .width(Length::Fill)
            .max_width(max_width)
            .center_x(Length::Fill)
            .into()
    }

    fn tool_card<'a>(
        &'a self,
        id: ToolCardId,
        tool_type: &'a str,
        state: ToolUIPartState,
        input: &'a str,
        output: Option<ToolOutputProps<'a>>,
        tool_props: ToolProps,
    ) -> Element<'a, Message> {
        let theme = &self.theme;
        let open = self.open[id.index()];
        let header = tool_header_default(open, ToolHeaderProps::new(tool_type, state), theme);

        let has_output = output.is_some();
        let input_props = if has_output {
            ToolInputProps::new(input).padding(Padding {
                top: 16.0,
                right: 16.0,
                bottom: 8.0,
                left: 16.0,
            })
        } else {
            ToolInputProps::new(input)
        };

        let mut parts: Vec<Element<'a, Message>> = vec![tool_input(input_props, theme)];
        if let Some(output_props) = output {
            let output_props = output_props.padding(Padding {
                top: 8.0,
                right: 16.0,
                bottom: 16.0,
                left: 16.0,
            });
            parts.push(tool_output(output_props, theme));
        }

        tool(
            open,
            header,
            column(parts).spacing(0).width(Length::Fill),
            Some(move |next_open| Message::SetOpen(id, next_open)),
            tool_props,
            ToolContentProps::new(),
            theme,
        )
    }
}

pub fn main() -> iced::Result {
    iced::application(Example::default, Example::update, Example::view)
        .title("Tool Demo")
        .font(LUCIDE_FONT_BYTES)
        .run()
}
