use std::fs;

fn main() {
    let kwlist: Vec<String> = fs::read_to_string("kw.txt")
        .unwrap()
        .lines()
        .map(|l| l.trim().to_string())
        .filter(|l| !l.is_empty())
        .collect();
    let mut out: String = format!("pub static KW_TAB:[&str; {}] = [\n", kwlist.len());
    for kw in &kwlist {
        out.push_str(&format!("    \"{}\",\n", kw));
    }
    out.push_str("];\n\n#[derive(PartialEq,Clone,Copy,Debug)]\npub enum Kw {\n");
    for kw in &kwlist {
        out.push_str(&format!("    {},\n", casecnv(kw)))
    }
    out.push_str(
        "}

impl Kw {
    pub fn findkw(s:&str) -> Option<Self> {
        match s {\n",
    );
    for kw in &kwlist {
        out.push_str(&format!(
            "\t\t\t\"{}\" => Some(Self::{}),\n",
            kw,
            casecnv(kw)
        ))
    }
    out.push_str(
        "
            _ => None
        }
    }
}",
    );
    fs::write("src/compile/tkn/kw.rs", out).unwrap();

    println!("cargo::rerun-if-changed=kw.txt")
}

fn casecnv(kw: &str) -> String {
    let mut upper: bool = true;
    let mut s: String = String::new();
    for c in kw.chars() {
        if c == '-' {
            upper = true;
        } else if upper {
            upper = false;
            s.push_str(&c.to_uppercase().to_string());
        } else {
            s.push_str(&c.to_lowercase().to_string());
        }
    }
    s
}
