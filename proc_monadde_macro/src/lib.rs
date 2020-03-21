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
    let mut result_string = String::from("macro_rules! monadde {\n");
    result_string.append_line("($e_in:expr => $i:ident |> $e_out:expr)");
    result_string.append_line("=> { $e_in.map(move |$i| $e_out) };");
    result_string.append_line("($e_in0:expr => $i0:ident |> $e_in:expr => $i:ident |> $e_out:expr)");
    result_string.append_line("=> { $e_in0.flat_map(move |$i0| monadde!{ $e_in => $i |> $e_out }) };");
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
    result_string.append_line("}");
    (&result_string).parse::<TokenStream>().unwrap()
}
