use crate::day05::Instruction;
use crate::day05::Stack;

pub struct Depot {
    stacks: Vec<Stack>,
}

impl Depot {
    pub fn from(input_str: &str) -> Self {
        let mut depot = Depot { stacks: Vec::new() };
        depot.fill(input_str.trim_end());
        depot
    }

    fn fill(&mut self, input_str: &str) {
        let mut lines: Vec<&str> = input_str.lines().collect();
        lines.reverse();
        for line in lines {
            for (index, char) in line.chars().enumerate() {
                if Depot::is_char_a_letter(char) {
                    let stack_index = Depot::get_stack_index(index);
                    self.add_to_stack(stack_index, char);
                }
            }
        }
    }

    fn is_char_a_letter(char: char) -> bool {
        char != ' ' && char != '[' && char != ']'
    }

    fn get_stack_index(index_in_string: usize) -> usize {
        (index_in_string - 1) / 4
    }

    fn add_to_stack(&mut self, stack_index: usize, item: char) {
        if self.has_not_enough_space(stack_index) {
            self.ensure_enough_stacks(stack_index);
        }

        let stack = self
            .stacks
            .get_mut(stack_index)
            .expect("allocated more space but could still not find Stack");
        stack.add_new(item);
    }

    fn has_not_enough_space(&self, stack_index: usize) -> bool {
        self.stacks.get(stack_index).is_none()
    }

    fn ensure_enough_stacks(&mut self, stack_index: usize) {
        while self.stacks.len() < stack_index + 1 {
            self.stacks.push(Stack::new());
        }
    }

    pub fn execute_all(&mut self, instructions: &Vec<Instruction>) {
        for instruction in instructions {
            self.execute(instruction, false);
        }
    }

    pub fn execute_all_with_all_crates_lifted_together(&mut self, instructions: &Vec<Instruction>) {
        for instruction in instructions {
            self.execute(instruction, true);
        }
    }

    fn execute(&mut self, instruction: &Instruction, lift_all_crates_together: bool) {
        let Some(source_stack) = self.stacks.get_mut(instruction.source_stack) else {
            panic!("could not find the source stack!");
        };

        let new_crates = source_stack.take(instruction.move_count, lift_all_crates_together);

        let Some(target_stack) = self.stacks.get_mut(instruction.target_stack) else {
            panic!("could not find the target stack!");
        };
        target_stack.add(new_crates);
    }

    pub fn get_most_upper_crates_as_message(&self) -> String {
        let mut message: String = String::new();
        for stack in self.stacks.as_slice() {
            message.push(stack.last());
        }
        message
    }
}
