pub mod stone_counter {
   
    type Id = usize;
    type Count = usize;

    #[derive(Debug)]
    pub struct StoneTable {
        table: Vec<(Id, Count)>,
        rule_table: Vec<(Id, Id)>
    }

    impl StoneTable {
        pub fn new(data: Vec<String>) -> Self {
            let table: Vec<(Id, Count)> = data.iter().map(|x| (x.parse::<usize>().unwrap(), 1)).collect();
            StoneTable {
                table,
                rule_table: Vec::new(),
            }
        }

        pub fn move_stones(&mut self, identifier: Id) {
            let applied_rules = self.find_rules(identifier);
            if applied_rules.len() == 0 {
               self.add_rules(identifier);
               self.move_stones(identifier);
            } else {
                let index = self.table.iter().position(|(x, _)| x == &identifier).unwrap();
                let stones = self.table[index];


                for rule in applied_rules.iter() {
                  match self.table.iter().position(|(x, _)| x == &rule.1) {
                      Some(x) => ,
                      None =>
                  }
                  self.table[position] = self.table[position]
            }
            }

        }

        pub fn find_rules(&self, identifier: Id) -> Vec<(Id, Id)> {
            self.rule_table.iter().filter(|(x, _)| x == &identifier).map(|t| t.clone()).collect()
        }

        pub fn add_rules(&mut self, identifier: Id) {
            match identifier {
                x if x.to_string().len() % 2 == 0 => self.helper_function(x)
                ,
                x => self.rule_table.push((x, x * 2024)),
                    }
        }

        fn helper_function(&mut self, identifier: Id) {
                    let value = identifier.to_string();
                    let (left, right) = value.split_at(value.len() / 2);
                    self.rule_table.push((identifier, left.parse::<usize>().unwrap()));
                    self.rule_table.push((identifier, right.parse::<usize>().unwrap()));
        }
    }

}
