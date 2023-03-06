use turtle::{
    basis::BasisSet,
    methods::{CorrelationMethod, HFType},
    settings::RunType,
};

use iced::widget::{
    button, column, container, horizontal_space, pick_list, row, scrollable, text, vertical_space,
};
use iced::{
    theme::{self},
    widget::text_input,
};
use iced::{Alignment, Element, Length, Sandbox, Settings};

pub fn main() -> iced::Result {
    Turtle::run(Settings::default())
}

struct Turtle {
    runtype: RunType,
    hftype: HFType,
    correlation_method: Option<CorrelationMethod>,
    basis_set: BasisSet,

    charge_str: String,
    charge: i8,
    multiplicity_str: String,
    multiplicity: u8,

    // program settings
    selection_width: u16,
}

impl Default for Turtle {
    fn default() -> Self {
        Self {
            runtype: RunType::default(),
            hftype: HFType::default(),
            correlation_method: None,
            basis_set: BasisSet::default(),
            charge_str: "0".to_string(),
            charge: 0,
            multiplicity_str: "1".to_string(),
            multiplicity: 1,
            selection_width: 200,
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    RunTypeSelected(RunType),
    HFTypeSelected(HFType),
    CorrelationMethodTypeSelected(CorrelationMethod),
    BasisSetSelected(BasisSet),
    ChargeEditing(String),
    ChargeEdited(String),
    MultiplicityEditing(String),
    MultiplicityEdited(String),
    RunORCA,
}

impl Sandbox for Turtle {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Turtle - ORCA GUI")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::RunTypeSelected(runtype) => self.runtype = runtype,
            Message::HFTypeSelected(hftype) => self.hftype = hftype,
            Message::CorrelationMethodTypeSelected(correlaton_method) => {
                self.correlation_method = if correlaton_method == CorrelationMethod::None {
                    None
                } else {
                    Some(correlaton_method)
                };
            }
            Message::BasisSetSelected(basis_set) => self.basis_set = basis_set,
            Message::ChargeEditing(charge) => self.charge_str = charge,
            Message::ChargeEdited(charge) => match charge.parse::<i8>() {
                Ok(c) => self.charge = c,
                Err(_) => {
                    self.charge_str = "0".to_string();
                    self.charge = 0
                }
            },
            Message::MultiplicityEditing(multiplicity) => self.multiplicity_str = multiplicity,
            Message::MultiplicityEdited(multiplicity) => match multiplicity.parse::<u8>() {
                Ok(m) => self.multiplicity = m,
                Err(_) => {
                    self.multiplicity_str = "1".to_string();
                    self.multiplicity = 1
                }
            },
            Message::RunORCA => (),
            _ => (),
        }
    }

    fn view(&self) -> Element<Message> {
        let pick_list_runtype = row![
            text("Run Type"),
            horizontal_space(Length::Fill),
            pick_list(
                &RunType::ALL[..],
                Some(self.runtype),
                Message::RunTypeSelected
            )
            .placeholder(RunType::default().to_string())
            .width(self.selection_width)
        ]
        .width(Length::Fill);

        let pick_list_hf = row![
            text("Reference"),
            horizontal_space(Length::Fill),
            pick_list(&HFType::ALL[..], Some(self.hftype), Message::HFTypeSelected)
                .placeholder(HFType::default().to_string())
                .width(self.selection_width)
        ]
        .width(Length::Fill);

        let pick_list_correlation = row![
            text("Correlation Method"),
            horizontal_space(Length::Fill),
            pick_list(
                &CorrelationMethod::ALL[..],
                self.correlation_method,
                Message::CorrelationMethodTypeSelected,
            )
            .placeholder(CorrelationMethod::default().to_string())
            .width(self.selection_width)
        ];

        let pick_list_basis_set = row![
            text("Basis Set"),
            horizontal_space(Length::Fill),
            pick_list(
                &BasisSet::ALL[..],
                Some(self.basis_set),
                Message::BasisSetSelected,
            )
            .placeholder(BasisSet::default().to_string())
            .width(self.selection_width)
        ];

        let charge = row![
            text("Charge"),
            horizontal_space(Length::Fill),
            text_input("0", &self.charge_str, Message::ChargeEditing)
                .width(self.selection_width)
                .on_submit(Message::ChargeEdited(self.charge_str.clone()))
        ];

        let multiplicity = row![
            text("Multiplicity"),
            horizontal_space(Length::Fill),
            text_input("1", &self.multiplicity_str, Message::MultiplicityEditing)
                .width(self.selection_width)
                .on_submit(Message::MultiplicityEdited(self.multiplicity_str.clone()))
        ];

        let calculation_settings = column![
            pick_list_runtype,
            pick_list_hf,
            pick_list_correlation,
            pick_list_basis_set,
            charge,
            multiplicity,
            vertical_space(50),
            button("Run")
                .on_press(Message::RunORCA)
                .style(theme::Button::Secondary),
        ]
        .width(Length::Fixed(500.))
        .align_items(Alignment::Center)
        .spacing(10)
        .padding(30);

        let molview = column![text("this is where the molecular editor will be\nIced doesn't support multi-line text input yet, unfortunately, nor have I created a proper molecular viewer")]
            .width(Length::Fill)
            .align_items(Alignment::Center);

        let content = row![molview, calculation_settings];

        container(scrollable(content))
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
