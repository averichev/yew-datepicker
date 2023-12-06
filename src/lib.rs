use chrono::{Datelike, Duration, Month, NaiveDate, Utc};
use chrono::{Locale, Weekday};
use chronoutil::shift_months;
use std::convert::TryFrom;
use std::mem;
use yew::{html, Callback, Component, Context, Html, Properties};

pub struct Datepicker {
    current_date: NaiveDate,
    locale: Locale,
}

#[derive(Properties, PartialEq)]
pub struct DatepickerProperties {
    pub on_select: Callback<NaiveDate>,
    #[prop_or_default]
    pub locale: Option<Locale>,
}

impl Default for DatepickerProperties {
    fn default() -> Self {
        DatepickerProperties {
            on_select: Default::default(),
            locale: Some(Locale::en_US),
        }
    }
}

pub enum DatepickerMessage {
    CurrentMonthChange(NaiveDate),
    Select(u32),
}

impl Component for Datepicker {
    type Message = DatepickerMessage;
    type Properties = DatepickerProperties;

    fn create(ctx: &Context<Self>) -> Self {
        let current_date = chrono::offset::Local::now()
            .date_naive()
            .with_day0(0)
            .unwrap();
        let locale = match ctx.props().locale {
            None => Locale::en_US,
            Some(l) => l,
        };
        Datepicker {
            current_date,
            locale,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            DatepickerMessage::CurrentMonthChange(date) => self.current_date = date,
            DatepickerMessage::Select(date) => {
                let selected_date = self.current_date.with_day(date).unwrap();
                let _ = &ctx.props().on_select.emit(selected_date);
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let columns = self
            .current_week()
            .into_iter()
            .map(|n: NaiveDate| {
                html! {
                   <th>{n.format_localized("%a", self.locale).to_string()}</th>
                }
            })
            .collect::<Html>();

        let date = self.current_date.clone();
        let context = ctx.link().clone();
        let onclick = Callback::from(move |_| {
            context.send_message(DatepickerMessage::CurrentMonthChange(shift_months(
                date, -1,
            )));
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

        let rows = calendarize
            .iter()
            .cloned()
            .map(|n| {
                let cells = n
                    .iter()
                    .cloned()
                    .map(|cl| {
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
                    })
                    .collect::<Html>();
                html! {
                    <tr>
                    {cells}
                    </tr>
                }
            })
            .collect::<Html>();

        html! {
            <table>
                <thead>
                    <tr>
                        <th colspan="7">
                            {prev} {self.current_month_name()} {self.current_date.year()} {next}
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
    fn current_week(&self) -> Vec<NaiveDate> {
        let current = Utc::now().date_naive();
        let week = current.week(Weekday::Mon);
        let first_day = week.first_day();
        let last_day = week.last_day();
        let mut result = Vec::new();
        for day in NaiveDateRange(first_day, last_day) {
            result.push(day);
        }
        result
    }
    fn current_month_name(&self) -> String{
        match self.locale {
            Locale::ru_RU => {
                let month = Month::try_from(self.current_date.month() as u8).unwrap();
                self.russian_month_name(month).to_string()
            }
            _ => {
                self.current_date.format_localized("%B", self.locale).to_string()
            }
        }
    }
    fn russian_month_name(&self, month: Month) -> &'static str {
        match month {
            Month::January => "Январь",
            Month::February => "Февраль",
            Month::March => "Март",
            Month::April => "Апрель",
            Month::May => "Май",
            Month::June => "Июнь",
            Month::July => "Июль",
            Month::August => "Август",
            Month::September => "Сентябрь",
            Month::October => "Октябрь",
            Month::November => "Ноябрь",
            Month::December => "Декабрь",
        }
    }
}

struct NaiveDateRange(NaiveDate, NaiveDate);

impl Iterator for NaiveDateRange {
    type Item = NaiveDate;
    fn next(&mut self) -> Option<Self::Item> {
        if self.0 <= self.1 {
            let next = self.0 + Duration::days(1);
            Some(mem::replace(&mut self.0, next))
        } else {
            None
        }
    }
}
