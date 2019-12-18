mod custom_paintfuck_interpreter {

    fn interpreter(code: &str, mut iterations: usize, width: usize, height: usize) -> String {
        let chars: Vec<char> = code
            .chars()
            .filter(|c| match *c {
                '*' | 'w' | 'e' | 'n' | 's' | '[' | ']' => true,
                _ => false,
            })
            .collect();
        let mut data = vec![vec!['0'; width]; height];
        let mut x = 0;
        let mut y = 0;
        let mut current_index = 0;
        while iterations > 0 && current_index < chars.len() {
            let v = data[y][x];
            match chars[current_index] {
                'e' => {
                    x += 1;
                    if x == width {
                        x = 0;
                    }
                    current_index += 1;
                }
                'w' => {
                    if x == 0 {
                        x = width;
                    }
                    x -= 1;
                    current_index += 1;
                }
                'n' => {
                    if y == 0 {
                        y = height;
                    }
                    y -= 1;
                    current_index += 1;
                }
                's' => {
                    y += 1;
                    if y == height {
                        y = 0;
                    }
                    current_index += 1;
                }
                '*' => {
                    data[y][x] = if v == '0' { '1' } else { '0' };
                    current_index += 1;
                }
                '[' => {
                    current_index += 1;
                    if v == '0' {
                        let mut open_count = 0;
                        for i in current_index..chars.len() {
                            match chars[i] {
                                '[' => open_count += 1,
                                ']' => {
                                    if open_count == 0 {
                                        current_index = i + 1;
                                        break;
                                    } else {
                                        open_count -= 1;
                                    }
                                }
                                _ => {}
                            };
                        }
                    }
                }
                ']' => {
                    if v == '1' {
                        let mut close_count = 0;
                        for i in (0..current_index).rev() {
                            match chars[i] {
                                ']' => close_count += 1,
                                '[' => {
                                    if close_count == 0 {
                                        // Should jump to the command straight *after*
                                        // the matching "[" on the iteration where it
                                        // hits the "]" and *not* the matching "[" itself
                                        current_index = i + 1;
                                        break;
                                    } else {
                                        close_count -= 1;
                                    }
                                }
                                _ => {}
                            };
                        }
                    } else {
                        current_index += 1;
                    }
                }
                _ => {
                    current_index += 1;
                }
            };
            iterations -= 1;
        }
        data.into_iter()
            .map(|line| line.into_iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\r\n")
    }

    // #[test]
    fn simple_cases() {
        assert_eq!(&interpreter("*e*e*e*es*es*ws*ws*w*w*w*n*n*n*ssss*s*s*s*", 0, 6, 9), "000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000", "Your interpreter should initialize all cells in the datagrid to 0");
        assert_eq!(&interpreter("*e*e*e*es*es*ws*ws*w*w*w*n*n*n*ssss*s*s*s*", 7, 6, 9), "111100\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000", "Your interpreter should adhere to the number of iterations specified");
        assert_eq!(&interpreter("*e*e*e*es*es*ws*ws*w*w*w*n*n*n*ssss*s*s*s*", 19, 6, 9), "111100\r\n000010\r\n000001\r\n000010\r\n000100\r\n000000\r\n000000\r\n000000\r\n000000", "Your interpreter should traverse the 2D datagrid correctly");
        assert_eq!(&interpreter("*e*e*e*es*es*ws*ws*w*w*w*n*n*n*ssss*s*s*s*", 42, 6, 9), "111100\r\n100010\r\n100001\r\n100010\r\n111100\r\n100000\r\n100000\r\n100000\r\n100000", "Your interpreter should traverse the 2D datagrid correctly for all of the \"n\", \"e\", \"s\" and \"w\" commands");
        assert_eq!(&interpreter("*e*e*e*es*es*ws*ws*w*w*w*n*n*n*ssss*s*s*s*", 100, 6, 9),"111100\r\n100010\r\n100001\r\n100010\r\n111100\r\n100000\r\n100000\r\n100000\r\n100000", "Your interpreter should terminate normally and return a representation of the final state of the 2D datagrid when all commands have been considered from left to right even if the number of iterations specified have not been fully performed");
    }
}
