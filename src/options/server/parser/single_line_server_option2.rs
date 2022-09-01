#[derive(Default, PartialEq, Clone, Debug)]
pub struct OptionsArray<'a>(Vec<Cow<'a, str>>);

impl<'a> OptionsArray<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn push<S: Into<Cow<'a, str>>>(&mut self, item: S) {
        self.0.push(item.into());
    }

    pub fn insert<S: Into<Cow<'a, str>>>(&mut self, i: usize, item: S) {
        self.0.insert(i, item.into());
    }

    pub fn to_string_ext<S: fmt::Display>(self, name: S) -> String {
        let mut s = String::new();
        for (i, item) in self.0.iter().enumerate() {
            s += &format!("{}[{}] {}\n", name, i, item);
        }
        s
    }
}

#[test]
fn options_array() {
    let mut a = OptionsArray::new();
    a.push("asdf1");
    a.push("asdf2");
    let result = a.to_string_ext(TERMINAL_OVERRIDES);
    dbg!(result);
}

pub enum TmuxServerOption {
    A,
}

pub enum TmuxOptionName {
    TmuxServerOption(TmuxServerOption),
    //Session(),
    //Window(),
    //Pane(),
}

//#[derive(Default)]
pub struct TmuxOption<T> {
    pub name: TmuxOptionName,
    pub index: Option<usize>,
    pub value: Option<T>,
}

impl<T: fmt::Display> TmuxOption<T> {}

//#[derive(Default)]
pub struct TmuxServerOption2<T>(pub TmuxOption<T>);
