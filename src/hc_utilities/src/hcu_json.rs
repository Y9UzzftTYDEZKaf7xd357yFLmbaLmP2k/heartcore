use core::panic;
use jaq_core::{load, Ctx, RcIter};
use jaq_json::Val;
use load::{Arena, File, Loader};
use std::fmt;
use core::fmt::{Display, Formatter};

#[macro_export]
macro_rules! json_encode {
    ($x:expr) => {
        // The $crate prefix is used to refer to the current crate, so that the macro can be used in other crates.
        $crate::hc_utilities_serde_json::to_string(&$crate::hc_utilities_serde_json_json!($x)).unwrap()
    };
}

/* Remaining code is based on https://github.com/01mf02/jaq
    License:

    Permission is hereby granted, free of charge, to any
    person obtaining a copy of this software and associated
    documentation files (the "Software"), to deal in the
    Software without restriction, including without
    limitation the rights to use, copy, modify, merge,
    publish, distribute, sublicense, and/or sell copies of
    the Software, and to permit persons to whom the Software
    is furnished to do so, subject to the following
    conditions:

    The above copyright notice and this permission notice
    shall be included in all copies or substantial portions
    of the Software.

    THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
    ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
    TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
    PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
    SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
    CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
    OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
    IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
    DEALINGS IN THE SOFTWARE.
*/

pub fn jq(query: &str, input: &str) -> String {

    let program = File {
        code: query,
        path: (),
    };

    let loader = Loader::new(jaq_std::defs().chain(jaq_json::defs()));
    let arena = Arena::default();

    let modules = loader.load(&arena, program).unwrap();

    let filter = jaq_core::Compiler::default()
        .with_funs(jaq_std::funs().chain(jaq_json::funs()))
        .compile(modules)
        .unwrap();

    let inputs = RcIter::new(core::iter::empty());

    let mut out = filter.run((Ctx::new([], &inputs), Val::from(input.to_string())));

    let cli = Cli {
        compact_output: false,
        raw_output: false,
        join_output: false,
        in_place: false,
        sort_keys: false,
        color_output: false,
        monochrome_output: false,
        tab: false,
        indent: 2,
    };
    let mut result;
    result = "".to_string();
    while let Ok(val) = out.next().unwrap() {

        let f = |f: &mut Formatter| {
            let opts = PpOpts {
                compact: cli.compact_output,
                indent: if cli.tab {
                    String::from("\t")
                } else {
                    " ".repeat(cli.indent)
                },
                sort_keys: cli.sort_keys,
            };
            fmt_val(f, &opts, 0, &val)
        };
        if let Val::Str(s) = &val {
            if cli.raw_output || cli.join_output {
                result = format!("{}{}", result, s);
            } else {
                result = format!("{}{}", result, FormatterFn(f));
            }
        } else {
            result = format!("{}{}", result, FormatterFn(f));
        }
        return result;
    }

    panic!("Error formatting json");
}

pub struct Cli {
    // see https://github.com/01mf02/jaq/blob/main/jaq/src/cli.rs
    pub compact_output: bool,
    pub raw_output: bool,
    pub join_output: bool,
    pub in_place: bool,
    pub sort_keys: bool,
    pub color_output: bool,
    pub monochrome_output: bool,
    pub tab: bool,
    pub indent: usize,
}

// see https://github.com/01mf02/jaq/blob/main/jaq/src/main.rs
struct FormatterFn<F>(F);

impl<F: Fn(&mut Formatter) -> fmt::Result> Display for FormatterFn<F> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.0(f)
    }
}

struct PpOpts {
    compact: bool,
    indent: String,
    sort_keys: bool,
}

impl PpOpts {
    fn indent(&self, f: &mut Formatter, level: usize) -> fmt::Result {
        if !self.compact {
            write!(f, "{}", self.indent.repeat(level))?;
        }
        Ok(())
    }

    fn newline(&self, f: &mut Formatter) -> fmt::Result {
        if !self.compact {
            writeln!(f)?;
        }
        Ok(())
    }
}

fn fmt_seq<T, I, F>(fmt: &mut Formatter, opts: &PpOpts, level: usize, xs: I, f: F) -> fmt::Result
where
    I: IntoIterator<Item = T>,
    F: Fn(&mut Formatter, T) -> fmt::Result,
{
    opts.newline(fmt)?;
    let mut iter = xs.into_iter().peekable();
    while let Some(x) = iter.next() {
        opts.indent(fmt, level + 1)?;
        f(fmt, x)?;
        if iter.peek().is_some() {
            write!(fmt, ",")?;
        }
        opts.newline(fmt)?;
    }
    opts.indent(fmt, level)
}

fn fmt_val(f: &mut Formatter, opts: &PpOpts, level: usize, v: &Val) -> fmt::Result {
    use yansi::Paint;
    match v {
        Val::Null | Val::Bool(_) | Val::Int(_) | Val::Float(_) | Val::Num(_) => v.fmt(f),
        Val::Str(_) => write!(f, "{}", v.green()),
        Val::Arr(a) => {
            '['.bold().fmt(f)?;
            if !a.is_empty() {
                fmt_seq(f, opts, level, &**a, |f, x| fmt_val(f, opts, level + 1, x))?;
            }
            ']'.bold().fmt(f)
        }
        Val::Obj(o) => {
            '{'.bold().fmt(f)?;
            let kv = |f: &mut Formatter, (k, val): (&std::rc::Rc<String>, &Val)| {
                write!(f, "{:?}:", k.bold())?;
                if !opts.compact {
                    write!(f, " ")?;
                }
                fmt_val(f, opts, level + 1, val)
            };
            if !o.is_empty() {
                if opts.sort_keys {
                    let mut o: Vec<_> = o.iter().collect();
                    o.sort_by_key(|(k, _v)| *k);
                    fmt_seq(f, opts, level, o, kv)
                } else {
                    fmt_seq(f, opts, level, &**o, kv)
                }?
            }
            '}'.bold().fmt(f)
        }
    }
}