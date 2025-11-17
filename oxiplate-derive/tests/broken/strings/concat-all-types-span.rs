use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline(r##"{{ "foo".repeat(
    foo
    ~ 'a' ~ '\n' ~ '\r' ~ '\t' ~ '\0' ~ '\\' ~ '\'' ~ '\"'
    ~ "hello world" ~ #"jane "the deer" doe"#
    ~ true ~ false
    ~ 0b_1_0011_
    ~ 0x_1_3_
    ~ 0o_2_3_
    ~ 1_9_
    ~ 1_9_e_0_ ~ 19e-1 ~ 19E+1
    ~ 1_.9_
) }}"##)]
struct Data {}

fn main() {
    let data = Data {};

    print!("{}", data);
}
