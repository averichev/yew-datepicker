use chrono::{Datelike, Duration, Month, Months, NaiveDate, Utc};
use chrono::{Locale, Weekday};
use chronoutil::shift_months;

use std::convert::TryFrom;
use std::mem;

use std::str::FromStr;
use stylist::ast::Sheet;
use stylist::Style;
use yew::MouseEvent;
use yew::{html, Callback, Component, Context, Html, Properties};
use yew_template::template_html;

pub struct Datepicker {
    current_month: NaiveDate,
    locale: Locale,
    selected_date: Option<NaiveDate>,
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
    Select(NaiveDate),
}

impl Component for Datepicker {
    type Message = DatepickerMessage;
    type Properties = DatepickerProperties;

    fn create(ctx: &Context<Self>) -> Self {
        let current_date = chrono::offset::Local::now()
            .date_naive()
            .with_day0(0)
            .unwrap();
        let locale = ctx.props().locale.unwrap_or_else(|| Locale::en_US);
        Datepicker {
            current_month: current_date,
            locale,
            selected_date: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            DatepickerMessage::CurrentMonthChange(date) => self.current_month = date,
            DatepickerMessage::Select(selected) => {
                self.selected_date = Some(selected);
                let _ = ctx.props().on_select.emit(selected);
                let in_current_mont_selected = selected.year() == self.current_month.year()
                    && selected.month() == self.current_month.month();
                if !in_current_mont_selected {
                    ctx.link()
                        .send_message(DatepickerMessage::CurrentMonthChange(
                            selected.with_day0(0).unwrap(),
                        ))
                }
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        const STYLE_FILE: &str = include_str!("styles.css");
        let sheet = Sheet::from_str(STYLE_FILE).unwrap();
        let style = Style::new(sheet).unwrap();
        let days_names = self
            .current_week()
            .into_iter()
            .map(|n: NaiveDate| {
                html! {
                   <div class="day">{n.format_localized("%a", self.locale).to_string()}</div>
                }
            })
            .collect::<Html>();
        let columns = html! {
            <>
            <div class="day"></div>
            {days_names}
            </>
        };

        let date = self.current_month.clone();
        let context = ctx.link().clone();
        let onclick = Callback::from(move |_| {
            context.send_message(DatepickerMessage::CurrentMonthChange(shift_months(
                date, -1,
            )));
        });
        let prev = html! {
            <button class="btn" {onclick} type="button">{"<"}</button>
        };

        let context = ctx.link().clone();
        let onclick_next = Callback::from(move |_| {
            context.send_message(DatepickerMessage::CurrentMonthChange(shift_months(date, 1)));
        });
        let next = html! {
            <button class="btn" onclick={onclick_next} type="button">{">"}</button>
        };

        let calendarize = calendarize::calendarize_with_offset(self.current_month, 1);
        let selected_day = self.selected_date;
        let current_month = self.current_month;

        let previous_month = self
            .current_month
            .checked_sub_months(Months::new(1))
            .unwrap();

        let prev_month = calendarize::calendarize_with_offset(previous_month, 1);
        let prev_month_last_week = prev_month.last().unwrap();

        let next_month = self
            .current_month
            .checked_add_months(Months::new(1))
            .unwrap();
        let next_month_calendarize = calendarize::calendarize_with_offset(next_month, 1);
        let next_month_first_week = next_month_calendarize.first().unwrap();

        let weeks_number = calendarize.len();

        let rows = calendarize
            .iter()
            .cloned()
            .enumerate()
            .map(|(week_index, n)| {
                let week_number = week_index + 1;
                let is_first_week = week_index == 0;
                let is_last_week = weeks_number == week_number;
                let cells = n
                    .iter()
                    .cloned()
                    .enumerate()
                    .map(|(day_of_month_index, cl)| {
                        let mut day_class = String::from("day");
                        let context = ctx.link().clone();
                        let current_iter_date: Option<NaiveDate>;

                        let number: String;
                        if cl > 0 {
                            number = cl.to_string();
                            current_iter_date = current_month.with_day(cl);
                        } else {
                            if is_first_week {
                                let prev_month_last_week_day =
                                    prev_month_last_week.get(day_of_month_index).unwrap();
                                number = prev_month_last_week_day.to_string();
                                day_class.push_str(" day--nc-month");
                                current_iter_date =
                                    previous_month.with_day(*prev_month_last_week_day);
                            } else {
                                if is_last_week {
                                    let next_month_day =
                                        next_month_first_week.get(day_of_month_index).unwrap();
                                    number = next_month_day.to_string();
                                    day_class.push_str(" day--nc-month");
                                    current_iter_date = next_month.with_day(*next_month_day);
                                } else {
                                    number = String::new();
                                    current_iter_date = None;
                                }
                            }
                        }
                        match selected_day {
                            None => {}
                            Some(s) => match current_iter_date {
                                None => {}
                                Some(sl) => {
                                    if s == sl {
                                        day_class.push_str(" day--selected");
                                    }
                                }
                            },
                        }
                        let onclick: Callback<MouseEvent> =
                            Callback::from(move |event: MouseEvent| {
                                event.prevent_default();
                                match current_iter_date {
                                    None => {}
                                    Some(s) => {
                                        context.send_message(DatepickerMessage::Select(s));
                                    }
                                }
                            });
                        html! {
                            <a class={day_class} {onclick} href="#">{number}</a>
                        }
                    })
                    .collect::<Html>();
                html! {
                    <>
                    <div>{week_number}</div>
                    {cells}
                    </>
                }
            })
            .collect::<Html>();
        let class = style.get_class_name().to_string();
        let header = format!(
            "{} {}",
            self.current_month_name(),
            self.current_month.year()
        );
        template_html!(
            "src/templates/default.html",
            prev = { prev },
            next = { next },
            columns = { columns },
            rows = { rows },
            class = { class },
            header = { header }
        )
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
    fn current_month_name(&self) -> String {
        match self.locale {
            Locale::ru_RU => {
                let month = Month::try_from(self.current_month.month() as u8).unwrap();
                self.russian_month_name(month).to_string()
            }
            _ => self
                .current_month
                .format_localized("%B", self.locale)
                .to_string(),
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
