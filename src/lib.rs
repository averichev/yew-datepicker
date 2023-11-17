use std::convert::TryFrom;
use chrono::{Datelike, Month, NaiveDate};
use chrono::Weekday;
use chronoutil::shift_months;
use yew::{Callback, Component, Context, Html, html, Properties};


pub struct Datepicker {
    current_date: NaiveDate,
}


#[derive(Default, Properties, PartialEq)]
pub struct DatepickerProperties {
    pub on_select: Callback<NaiveDate>,
}

pub enum DatepickerMessage {
    CurrentMonthChange(NaiveDate),
    Select(u32),
}

impl Component for Datepicker {
    type Message = DatepickerMessage;
    type Properties = DatepickerProperties;

    fn create(_ctx: &Context<Self>) -> Self {
        let current_date = chrono::offset::Local::now().date_naive()
            .with_day0(0)
            .unwrap();
        Datepicker { current_date }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            DatepickerMessage::CurrentMonthChange(date) => {
                self.current_date = date
            }
            DatepickerMessage::Select(date) => {
                let selected_date = self.current_date.with_day(date).unwrap();
                let _ = &ctx.props().on_select.emit(selected_date);
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let all_days_of_week: Vec<Weekday> = (0..7)
            .map(|i| Weekday::try_from(i).unwrap())
            .collect();
        let columns = all_days_of_week.iter().map(|n|
            html! {
                            <th>{self.weekday_number_to_string(n)}</th>
                         }
        ).collect::<Html>();

        let date = self.current_date.clone();
        let context = ctx.link().clone();
        let onclick = Callback::from(move |_| {
            context.send_message(DatepickerMessage::CurrentMonthChange(shift_months(date, -1)));
        });
        let prev = html! {
                    <button {onclick} type="button">{"<"}</button>
                };

        let context = ctx.link().clone();
        let onclick_next = Callback::from(move |_| {
            context.send_message(DatepickerMessage::CurrentMonthChange(shift_months(date, 1)));
        });
        let next = html! {
                    <button onclick={onclick_next} type="button">{">"}</button>
                };

        let calendarize = calendarize::calendarize_with_offset(self.current_date, 1);

        let rows = calendarize.iter().cloned().map(|n| {
            let cells = n.iter().cloned().map(|cl| {
                let context = ctx.link().clone();
                let onclick = Callback::from(move |_| {
                    context.send_message(DatepickerMessage::Select(cl));
                });
                let mut number = String::new();
                if cl > 0 {
                    number = cl.to_string();
                }
                html! {
                    <td {onclick}>{number}</td>
                }
            }).collect::<Html>();
            html! {
                <tr>
                {cells}
                </tr>
            }
        }).collect::<Html>();

        let month_name = Month::try_from(self.current_date.month() as u8).unwrap();

        html! {
            <table>
                <thead>
                    <tr>
                        <th colspan="7">
                            {prev} {month_name.name()} {self.current_date.year()} {next}
                        </th>
                    </tr>
                    <tr>
                        {columns}
                    </tr>
                </thead>
                <tbody>
                    {rows}
                </tbody>
            </table>
        }
    }
}

impl Datepicker {
    fn weekday_number_to_string(&self, weekday: &Weekday) -> &'static str {
        match weekday {
            Weekday::Mon => "Пн",
            Weekday::Tue => "Вт",
            Weekday::Wed => "Ср",
            Weekday::Thu => "Чт",
            Weekday::Fri => "Пт",
            Weekday::Sat => "Сб",
            Weekday::Sun => "Вс",
        }
    }
}


