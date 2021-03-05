pub use clap::{App, ArgMatches, SubCommand};

use std::collections::HashMap;

pub struct CliApp<'a: 'b, 'b> {
    pub app: App<'a, 'b>,
    pub callback: Box<dyn Fn(&ArgMatches) + 'a>,
}

impl<'a, 'b> CliApp<'a, 'b> {
    pub fn new<F>(app: App<'a, 'b>, f: F) -> CliApp<'a, 'b>
    where
        F: Fn(&ArgMatches) + 'a,
    {
        return CliApp {
            app: app,
            callback: Box::new(f),
        };
    }
    pub fn invoke(&self, m: &ArgMatches) {
        (self.callback)(m)
    }
}

pub fn cmd_group<'a, 'b>(name: &'a str, cmds: Vec<CliApp<'a, 'b>>) -> CliApp<'a, 'b> {
    let mut cbs: HashMap<String, _> = HashMap::with_capacity(cmds.len());
    let mut subs = Vec::with_capacity(cmds.len());
    for a in cmds {
        cbs.insert(a.app.get_name().to_owned(), a.callback);
        subs.push(a.app);
    }
    let app: App<'a, 'b> = App::new(name).subcommands(subs);

    let callback = Box::new(move |m: &ArgMatches| {
        if let (cmd, Some(matches)) = m.subcommand() {
            let cb = cbs.get(cmd).unwrap();
            cb(matches);
        }
    });
    return CliApp {
        app: app,
        callback: callback,
    };
}
