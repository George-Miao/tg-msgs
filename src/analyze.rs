use std::{
    collections::BTreeMap,
    io::{Result as IoResult, Write},
};

use url::Url;

use crate::model::*;

pub struct Analyzer<'a> {
    wrap_with: Option<&'a str>,
    write_to: Box<dyn Write>,
    data: &'a ChatData,
    take_num: usize,
    opt_out: &'a [i64],
}

impl<'a> Analyzer<'a> {
    pub fn new(data: &'a ChatData) -> Self {
        Analyzer {
            wrap_with: None,
            take_num: usize::MAX,
            write_to: Box::new(std::io::stdout()),
            opt_out: &[],
            data,
        }
    }

    pub fn write_to(mut self, write_to: impl Write + 'static) -> Self {
        self.write_to = Box::new(write_to);
        self
    }

    pub fn wrap_with(mut self, wrap_with: &'a str) -> Self {
        self.wrap_with = Some(wrap_with);
        self
    }

    pub fn take(mut self, take_num: usize) -> Self {
        self.take_num = take_num;
        self
    }

    pub fn opt_out(mut self, opt_out: &'a [i64]) -> Self {
        self.opt_out = opt_out;
        self
    }

    fn msgs(&self) -> impl Iterator<Item = &Message> {
        self.data
            .msgs()
            .filter(|msg| !self.opt_out.contains(&msg.sender_id().unwrap().as_num()))
    }

    fn wrap(&mut self) -> IoResult<()> {
        if let Some(wrap_with) = self.wrap_with {
            writeln!(&mut self.write_to, "{}", wrap_with)?;
        }
        Ok(())
    }

    pub fn sender_rank(mut self) -> IoResult<Self> {
        let total = self.data.messages.len();
        let mut users = BTreeMap::new();

        for msg in self.msgs() {
            let name = msg.sender_name().unwrap();
            users.entry(name).and_modify(|x| *x += 1).or_insert(1);
        }

        self.wrap()?;

        let w = &mut self.write_to;
        writeln!(w, "Total messages: {total}\n")?;

        let mut users = users.into_iter().collect::<Vec<_>>();
        users.sort_by_key(|(_, v)| *v);

        for (k, v) in users.into_iter().rev().take(self.take_num) {
            let percentage = (v as f64 / total as f64) * 100.0;
            writeln!(w, "({v}) ({percentage:.2}%) {k}")?;
        }

        self.wrap()?;

        Ok(self)
    }

    pub fn count_substring(mut self, string: &str) -> IoResult<Self> {
        let mut total = 0;
        let mut users = BTreeMap::new();

        for msg in self.msgs() {
            let name = msg.sender_name().unwrap();
            let count = msg.count(string);

            if count == 0 {
                continue;
            }

            total += count;
            users
                .entry(name)
                .and_modify(|x| *x += count)
                .or_insert(count);
        }

        self.wrap()?;

        let w = &mut self.write_to;
        writeln!(w, "Total `{string}`: {total}\n")?;

        let mut users = users.into_iter().collect::<Vec<_>>();
        users.sort_by_key(|(_, v)| *v);

        for (k, v) in users.into_iter().rev().take(self.take_num) {
            let percentage = (v as f64 / total as f64) * 100.0;
            writeln!(w, "({v}) ({percentage:.2}%) {k}")?;
        }

        self.wrap()?;
        Ok(self)
    }

    pub fn count_link(mut self) -> IoResult<Self> {
        let mut users = BTreeMap::new();
        let mut domains = BTreeMap::new();
        let mut total = 0;

        for msg in self.msgs() {
            let link_count = msg
                .text
                .as_entities()
                .filter_map(|s| {
                    let url = match s.text_type {
                        TextType::Link => {
                            let url = if s.text.contains("://") {
                                s.text.to_owned()
                            } else {
                                format!("http://{}", s.text)
                            };
                            Url::parse(&url).unwrap()
                        }
                        TextType::TextLink => s.href.clone().unwrap(),
                        _ => return None,
                    };

                    if let Some(domain) = url.domain() {
                        domains
                            .entry(domain.to_owned())
                            .and_modify(|x| *x += 1)
                            .or_insert(1);
                    }

                    Some(())
                })
                .count();

            if link_count != 0 {
                total += link_count;
                let user = msg.sender_name().unwrap();
                users
                    .entry(user)
                    .and_modify(|x| *x += link_count)
                    .or_insert(link_count);
            }
        }

        let mut users = users.into_iter().collect::<Vec<_>>();
        let mut domains = domains.into_iter().collect::<Vec<_>>();
        users.sort_by_key(|(_, v)| *v);
        domains.sort_by_key(|(_, v)| *v);

        {
            self.wrap()?;
            let w = &mut self.write_to;

            writeln!(w, "Total links:")?;
            writeln!(w, "# Top users sending links:\n")?;

            for (k, v) in users.into_iter().rev().take(self.take_num) {
                let percentage = (v as f64 / total as f64) * 100.0;
                writeln!(w, "({v}) ({percentage:.2}%) {k}")?;
            }
            self.wrap()?;
        }

        {
            self.wrap()?;
            let w = &mut self.write_to;

            writeln!(w, "# Top domains:\n")?;

            for (k, v) in domains.into_iter().rev().take(self.take_num) {
                let percentage = (v as f64 / total as f64) * 100.0;
                writeln!(w, "({v}) ({percentage:.2}%) {k}")?;
            }

            self.wrap()?;
        }
        Ok(self)
    }
}
