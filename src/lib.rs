use chrono::{Datelike, Duration, Month, NaiveDate, Utc};
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
                let _ = &ctx.props().on_select.emit(selected);
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        const STYLE_FILE: &str = include_str!("styles.css");
        let sheet = Sheet::from_str(STYLE_FILE).unwrap();
        let style = Style::new(sheet).unwrap();
        let columns = self
            .current_week()
            .into_iter()
            .map(|n: NaiveDate| {
                html! {
                   <div class="day">{n.format_localized("%a", self.locale).to_string()}</div>
                }
            })
            .collect::<Html>();

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

        let rows = calendarize
            .iter()
            .cloned()
            .map(|n| {
                let cells = n
                    .iter()
                    .cloned()
                    .map(|cl| {
                        let context = ctx.link().clone();
                        let selected = current_month.with_day(cl);
                        let onclick: Callback<MouseEvent> =
                            Callback::from(move |event: MouseEvent| {
                                event.prevent_default();
                                match selected {
                                    None => {}
                                    Some(s) => {
                                        context.send_message(DatepickerMessage::Select(s));
                                    }
                                }
                            });
                        let mut number = String::new();
                        if cl > 0 {
                            number = cl.to_string();
                        }
                        let mut day_class = String::from("day");
                        match selected_day {
                            None => {}
                            Some(s) => match selected {
                                None => {}
                                Some(sl) => {
                                    if s == sl {
                                        day_class.push_str(" day--selected");
                                    }
                                }
                            },
                        }

                        html! {
                            <a class={day_class} {onclick} href="#">{number}</a>
                        }
                    })
                    .collect::<Html>();
                html! {
                    {cells}
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
