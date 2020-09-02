use crate::data::{Lecture, Theme, Timetable};
use crate::font;
use crate::sound::play_bell;
use chrono::prelude::*;
use chrono::Duration;
use iced::{
    container, executor, scrollable, time, Align, Application, Background, Color, Column, Command,
    Container, Element, HorizontalAlignment, Length, ProgressBar, Row, Scrollable, Subscription,
    Text,
};
use std::cmp::Ordering;
use std::time::Duration as StdDuration;

pub struct App {
    now: DateTime<FixedOffset>,
    app_start: DateTime<FixedOffset>,
    timetable: Timetable,
    theme: Theme,
    time_kind: TimeKind,
    scroll: scrollable::State,
}

#[derive(Clone, PartialEq)]
pub enum TimeKind {
    Unknown,
    Weekends,
    BeforeSchool {
        lecture_begin: NaiveTime,
    },
    Lecture(usize, Lecture),
    FreeTime {
        prev: (usize, Lecture),
        next: (usize, Option<Lecture>),
    },
    AfterSchool,
}

#[derive(Debug)]
pub enum Message {
    UpdateTime,
    None,
}

#[derive(Default)]
pub struct Flags {
    pub timetable: Timetable,
    pub theme: Theme,
}

fn korean_now() -> DateTime<FixedOffset> {
    FixedOffset::east(9 * 3600).from_utc_datetime(&Utc::now().naive_utc())
}

impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = Flags;

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let Flags { timetable, theme } = flags;
        (
            App {
                now: korean_now(),
                app_start: korean_now(),
                timetable,
                theme,
                time_kind: TimeKind::Unknown,
                scroll: Default::default(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("TimerTable")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        let old_kind = self.time_kind.clone();

        match message {
            Message::UpdateTime => {
                self.now = korean_now();

                let weekday = self.now.weekday().num_days_from_monday() as usize;
                self.time_kind = if weekday > 4 {
                    TimeKind::Weekends
                } else {
                    let day_timetable = &self.timetable[weekday];
                    let now_time = self.now.time();
                    if now_time < day_timetable[0].begin {
                        TimeKind::BeforeSchool {
                            lecture_begin: day_timetable[0].begin,
                        }
                    } else if now_time > day_timetable[day_timetable.len() - 1].end {
                        TimeKind::AfterSchool
                    } else {
                        let result = day_timetable.binary_search_by(|lecture| {
                            if now_time < lecture.begin {
                                Ordering::Greater
                            } else if now_time > lecture.end {
                                Ordering::Less
                            } else {
                                Ordering::Equal
                            }
                        });
                        match result {
                            Ok(index) => TimeKind::Lecture(index, day_timetable[index].clone()),
                            Err(index) => TimeKind::FreeTime {
                                prev: (index - 1, day_timetable[index - 1].clone()),
                                next: (index, day_timetable.get(index).cloned()),
                            },
                        }
                    }
                }
            }
            Message::None => return Command::none(),
        }
        if old_kind != TimeKind::Unknown && old_kind != self.time_kind {
            Command::perform(play_bell(), |_| Message::None)
        } else {
            Command::none()
        }
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        time::every(StdDuration::from_secs(1)).map(|_| Message::UpdateTime)
    }

    fn view(&mut self) -> Element<Self::Message> {
        let time_message = Text::new(format!(
            "지금은 {weekday} {hour:02}시 {minute:02}분 {second:02}초로\n{lecture}",
            weekday = match self.now.weekday() {
                Weekday::Mon => "월요일",
                Weekday::Tue => "화요일",
                Weekday::Wed => "수요일",
                Weekday::Thu => "목요일",
                Weekday::Fri => "금요일",
                Weekday::Sat => "토요일",
                Weekday::Sun => "일요일",
            },
            hour = {
                let (is_pm, hour) = self.now.hour12();

                format!("{} {}", if is_pm { "오후" } else { "오전" }, hour)
            },
            minute = self.now.minute(),
            second = self.now.second(),
            lecture = match &self.time_kind {
                TimeKind::Unknown => String::new(),
                TimeKind::Weekends => String::from("주말입니다"),
                TimeKind::BeforeSchool { .. } => String::from("오늘 수업을 위해 준비할 시간입니다"),
                TimeKind::AfterSchool => String::from("오늘 수업이 끝났습니다"),
                TimeKind::Lecture(_, lecture) =>
                    format!("{} 수업을 듣는 중입니다", lecture.subject),
                TimeKind::FreeTime {
                    next: (_, next), ..
                } => format!(
                    "쉬는 시간입니다 (다음 교시 {})",
                    next.clone()
                        .map(|lecture| lecture.subject)
                        .unwrap_or_else(|| String::from("없음"))
                ),
            }
        ))
        .font(font::TEXT)
        .width(Length::Fill)
        .size(50)
        .horizontal_alignment(HorizontalAlignment::Center);

        let duration = match &self.time_kind {
            TimeKind::BeforeSchool { lecture_begin } => Some((
                lecture_begin.clone() - self.now.time(),
                lecture_begin.clone() - self.app_start.time(),
            )),
            TimeKind::Lecture(_, lecture) => Some((
                lecture.end.clone() - self.now.time(),
                lecture.end.clone() - lecture.begin.clone(),
            )),
            TimeKind::FreeTime {
                prev: (_, prev),
                next: (_, Some(next)),
                ..
            } => Some((
                next.begin.clone() - self.now.time(),
                next.begin.clone() - prev.end.clone(),
            )),
            _ => None,
        };

        let progress_bar = ProgressBar::new(
            0.0..=1.0,
            match &duration {
                Some((curr, max)) => 1.0 - curr.num_seconds() as f32 / max.num_seconds() as f32,
                _ => 1.0,
            },
        )
        .height(Length::Units(4));

        let progress_bar_real_time = Text::new(match &duration {
            Some((curr, max)) => {
                let max_real_seconds = max.num_seconds();

                let max_seconds = max_real_seconds % 60;
                let max_minutes = max_real_seconds / 60;

                let left_real_seconds = curr.num_seconds();

                let left_seconds = left_real_seconds % 60;
                let left_minutes = left_real_seconds / 60;

                let percentage = (max_real_seconds as f64 - left_real_seconds as f64) * 100.0
                    / (max_real_seconds as f64);

                format!(
                    "{:02}분 {:02}초 중 {:02}분 {:02}초 남음 ({:.1}% 완료)",
                    max_minutes, max_seconds, left_minutes, left_seconds, percentage
                )
            }
            None => String::new(),
        });

        let timetable = {
            let highlight_x = self.now.weekday().num_days_from_monday() as usize;
            let highlight_y = match self.time_kind {
                _ if highlight_x > 4 => vec![],
                TimeKind::Lecture(id, ..) => vec![id],
                TimeKind::FreeTime {
                    prev: (prev, _),
                    next: (next, _),
                } => vec![prev, next],
                _ => vec![],
            };
            let min_time = self
                .timetable
                .iter()
                .flat_map(|lecture_vec| lecture_vec.iter().map(|lecture| lecture.begin))
                .min()
                .expect("Should have one or more lectures");
            let max_time = self
                .timetable
                .iter()
                .flat_map(|lecture_vec| lecture_vec.iter().map(|lecture| lecture.end))
                .max()
                .expect("Should have one or more lectures");
            let should_show_time: Vec<_> = self
                .timetable
                .iter()
                .flat_map(|lecture_vec| {
                    lecture_vec
                        .iter()
                        .flat_map(|lecture| vec![lecture.begin, lecture.end].into_iter())
                })
                .collect();

            let mut every_five_minutes = Vec::new();
            let mut curr = min_time.clone();
            while curr <= max_time {
                every_five_minutes.push(curr.clone());
                curr += Duration::minutes(5);
            }

            let mut time_panel = Column::new();
            for time in &every_five_minutes {
                time_panel = time_panel.push(Text::new(if should_show_time.contains(&time) {
                    time.format("%H:%M:%S").to_string()
                } else {
                    String::from(" ")
                }));
            }
            let item_per_day = ((max_time - min_time).num_minutes() / 5 + 1) as f64;

            let mut row = Row::new().spacing(16).push(time_panel);
            for (x, day_timetable) in self.timetable.iter().enumerate() {
                let mut column = Column::new()
                    .width(Length::Fill)
                    .height(Length::Units((item_per_day * 20.12) as u16))
                    .push(Text::new("").height(Length::Units(8)));
                let mut current_lecture = None::<Lecture>;
                let mut n_five_minutes = 0;
                let mut last_lecture_index = 0;

                for time in &every_five_minutes {
                    if let Some(lecture) = &current_lecture {
                        if lecture.end <= *time {
                            let subject = lecture.subject.clone();
                            let mut foreground = self
                                .theme
                                .get(&subject)
                                .map(|cell_color| cell_color.foreground.clone().into())
                                .unwrap_or_else(|| Color::BLACK);

                            let mut background = self
                                .theme
                                .get(&subject)
                                .map(|cell_color| cell_color.background.clone().into())
                                .unwrap_or_else(|| Color::TRANSPARENT);
                            if x != highlight_x {
                                foreground.a *= 0.8;
                                background.a *= 0.8;
                            };
                            column = column.push(
                                Container::new(Text::new(subject).size(48).color(foreground))
                                    .style(TableCell(
                                        Some(Background::Color(background)),
                                        if x == highlight_x
                                            && highlight_y.contains(&last_lecture_index)
                                        {
                                            3
                                        } else {
                                            1
                                        },
                                    ))
                                    .width(Length::Fill)
                                    .height(Length::FillPortion(n_five_minutes))
                                    .center_x()
                                    .center_y(),
                            );
                            current_lecture = None;
                            n_five_minutes = 0;
                            last_lecture_index += 1;
                        }
                    } else {
                        let lecture = day_timetable.get(last_lecture_index).cloned();
                        if let Some(lecture) = lecture {
                            if lecture.begin <= *time {
                                if n_five_minutes > 0 {
                                    column = column.push(
                                        Container::new(Text::new(""))
                                            .width(Length::Fill)
                                            .height(Length::FillPortion(n_five_minutes)),
                                    );
                                }
                                current_lecture = Some(lecture);
                                n_five_minutes = 0;
                            }
                        }
                    }
                    n_five_minutes += 1;
                }
                if n_five_minutes > 0 {
                    column = column.push(
                        Container::new(Text::new(""))
                            .width(Length::Fill)
                            .height(Length::FillPortion(n_five_minutes)),
                    );
                }
                row = row.push(column);
            }
            row
        };

        Column::new()
            .align_items(Align::Center)
            .padding(4)
            .width(Length::Fill)
            .height(Length::Fill)
            .push(time_message)
            .push(progress_bar)
            .push(progress_bar_real_time)
            .push(
                Container::new(
                    Scrollable::new(&mut self.scroll)
                        .padding(32)
                        .push(timetable),
                )
                .padding(4),
            )
            .into()
    }
}

struct TableCell(Option<Background>, u16);

impl container::StyleSheet for TableCell {
    fn style(&self) -> container::Style {
        container::Style {
            background: self.0,
            border_color: Color::BLACK,
            border_width: self.1,
            ..Default::default()
        }
    }
}
