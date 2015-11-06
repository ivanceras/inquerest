#![allow(non_snake_case, unused)]
use super::*;
use self::RuleResult::{Matched, Failed};
fn escape_default(s: &str) -> String {
    s.chars().flat_map(|c| c.escape_default()).collect()
}
fn char_range_at(s: &str, pos: usize) -> (char, usize) {
    let c = &s[pos..].chars().next().unwrap();
    let next_pos = pos + c.len_utf8();
    (*c, next_pos)
}
#[derive(Clone)]
enum RuleResult<T> { Matched(usize, T), Failed, }
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ParseError {
    pub line: usize,
    pub column: usize,
    pub offset: usize,
    pub expected: ::std::collections::HashSet<&'static str>,
}
pub type ParseResult<T> = Result<T, ParseError>;
impl ::std::fmt::Display for ParseError {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter)
     -> ::std::result::Result<(), ::std::fmt::Error> {
        try!(write ! (
             fmt , "error at {}:{}: expected " , self . line , self . column
             ));
        if self.expected.len() == 0 {
            try!(write ! ( fmt , "EOF" ));
        } else if self.expected.len() == 1 {
            try!(write ! (
                 fmt , "`{}`" , escape_default (
                 self . expected . iter (  ) . next (  ) . unwrap (  ) ) ));
        } else {
            let mut iter = self.expected.iter();
            try!(write ! (
                 fmt , "one of `{}`" , escape_default (
                 iter . next (  ) . unwrap (  ) ) ));
            for elem in iter {
                try!(write ! ( fmt , ", `{}`" , escape_default ( elem ) ));
            }
        }
        Ok(())
    }
}
impl ::std::error::Error for ParseError {
    fn description(&self) -> &str { "parse error" }
}
fn slice_eq(input: &str, state: &mut ParseState, pos: usize, m: &'static str)
 -> RuleResult<()> {
    #![inline]
    #![allow(dead_code)]
    let l = m.len();
    if input.len() >= pos + l &&
           &input.as_bytes()[pos..pos + l] == m.as_bytes() {
        Matched(pos + l, ())
    } else { state.mark_failure(pos, m) }
}
fn slice_eq_case_insensitive(input: &str, state: &mut ParseState, pos: usize,
                             m: &'static str) -> RuleResult<()> {
    #![inline]
    #![allow(dead_code)]
    let mut used = 0usize;
    let mut input_iter = input[pos..].chars().flat_map(|x| x.to_uppercase());
    for m_char_upper in m.chars().flat_map(|x| x.to_uppercase()) {
        used += m_char_upper.len_utf8();
        let input_char_result = input_iter.next();
        if input_char_result.is_none() ||
               input_char_result.unwrap() != m_char_upper {
            return state.mark_failure(pos, m);
        }
    }
    Matched(pos + used, ())
}
fn any_char(input: &str, state: &mut ParseState, pos: usize)
 -> RuleResult<()> {
    #![inline]
    #![allow(dead_code)]
    if input.len() > pos {
        let (_, next) = char_range_at(input, pos);
        Matched(next, ())
    } else { state.mark_failure(pos, "<character>") }
}
fn pos_to_line(input: &str, pos: usize) -> (usize, usize) {
    let mut remaining = pos;
    let mut lineno: usize = 1;
    for line in input.lines() {
        let line_length = line.len() + 1;
        if remaining < line_length { return (lineno, remaining + 1); }
        remaining -= line_length;
        lineno += 1;
    }
    return (lineno, remaining + 1);
}
struct ParseState<'input> {
    max_err_pos: usize,
    expected: ::std::collections::HashSet<&'static str>,
    _phantom: ::std::marker::PhantomData<&'input ()>,
}
impl <'input> ParseState<'input> {
    fn new() -> ParseState<'input> {
        ParseState{max_err_pos: 0,
                   expected: ::std::collections::HashSet::new(),
                   _phantom: ::std::marker::PhantomData,}
    }
    fn mark_failure(&mut self, pos: usize, expected: &'static str)
     -> RuleResult<()> {
        if pos > self.max_err_pos {
            self.max_err_pos = pos;
            self.expected.clear();
        }
        if pos == self.max_err_pos { self.expected.insert(expected); }
        Failed
    }
}
fn parse_name<'input>(input: &'input str, state: &mut ParseState<'input>,
                      pos: usize) -> RuleResult<String> {
    {
        let start_pos = pos;
        {
            let seq_res =
                {
                    let mut repeat_pos = pos;
                    let mut repeat_value = vec!();
                    loop  {
                        let pos = repeat_pos;
                        let step_res =
                            if input.len() > pos {
                                let (ch, next) = char_range_at(input, pos);
                                match ch {
                                    'a' ...'z' | 'A' ...'Z' | '0' ...'9' | '_'
                                    => Matched(next, ()),
                                    _ =>
                                    state.mark_failure(pos, "[a-zA-Z0-9_]"),
                                }
                            } else {
                                state.mark_failure(pos, "[a-zA-Z0-9_]")
                            };
                        match step_res {
                            Matched(newpos, value) => {
                                repeat_pos = newpos;
                                repeat_value.push(value);
                            }
                            Failed => { break ; }
                        }
                    }
                    if repeat_value.len() >= 1usize {
                        Matched(repeat_pos, ())
                    } else { Failed }
                };
            match seq_res {
                Matched(pos, _) => {
                    {
                        let match_str = &input[start_pos..pos];
                        Matched(pos, { match_str.to_string() })
                    }
                }
                Failed => Failed,
            }
        }
    }
}
fn parse_equation<'input>(input: &'input str, state: &mut ParseState<'input>,
                          pos: usize) -> RuleResult<Equation> {
    {
        let start_pos = pos;
        {
            let seq_res = parse_operand(input, state, pos);
            match seq_res {
                Matched(pos, l) => {
                    {
                        let seq_res = slice_eq(input, state, pos, "=");
                        match seq_res {
                            Matched(pos, _) => {
                                {
                                    let seq_res =
                                        parse_operand(input, state, pos);
                                    match seq_res {
                                        Matched(pos, r) => {
                                            {
                                                let match_str =
                                                    &input[start_pos..pos];
                                                Matched(pos,
                                                        {
                                                            Equation{left: l,
                                                                     right:
                                                                         r,}
                                                        })
                                            }
                                        }
                                        Failed => Failed,
                                    }
                                }
                            }
                            Failed => Failed,
                        }
                    }
                }
                Failed => Failed,
            }
        }
    }
}
fn parse_operand<'input>(input: &'input str, state: &mut ParseState<'input>,
                         pos: usize) -> RuleResult<Operand> {
    {
        let choice_res =
            {
                let start_pos = pos;
                {
                    let seq_res = parse_name(input, state, pos);
                    match seq_res {
                        Matched(pos, c) => {
                            {
                                let match_str = &input[start_pos..pos];
                                Matched(pos, { Operand::Column(c) })
                            }
                        }
                        Failed => Failed,
                    }
                }
            };
        match choice_res {
            Matched(pos, value) => Matched(pos, value),
            Failed => {
                let start_pos = pos;
                {
                    let seq_res = parse_function(input, state, pos);
                    match seq_res {
                        Matched(pos, f) => {
                            {
                                let match_str = &input[start_pos..pos];
                                Matched(pos, { Operand::Function(f) })
                            }
                        }
                        Failed => Failed,
                    }
                }
            }
        }
    }
}
fn parse_function<'input>(input: &'input str, state: &mut ParseState<'input>,
                          pos: usize) -> RuleResult<Function> {
    {
        let start_pos = pos;
        {
            let seq_res = parse_name(input, state, pos);
            match seq_res {
                Matched(pos, f) => {
                    {
                        let seq_res = slice_eq(input, state, pos, "(");
                        match seq_res {
                            Matched(pos, _) => {
                                {
                                    let seq_res =
                                        parse_operand(input, state, pos);
                                    match seq_res {
                                        Matched(pos, p) => {
                                            {
                                                let seq_res =
                                                    slice_eq(input, state,
                                                             pos, ")");
                                                match seq_res {
                                                    Matched(pos, _) => {
                                                        {
                                                            let match_str =
                                                                &input[start_pos..pos];
                                                            Matched(pos,
                                                                    {
                                                                        Function{function:
                                                                                     f,
                                                                                 params:
                                                                                     vec!(p),}
                                                                    })
                                                        }
                                                    }
                                                    Failed => Failed,
                                                }
                                            }
                                        }
                                        Failed => Failed,
                                    }
                                }
                            }
                            Failed => Failed,
                        }
                    }
                }
                Failed => Failed,
            }
        }
    }
}
fn parse_equality<'input>(input: &'input str, state: &mut ParseState<'input>,
                          pos: usize) -> RuleResult<Equality> {
    {
        let choice_res =
            {
                let start_pos = pos;
                {
                    let seq_res = slice_eq(input, state, pos, "eq");
                    match seq_res {
                        Matched(pos, _) => {
                            {
                                let match_str = &input[start_pos..pos];
                                Matched(pos, { Equality::EQ })
                            }
                        }
                        Failed => Failed,
                    }
                }
            };
        match choice_res {
            Matched(pos, value) => Matched(pos, value),
            Failed => {
                let choice_res =
                    {
                        let start_pos = pos;
                        {
                            let seq_res = slice_eq(input, state, pos, "neq");
                            match seq_res {
                                Matched(pos, _) => {
                                    {
                                        let match_str =
                                            &input[start_pos..pos];
                                        Matched(pos, { Equality::NEQ })
                                    }
                                }
                                Failed => Failed,
                            }
                        }
                    };
                match choice_res {
                    Matched(pos, value) => Matched(pos, value),
                    Failed => {
                        let choice_res =
                            {
                                let start_pos = pos;
                                {
                                    let seq_res =
                                        slice_eq(input, state, pos, "lt");
                                    match seq_res {
                                        Matched(pos, _) => {
                                            {
                                                let seq_res =
                                                    match slice_eq(input,
                                                                   state, pos,
                                                                   "e") {
                                                        Matched(newpos, value)
                                                        => {
                                                            Matched(newpos,
                                                                    Some(value))
                                                        }
                                                        Failed => {
                                                            Matched(pos, None)
                                                        }
                                                    };
                                                match seq_res {
                                                    Matched(pos, e) => {
                                                        {
                                                            let match_str =
                                                                &input[start_pos..pos];
                                                            Matched(pos,
                                                                    {
                                                                        match e
                                                                            {
                                                                            None
                                                                            =>
                                                                            Equality::LT,
                                                                            Some(e)
                                                                            =>
                                                                            Equality::LTE,
                                                                        }
                                                                    })
                                                        }
                                                    }
                                                    Failed => Failed,
                                                }
                                            }
                                        }
                                        Failed => Failed,
                                    }
                                }
                            };
                        match choice_res {
                            Matched(pos, value) => Matched(pos, value),
                            Failed => {
                                let choice_res =
                                    {
                                        let start_pos = pos;
                                        {
                                            let seq_res =
                                                slice_eq(input, state, pos,
                                                         "gt");
                                            match seq_res {
                                                Matched(pos, _) => {
                                                    {
                                                        let seq_res =
                                                            match slice_eq(input,
                                                                           state,
                                                                           pos,
                                                                           "e")
                                                                {
                                                                Matched(newpos,
                                                                        value)
                                                                => {
                                                                    Matched(newpos,
                                                                            Some(value))
                                                                }
                                                                Failed => {
                                                                    Matched(pos,
                                                                            None)
                                                                }
                                                            };
                                                        match seq_res {
                                                            Matched(pos, e) =>
                                                            {
                                                                {
                                                                    let match_str =
                                                                        &input[start_pos..pos];
                                                                    Matched(pos,
                                                                            {
                                                                                match e
                                                                                    {
                                                                                    None
                                                                                    =>
                                                                                    Equality::GT,
                                                                                    Some(e)
                                                                                    =>
                                                                                    Equality::GTE,
                                                                                }
                                                                            })
                                                                }
                                                            }
                                                            Failed => Failed,
                                                        }
                                                    }
                                                }
                                                Failed => Failed,
                                            }
                                        }
                                    };
                                match choice_res {
                                    Matched(pos, value) =>
                                    Matched(pos, value),
                                    Failed => {
                                        let choice_res =
                                            {
                                                let start_pos = pos;
                                                {
                                                    let seq_res =
                                                        slice_eq(input, state,
                                                                 pos, "in");
                                                    match seq_res {
                                                        Matched(pos, _) => {
                                                            {
                                                                let match_str =
                                                                    &input[start_pos..pos];
                                                                Matched(pos,
                                                                        {
                                                                            Equality::IN
                                                                        })
                                                            }
                                                        }
                                                        Failed => Failed,
                                                    }
                                                }
                                            };
                                        match choice_res {
                                            Matched(pos, value) =>
                                            Matched(pos, value),
                                            Failed => {
                                                let choice_res =
                                                    {
                                                        let start_pos = pos;
                                                        {
                                                            let seq_res =
                                                                slice_eq(input,
                                                                         state,
                                                                         pos,
                                                                         "not_in");
                                                            match seq_res {
                                                                Matched(pos,
                                                                        _) =>
                                                                {
                                                                    {
                                                                        let match_str =
                                                                            &input[start_pos..pos];
                                                                        Matched(pos,
                                                                                {
                                                                                    Equality::NOT_IN
                                                                                })
                                                                    }
                                                                }
                                                                Failed =>
                                                                Failed,
                                                            }
                                                        }
                                                    };
                                                match choice_res {
                                                    Matched(pos, value) =>
                                                    Matched(pos, value),
                                                    Failed => {
                                                        let choice_res =
                                                            {
                                                                let start_pos =
                                                                    pos;
                                                                {
                                                                    let seq_res =
                                                                        slice_eq(input,
                                                                                 state,
                                                                                 pos,
                                                                                 "is");
                                                                    match seq_res
                                                                        {
                                                                        Matched(pos,
                                                                                _)
                                                                        => {
                                                                            {
                                                                                let seq_res =
                                                                                    match slice_eq(input,
                                                                                                   state,
                                                                                                   pos,
                                                                                                   "_not")
                                                                                        {
                                                                                        Matched(newpos,
                                                                                                value)
                                                                                        =>
                                                                                        {
                                                                                            Matched(newpos,
                                                                                                    Some(value))
                                                                                        }
                                                                                        Failed
                                                                                        =>
                                                                                        {
                                                                                            Matched(pos,
                                                                                                    None)
                                                                                        }
                                                                                    };
                                                                                match seq_res
                                                                                    {
                                                                                    Matched(pos,
                                                                                            _not)
                                                                                    =>
                                                                                    {
                                                                                        {
                                                                                            let match_str =
                                                                                                &input[start_pos..pos];
                                                                                            Matched(pos,
                                                                                                    {
                                                                                                        match _not
                                                                                                            {
                                                                                                            None
                                                                                                            =>
                                                                                                            Equality::IS,
                                                                                                            Some(e)
                                                                                                            =>
                                                                                                            Equality::IS_NOT,
                                                                                                        }
                                                                                                    })
                                                                                        }
                                                                                    }
                                                                                    Failed
                                                                                    =>
                                                                                    Failed,
                                                                                }
                                                                            }
                                                                        }
                                                                        Failed
                                                                        =>
                                                                        Failed,
                                                                    }
                                                                }
                                                            };
                                                        match choice_res {
                                                            Matched(pos,
                                                                    value) =>
                                                            Matched(pos,
                                                                    value),
                                                            Failed => {
                                                                let start_pos =
                                                                    pos;
                                                                {
                                                                    let seq_res =
                                                                        slice_eq(input,
                                                                                 state,
                                                                                 pos,
                                                                                 "like");
                                                                    match seq_res
                                                                        {
                                                                        Matched(pos,
                                                                                _)
                                                                        => {
                                                                            {
                                                                                let match_str =
                                                                                    &input[start_pos..pos];
                                                                                Matched(pos,
                                                                                        {
                                                                                            Equality::LIKE
                                                                                        })
                                                                            }
                                                                        }
                                                                        Failed
                                                                        =>
                                                                        Failed,
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
fn parse_condition<'input>(input: &'input str, state: &mut ParseState<'input>,
                           pos: usize) -> RuleResult<Condition> {
    {
        let choice_res =
            {
                let start_pos = pos;
                {
                    let seq_res = parse_operand(input, state, pos);
                    match seq_res {
                        Matched(pos, l) => {
                            {
                                let seq_res =
                                    slice_eq(input, state, pos, "=");
                                match seq_res {
                                    Matched(pos, _) => {
                                        {
                                            let seq_res =
                                                parse_equality(input, state,
                                                               pos);
                                            match seq_res {
                                                Matched(pos, eq) => {
                                                    {
                                                        let seq_res =
                                                            slice_eq(input,
                                                                     state,
                                                                     pos,
                                                                     ".");
                                                        match seq_res {
                                                            Matched(pos, _) =>
                                                            {
                                                                {
                                                                    let seq_res =
                                                                        parse_operand(input,
                                                                                      state,
                                                                                      pos);
                                                                    match seq_res
                                                                        {
                                                                        Matched(pos,
                                                                                r)
                                                                        => {
                                                                            {
                                                                                let match_str =
                                                                                    &input[start_pos..pos];
                                                                                Matched(pos,
                                                                                        {
                                                                                            Condition{left:
                                                                                                          l,
                                                                                                      equality:
                                                                                                          eq,
                                                                                                      right:
                                                                                                          r,}
                                                                                        })
                                                                            }
                                                                        }
                                                                        Failed
                                                                        =>
                                                                        Failed,
                                                                    }
                                                                }
                                                            }
                                                            Failed => Failed,
                                                        }
                                                    }
                                                }
                                                Failed => Failed,
                                            }
                                        }
                                    }
                                    Failed => Failed,
                                }
                            }
                        }
                        Failed => Failed,
                    }
                }
            };
        match choice_res {
            Matched(pos, value) => Matched(pos, value),
            Failed => {
                let start_pos = pos;
                {
                    let seq_res = slice_eq(input, state, pos, "(");
                    match seq_res {
                        Matched(pos, _) => {
                            {
                                let seq_res =
                                    parse_condition(input, state, pos);
                                match seq_res {
                                    Matched(pos, c) => {
                                        {
                                            let seq_res =
                                                slice_eq(input, state, pos,
                                                         ")");
                                            match seq_res {
                                                Matched(pos, _) => {
                                                    {
                                                        let match_str =
                                                            &input[start_pos..pos];
                                                        Matched(pos, { c })
                                                    }
                                                }
                                                Failed => Failed,
                                            }
                                        }
                                    }
                                    Failed => Failed,
                                }
                            }
                        }
                        Failed => Failed,
                    }
                }
            }
        }
    }
}
fn parse_direction<'input>(input: &'input str, state: &mut ParseState<'input>,
                           pos: usize) -> RuleResult<Direction> {
    {
        let choice_res =
            {
                let start_pos = pos;
                {
                    let seq_res = slice_eq(input, state, pos, "asc");
                    match seq_res {
                        Matched(pos, _) => {
                            {
                                let match_str = &input[start_pos..pos];
                                Matched(pos, { Direction::ASC })
                            }
                        }
                        Failed => Failed,
                    }
                }
            };
        match choice_res {
            Matched(pos, value) => Matched(pos, value),
            Failed => {
                let start_pos = pos;
                {
                    let seq_res = slice_eq(input, state, pos, "desc");
                    match seq_res {
                        Matched(pos, _) => {
                            {
                                let match_str = &input[start_pos..pos];
                                Matched(pos, { Direction::DESC })
                            }
                        }
                        Failed => Failed,
                    }
                }
            }
        }
    }
}
fn parse_order<'input>(input: &'input str, state: &mut ParseState<'input>,
                       pos: usize) -> RuleResult<Order> {
    {
        let start_pos = pos;
        {
            let seq_res = parse_name(input, state, pos);
            match seq_res {
                Matched(pos, c) => {
                    {
                        let seq_res = slice_eq(input, state, pos, ".");
                        match seq_res {
                            Matched(pos, _) => {
                                {
                                    let seq_res =
                                        parse_direction(input, state, pos);
                                    match seq_res {
                                        Matched(pos, d) => {
                                            {
                                                let match_str =
                                                    &input[start_pos..pos];
                                                Matched(pos,
                                                        {
                                                            Order{column: c,
                                                                  direction:
                                                                      d,}
                                                        })
                                            }
                                        }
                                        Failed => Failed,
                                    }
                                }
                            }
                            Failed => Failed,
                        }
                    }
                }
                Failed => Failed,
            }
        }
    }
}
fn parse_connector<'input>(input: &'input str, state: &mut ParseState<'input>,
                           pos: usize) -> RuleResult<Connector> {
    {
        let choice_res =
            {
                let start_pos = pos;
                {
                    let seq_res = slice_eq(input, state, pos, "&");
                    match seq_res {
                        Matched(pos, _) => {
                            {
                                let match_str = &input[start_pos..pos];
                                Matched(pos, { Connector::AND })
                            }
                        }
                        Failed => Failed,
                    }
                }
            };
        match choice_res {
            Matched(pos, value) => Matched(pos, value),
            Failed => {
                let start_pos = pos;
                {
                    let seq_res = slice_eq(input, state, pos, "|");
                    match seq_res {
                        Matched(pos, _) => {
                            {
                                let match_str = &input[start_pos..pos];
                                Matched(pos, { Connector::OR })
                            }
                        }
                        Failed => Failed,
                    }
                }
            }
        }
    }
}
fn parse_filter<'input>(input: &'input str, state: &mut ParseState<'input>,
                        pos: usize) -> RuleResult<Filter> {
    {
        let choice_res =
            {
                let start_pos = pos;
                {
                    let seq_res = parse_condition(input, state, pos);
                    match seq_res {
                        Matched(pos, lc) => {
                            {
                                let seq_res =
                                    {
                                        let mut repeat_pos = pos;
                                        let mut repeat_value = vec!();
                                        loop  {
                                            let pos = repeat_pos;
                                            let step_res =
                                                parse_connector_condition(input,
                                                                          state,
                                                                          pos);
                                            match step_res {
                                                Matched(newpos, value) => {
                                                    repeat_pos = newpos;
                                                    repeat_value.push(value);
                                                }
                                                Failed => { break ; }
                                            }
                                        }
                                        Matched(repeat_pos, repeat_value)
                                    };
                                match seq_res {
                                    Matched(pos, cc) => {
                                        {
                                            let match_str =
                                                &input[start_pos..pos];
                                            Matched(pos,
                                                    {
                                                        let mut sub_filters =
                                                            vec!();
                                                        for (conn, cond) in cc
                                                            {
                                                            let filter =
                                                                Filter{connector:
                                                                           Some(conn),
                                                                       condition:
                                                                           cond,
                                                                       subfilter:
                                                                           vec!(),};
                                                            sub_filters.push(filter);
                                                        }
                                                        Filter{connector:
                                                                   None,
                                                               condition: lc,
                                                               subfilter:
                                                                   sub_filters,}
                                                    })
                                        }
                                    }
                                    Failed => Failed,
                                }
                            }
                        }
                        Failed => Failed,
                    }
                }
            };
        match choice_res {
            Matched(pos, value) => Matched(pos, value),
            Failed => {
                let choice_res =
                    {
                        let start_pos = pos;
                        {
                            let seq_res = parse_condition(input, state, pos);
                            match seq_res {
                                Matched(pos, c) => {
                                    {
                                        let match_str =
                                            &input[start_pos..pos];
                                        Matched(pos,
                                                {
                                                    Filter{connector: None,
                                                           condition: c,
                                                           subfilter: vec!(),}
                                                })
                                    }
                                }
                                Failed => Failed,
                            }
                        }
                    };
                match choice_res {
                    Matched(pos, value) => Matched(pos, value),
                    Failed => {
                        let choice_res =
                            {
                                let start_pos = pos;
                                {
                                    let seq_res =
                                        slice_eq(input, state, pos, "(");
                                    match seq_res {
                                        Matched(pos, _) => {
                                            {
                                                let seq_res =
                                                    parse_filter(input, state,
                                                                 pos);
                                                match seq_res {
                                                    Matched(pos, f) => {
                                                        {
                                                            let seq_res =
                                                                slice_eq(input,
                                                                         state,
                                                                         pos,
                                                                         ")");
                                                            match seq_res {
                                                                Matched(pos,
                                                                        _) =>
                                                                {
                                                                    {
                                                                        let match_str =
                                                                            &input[start_pos..pos];
                                                                        Matched(pos,
                                                                                {
                                                                                    f
                                                                                })
                                                                    }
                                                                }
                                                                Failed =>
                                                                Failed,
                                                            }
                                                        }
                                                    }
                                                    Failed => Failed,
                                                }
                                            }
                                        }
                                        Failed => Failed,
                                    }
                                }
                            };
                        match choice_res {
                            Matched(pos, value) => Matched(pos, value),
                            Failed => {
                                let choice_res =
                                    {
                                        let start_pos = pos;
                                        {
                                            let seq_res =
                                                parse_condition(input, state,
                                                                pos);
                                            match seq_res {
                                                Matched(pos, lc) => {
                                                    {
                                                        let seq_res =
                                                            parse_connector(input,
                                                                            state,
                                                                            pos);
                                                        match seq_res {
                                                            Matched(pos, con)
                                                            => {
                                                                {
                                                                    let seq_res =
                                                                        parse_filter(input,
                                                                                     state,
                                                                                     pos);
                                                                    match seq_res
                                                                        {
                                                                        Matched(pos,
                                                                                rf)
                                                                        => {
                                                                            {
                                                                                let match_str =
                                                                                    &input[start_pos..pos];
                                                                                Matched(pos,
                                                                                        {
                                                                                            Filter{connector:
                                                                                                       None,
                                                                                                   condition:
                                                                                                       lc,
                                                                                                   subfilter:
                                                                                                       vec!(Filter
                                                                                                            {
                                                                                                            connector
                                                                                                            :
                                                                                                            Some
                                                                                                            (
                                                                                                            con
                                                                                                            )
                                                                                                            ,
                                                                                                            condition
                                                                                                            :
                                                                                                            rf
                                                                                                            .
                                                                                                            condition
                                                                                                            ,
                                                                                                            subfilter
                                                                                                            :
                                                                                                            vec
                                                                                                            !
                                                                                                            [

                                                                                                            ]
                                                                                                            }),}
                                                                                        })
                                                                            }
                                                                        }
                                                                        Failed
                                                                        =>
                                                                        Failed,
                                                                    }
                                                                }
                                                            }
                                                            Failed => Failed,
                                                        }
                                                    }
                                                }
                                                Failed => Failed,
                                            }
                                        }
                                    };
                                match choice_res {
                                    Matched(pos, value) =>
                                    Matched(pos, value),
                                    Failed => {
                                        let start_pos = pos;
                                        {
                                            let seq_res =
                                                parse_filter(input, state,
                                                             pos);
                                            match seq_res {
                                                Matched(pos, lf) => {
                                                    {
                                                        let seq_res =
                                                            {
                                                                let mut repeat_pos =
                                                                    pos;
                                                                let mut repeat_value =
                                                                    vec!();
                                                                loop  {
                                                                    let pos =
                                                                        repeat_pos;
                                                                    let step_res =
                                                                        parse_connector_filter(input,
                                                                                               state,
                                                                                               pos);
                                                                    match step_res
                                                                        {
                                                                        Matched(newpos,
                                                                                value)
                                                                        => {
                                                                            repeat_pos
                                                                                =
                                                                                newpos;
                                                                            repeat_value.push(value);
                                                                        }
                                                                        Failed
                                                                        => {
                                                                            break
                                                                                ;
                                                                        }
                                                                    }
                                                                }
                                                                Matched(repeat_pos,
                                                                        repeat_value)
                                                            };
                                                        match seq_res {
                                                            Matched(pos,
                                                                    conn_fil)
                                                            => {
                                                                {
                                                                    let match_str =
                                                                        &input[start_pos..pos];
                                                                    Matched(pos,
                                                                            {
                                                                                let mut sub_filters =
                                                                                    vec!();
                                                                                for (conn,
                                                                                     fil)
                                                                                    in
                                                                                    conn_fil
                                                                                    {
                                                                                    let filter =
                                                                                        Filter{connector:
                                                                                                   Some(conn),
                                                                                               condition:
                                                                                                   fil.condition,
                                                                                               subfilter:
                                                                                                   vec!(),};
                                                                                    sub_filters.push(filter);
                                                                                }
                                                                                Filter{connector:
                                                                                           None,
                                                                                       condition:
                                                                                           lf.condition,
                                                                                       subfilter:
                                                                                           sub_filters,}
                                                                            })
                                                                }
                                                            }
                                                            Failed => Failed,
                                                        }
                                                    }
                                                }
                                                Failed => Failed,
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
fn parse_connector_condition<'input>(input: &'input str,
                                     state: &mut ParseState<'input>,
                                     pos: usize)
 -> RuleResult<(Connector, Condition)> {
    {
        let start_pos = pos;
        {
            let seq_res = parse_connector(input, state, pos);
            match seq_res {
                Matched(pos, con) => {
                    {
                        let seq_res = parse_condition(input, state, pos);
                        match seq_res {
                            Matched(pos, rc) => {
                                {
                                    let match_str = &input[start_pos..pos];
                                    Matched(pos, { (con, rc) })
                                }
                            }
                            Failed => Failed,
                        }
                    }
                }
                Failed => Failed,
            }
        }
    }
}
fn parse_connector_filter<'input>(input: &'input str,
                                  state: &mut ParseState<'input>, pos: usize)
 -> RuleResult<(Connector, Filter)> {
    {
        let start_pos = pos;
        {
            let seq_res = parse_connector(input, state, pos);
            match seq_res {
                Matched(pos, con) => {
                    {
                        let seq_res = parse_filter(input, state, pos);
                        match seq_res {
                            Matched(pos, rf) => {
                                {
                                    let match_str = &input[start_pos..pos];
                                    Matched(pos, { (con, rf) })
                                }
                            }
                            Failed => Failed,
                        }
                    }
                }
                Failed => Failed,
            }
        }
    }
}
pub fn name<'input>(input: &'input str) -> ParseResult<String> {
    let mut state = ParseState::new();
    match parse_name(input, &mut state, 0) {
        Matched(pos, value) => { if pos == input.len() { return Ok(value) } }
        _ => { }
    }
    let (line, col) = pos_to_line(input, state.max_err_pos);
    Err(ParseError{line: line,
                   column: col,
                   offset: state.max_err_pos,
                   expected: state.expected,})
}
pub fn equation<'input>(input: &'input str) -> ParseResult<Equation> {
    let mut state = ParseState::new();
    match parse_equation(input, &mut state, 0) {
        Matched(pos, value) => { if pos == input.len() { return Ok(value) } }
        _ => { }
    }
    let (line, col) = pos_to_line(input, state.max_err_pos);
    Err(ParseError{line: line,
                   column: col,
                   offset: state.max_err_pos,
                   expected: state.expected,})
}
pub fn operand<'input>(input: &'input str) -> ParseResult<Operand> {
    let mut state = ParseState::new();
    match parse_operand(input, &mut state, 0) {
        Matched(pos, value) => { if pos == input.len() { return Ok(value) } }
        _ => { }
    }
    let (line, col) = pos_to_line(input, state.max_err_pos);
    Err(ParseError{line: line,
                   column: col,
                   offset: state.max_err_pos,
                   expected: state.expected,})
}
pub fn function<'input>(input: &'input str) -> ParseResult<Function> {
    let mut state = ParseState::new();
    match parse_function(input, &mut state, 0) {
        Matched(pos, value) => { if pos == input.len() { return Ok(value) } }
        _ => { }
    }
    let (line, col) = pos_to_line(input, state.max_err_pos);
    Err(ParseError{line: line,
                   column: col,
                   offset: state.max_err_pos,
                   expected: state.expected,})
}
pub fn equality<'input>(input: &'input str) -> ParseResult<Equality> {
    let mut state = ParseState::new();
    match parse_equality(input, &mut state, 0) {
        Matched(pos, value) => { if pos == input.len() { return Ok(value) } }
        _ => { }
    }
    let (line, col) = pos_to_line(input, state.max_err_pos);
    Err(ParseError{line: line,
                   column: col,
                   offset: state.max_err_pos,
                   expected: state.expected,})
}
pub fn condition<'input>(input: &'input str) -> ParseResult<Condition> {
    let mut state = ParseState::new();
    match parse_condition(input, &mut state, 0) {
        Matched(pos, value) => { if pos == input.len() { return Ok(value) } }
        _ => { }
    }
    let (line, col) = pos_to_line(input, state.max_err_pos);
    Err(ParseError{line: line,
                   column: col,
                   offset: state.max_err_pos,
                   expected: state.expected,})
}
pub fn direction<'input>(input: &'input str) -> ParseResult<Direction> {
    let mut state = ParseState::new();
    match parse_direction(input, &mut state, 0) {
        Matched(pos, value) => { if pos == input.len() { return Ok(value) } }
        _ => { }
    }
    let (line, col) = pos_to_line(input, state.max_err_pos);
    Err(ParseError{line: line,
                   column: col,
                   offset: state.max_err_pos,
                   expected: state.expected,})
}
pub fn order<'input>(input: &'input str) -> ParseResult<Order> {
    let mut state = ParseState::new();
    match parse_order(input, &mut state, 0) {
        Matched(pos, value) => { if pos == input.len() { return Ok(value) } }
        _ => { }
    }
    let (line, col) = pos_to_line(input, state.max_err_pos);
    Err(ParseError{line: line,
                   column: col,
                   offset: state.max_err_pos,
                   expected: state.expected,})
}
pub fn connector<'input>(input: &'input str) -> ParseResult<Connector> {
    let mut state = ParseState::new();
    match parse_connector(input, &mut state, 0) {
        Matched(pos, value) => { if pos == input.len() { return Ok(value) } }
        _ => { }
    }
    let (line, col) = pos_to_line(input, state.max_err_pos);
    Err(ParseError{line: line,
                   column: col,
                   offset: state.max_err_pos,
                   expected: state.expected,})
}
pub fn filter<'input>(input: &'input str) -> ParseResult<Filter> {
    let mut state = ParseState::new();
    match parse_filter(input, &mut state, 0) {
        Matched(pos, value) => { if pos == input.len() { return Ok(value) } }
        _ => { }
    }
    let (line, col) = pos_to_line(input, state.max_err_pos);
    Err(ParseError{line: line,
                   column: col,
                   offset: state.max_err_pos,
                   expected: state.expected,})
}
pub fn connector_condition<'input>(input: &'input str)
 -> ParseResult<(Connector, Condition)> {
    let mut state = ParseState::new();
    match parse_connector_condition(input, &mut state, 0) {
        Matched(pos, value) => { if pos == input.len() { return Ok(value) } }
        _ => { }
    }
    let (line, col) = pos_to_line(input, state.max_err_pos);
    Err(ParseError{line: line,
                   column: col,
                   offset: state.max_err_pos,
                   expected: state.expected,})
}
pub fn connector_filter<'input>(input: &'input str)
 -> ParseResult<(Connector, Filter)> {
    let mut state = ParseState::new();
    match parse_connector_filter(input, &mut state, 0) {
        Matched(pos, value) => { if pos == input.len() { return Ok(value) } }
        _ => { }
    }
    let (line, col) = pos_to_line(input, state.max_err_pos);
    Err(ParseError{line: line,
                   column: col,
                   offset: state.max_err_pos,
                   expected: state.expected,})
}
