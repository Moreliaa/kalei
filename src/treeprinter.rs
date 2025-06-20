use std::collections::HashMap;
pub struct TreePrinter {
    print_items: HashMap<i32, Vec<PrintItem>>,
    min_ident_level: i32,
}

struct PrintItem {
    indent_level: i32,
    item: String,
}

impl TreePrinter {
    pub fn new() -> TreePrinter {
        TreePrinter {
            print_items: HashMap::new(),
            min_ident_level: 0,
        }
    }
    pub fn add_print_item(&mut self, item: String, depth: i32, indent_level: i32) {
        self.print_items.entry(depth).or_insert_with(|| vec![]);
        self.min_ident_level = std::cmp::min(indent_level, self.min_ident_level);
        let lvl = self.print_items.get_mut(&depth).unwrap();
        lvl.push(PrintItem { indent_level, item });
    }
    pub fn print_tree(&self) {
        let mut depth = 0;
        let max_depth = self.print_items.len();
        while let Some(line) = self.print_items.get(&depth) {
            let padding = "     ";
            depth += 1;
            let mut indent_level = self.min_ident_level;
            let mut items = vec![];
            for print_item in line {
                while indent_level < print_item.indent_level {
                    indent_level += 1;
                    items.push(String::from(padding));
                }
                items.push(format!("{:^5}", print_item.item));
                indent_level += 1;
            }
            println!("{}", items.join(""));
        }
    }
}
