use proc_macro::TokenStream;

trait AppentLine {
    fn append_line(&mut self, s: &str);
}

impl AppentLine for String {
    fn append_line(&mut self, s: &str) {
        self.push_str(s);
        self.push_str("\n");
    }
}

#[proc_macro]
pub fn define_monadde_macro(_: TokenStream) -> TokenStream {
    const DEPTH:i32 = 10; // make it bigger in the case you need more depth (thx Cap)
    let mut resultString = String::from("macro_rules! monadde {\n");
    resultString.append_line("($e_in:expr => $i:ident |> $e_out:expr)");
    resultString.append_line("=> { $e_in.fmap(|$i| $e_out) };");
    resultString.append_line("($e_in0:expr => $i0:ident |> $e_in:expr => $i:ident |> $e_out:expr)");
    resultString.append_line("=> { $e_in0.bind(|$i0| monadde!{ $e_in => $i |> $e_out }) };");
    for i in 1..DEPTH {
        resultString.append_line("(");
        resultString.append_line("$e_in0:expr => $i0:ident |> ");
        for j in 1..=i {
            let formatted = format!("$e_in{}:expr => $i{}:ident |> ", j, j);
            resultString.append_line(&formatted);
        }
        resultString.append_line("$e_in:expr => $i:ident |> $e_out:expr");
        resultString.append_line(") => {");
        resultString.append_line("$e_in0.bind(|$i0| monadde!{");
        for j in 1..=i {
            let formatted = format!("$e_in{} => $i{} |> ", j, j);
            resultString.append_line(&formatted);
        }
        resultString.append_line("$e_in => $i |> $e_out");
        resultString.append_line("})");
        resultString.append_line("};")
    }
    resultString.append_line("}");
    (&resultString).parse::<TokenStream>().unwrap()
}
