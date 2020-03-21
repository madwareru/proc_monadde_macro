use proc_macro::TokenStream;

trait AppendLine {
    fn append_line(&mut self, s: &str);
}

impl AppendLine for String {
    fn append_line(&mut self, s: &str) {
        self.push_str(s);
        self.push_str("\n");
    }
}

#[proc_macro]
pub fn define_monadde_macro(_: TokenStream) -> TokenStream {
    const DEPTH:i32 = 10; // make it bigger in the case you need more depth (thx Cap)
    let mut result_string = String::from("#[macro_export]\nmacro_rules! monadde {\n");

    //map step (simplest variant possible)
    result_string.append_line("($e_in:expr => $i:ident |> $e_out:expr)");
    result_string.append_line("=> { $e_in.map(move |$i| $e_out) };");

    //"filter_map" variant which takes care of filtering feature
    result_string.append_line("($e_in:expr => $i:ident |> when $cond:expr => $e_out:expr)");
    result_string.append_line("=> { $e_in.filter_map(move |$i| if $cond { Some($e_out) } else { None } ) };");

    //root bind step
    result_string.append_line("($e_in0:expr => $i0:ident |> $e_in:expr => $i:ident |> $e_out:expr)");
    result_string.append_line("=> { $e_in0.flat_map(move |$i0| monadde!{ $e_in => $i |> $e_out }) };");

    //root filtermappy bind
    result_string.append_line("($e_in0:expr => $i0:ident |> $e_in:expr => $i:ident |> when $cond:expr => $e_out:expr)");
    result_string.append_line("=> { $e_in0.flat_map(move |$i0| monadde!{ $e_in => $i |> when $cond => $e_out }) };");

    for i in 1..DEPTH {
        result_string.append_line("(");
        result_string.append_line("$e_in0:expr => $i0:ident |> ");
        for j in 1..=i {
            let formatted = format!("$e_in{}:expr => $i{}:ident |> ", j, j);
            result_string.append_line(&formatted);
        }
        result_string.append_line("$e_in:expr => $i:ident |> $e_out:expr");
        result_string.append_line(") => {");
        result_string.append_line("$e_in0.flat_map(move |$i0| monadde!{");
        for j in 1..=i {
            let formatted = format!("$e_in{} => $i{} |> ", j, j);
            result_string.append_line(&formatted);
        }
        result_string.append_line("$e_in => $i |> $e_out");
        result_string.append_line("})");
        result_string.append_line("};")
    }

    for i in 1..DEPTH {
        result_string.append_line("(");
        result_string.append_line("$e_in0:expr => $i0:ident |> ");
        for j in 1..=i {
            let formatted = format!("$e_in{}:expr => $i{}:ident |> ", j, j);
            result_string.append_line(&formatted);
        }
        result_string.append_line("$e_in:expr => $i:ident |> when $cond:expr => $e_out:expr");
        result_string.append_line(") => {");
        result_string.append_line("$e_in0.flat_map(move |$i0| monadde!{");
        for j in 1..=i {
            let formatted = format!("$e_in{} => $i{} |> ", j, j);
            result_string.append_line(&formatted);
        }
        result_string.append_line("$e_in => $i |> when $cond => $e_out");
        result_string.append_line("})");
        result_string.append_line("};")
    }
    result_string.append_line("}");
    (&result_string).parse::<TokenStream>().unwrap()
}
