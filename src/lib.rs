extern crate rand;

use rand::Rng;

#[derive(Clone, Debug)]
pub enum Operator {
    Acos,
    Acosh,
    And,
    Asin,
    Asinh,
    Atan,
    Atan2,
    Atanh,
    Cbrt,
    Ceil,
    Chs,
    Clear,
    Cos,
    Cosh,
    Deltapercent,
    Depth,
    Divide,
    Drop,
    Drop2,
    Dropn,
    Dupel,
    Dupel2,
    Dupn,
    Erf,
    Erfc,
    Exp,
    Exp10,
    Expm1,
    Fabs,
    Fact,
    Floor,
    Fp,
    Gcd,
    Help,
    Hypot,
    Idiv,
    Inv,
    Ip,
    J0,
    J1,
    Jn,
    Lgamma,
    Log,
    Log10,
    Log1p,
    Log2,
    Logb,
    Max,
    Min,
    Minus,
    Modulo,
    Multiply,
    Not,
    Or,
    Over,
    Percent,
    Percenttotal,
    Pick,
    Plus,
    Pop,
    Pow,
    Prec,
    Prod,
    Push,
    Quit,
    Rdz,
    Rint,
    Rnd,
    Roll,
    Rolld,
    Rot,
    Setbin,
    Setchar,
    Setdec,
    Sethex,
    Setoct,
    Showstack,
    Sign,
    Sin,
    Sinh,
    Sl,
    Slb,
    Sqr,
    Sqrt,
    Sr,
    Srb,
    Sum,
    Swap,
    Tan,
    Tanh,
    Warranty,
    Xor,
    Y0,
    Y1,
    Yn,
}

#[derive(Clone, Debug)]
pub enum OperationElt {
    Operator(Operator),
    Operand(f32),
    //Text(String),
}

fn tokenizer(expr: &str) -> Result<Vec<OperationElt>, String> {
    expr.split_whitespace()
        .map(|el| match el {
            "!" => Ok(OperationElt::Operator(Operator::Fact)), // "factorial"                               },
            "%" => Ok(OperationElt::Operator(Operator::Percent)), // "xy/100"                                  },
            "%ch" => Ok(OperationElt::Operator(Operator::Deltapercent)), // "100(y-x)/x"                           },
            "%t" => Ok(OperationElt::Operator(Operator::Percenttotal)), // "100y/x"                               },
            "&" => Ok(OperationElt::Operator(Operator::And)), // "logical and"                             },
            "*" => Ok(OperationElt::Operator(Operator::Multiply)), // "multiplication"                          },
            "+" => Ok(OperationElt::Operator(Operator::Plus)), // "addition"                                },
            "-" => Ok(OperationElt::Operator(Operator::Minus)), // "subtraction"                             },
            "/" => Ok(OperationElt::Operator(Operator::Divide)), // "division"                                },
            "?" => Ok(OperationElt::Operator(Operator::Help)), // "show all known functions and commands"   },
            "^" => Ok(OperationElt::Operator(Operator::Pow)), // "exponentiation"                          },
            "abs" => Ok(OperationElt::Operator(Operator::Fabs)), // "absolute value: |x|"                     },
            "acos" => Ok(OperationElt::Operator(Operator::Acos)), // "arc cosine"                              },
            "acosh" => Ok(OperationElt::Operator(Operator::Acosh)), // "inverse hyperbolic cosine"               },
            "alog" => Ok(OperationElt::Operator(Operator::Exp10)), // "exponentiation to the base 10: 10^x"     },
            "and" => Ok(OperationElt::Operator(Operator::And)), // "logical and"                             },
            "arg" => Ok(OperationElt::Operator(Operator::Atan2)), // "arc tangent of 2 variables"              },
            "asin" => Ok(OperationElt::Operator(Operator::Asin)), // "arc sine"                                },
            "asinh" => Ok(OperationElt::Operator(Operator::Asinh)), // "inverse hyperbolic sine"                 },
            "atan" => Ok(OperationElt::Operator(Operator::Atan)), // "arc tangent"                             },
            "atan2" => Ok(OperationElt::Operator(Operator::Atan2)), // "arc tangent of 2 variables"              },
            "atanh" => Ok(OperationElt::Operator(Operator::Atanh)), // "inverse hyperbolic tangent"              },
            "bin" => Ok(OperationElt::Operator(Operator::Setbin)), // "set binary"                              },
            "cbrt" => Ok(OperationElt::Operator(Operator::Cbrt)), // "cube root: x^1/3"                        },
            "ceil" => Ok(OperationElt::Operator(Operator::Ceil)), // "smallest integral value not less than x" },
            "char" => Ok(OperationElt::Operator(Operator::Setchar)), // "display ASCII values"                    },
            "chs" => Ok(OperationElt::Operator(Operator::Chs)), // "change sign"                             },
            "clear" => Ok(OperationElt::Operator(Operator::Clear)), // "clear stack"                             },
            "cos" => Ok(OperationElt::Operator(Operator::Cos)), // "cosine"                                  },
            "cosh" => Ok(OperationElt::Operator(Operator::Cosh)), // "arcus cosine"                            },
            "dec" => Ok(OperationElt::Operator(Operator::Setdec)), // "switch to decimal base"                  },
            "depth" => Ok(OperationElt::Operator(Operator::Depth)), // "show depth of stack"                     },
            "div" => Ok(OperationElt::Operator(Operator::Idiv)), // "divide"                                  },
            "drop" => Ok(OperationElt::Operator(Operator::Drop)), // "drop last stack element"                 },
            "drop2" => Ok(OperationElt::Operator(Operator::Drop2)), // "drop last 2 stack elements"              },
            "dropn" => Ok(OperationElt::Operator(Operator::Dropn)), // "drop last n stack elements"              },
            "dup" => Ok(OperationElt::Operator(Operator::Dupel)), // "duplicate last stack element"            },
            "dup2" => Ok(OperationElt::Operator(Operator::Dupel2)), // "duplicate last 2 stack elements"         },
            "dupn" => Ok(OperationElt::Operator(Operator::Dupn)), // "duplicate last n stack elements"         },
            "erf" => Ok(OperationElt::Operator(Operator::Erf)), // "error function"                          },
            "erfc" => Ok(OperationElt::Operator(Operator::Erfc)), // "complementary error function: 1-erf(x)"  },
            "exit" => Ok(OperationElt::Operator(Operator::Quit)), // "exit rpncalc"                            },
            "exp" => Ok(OperationElt::Operator(Operator::Exp)), // "exponential function: e^x"               },
            "expm" => Ok(OperationElt::Operator(Operator::Expm1)), // "e^x-1"                                   },
            "fact" => Ok(OperationElt::Operator(Operator::Fact)), // "factorial"                               },
            "floor" => Ok(OperationElt::Operator(Operator::Floor)), // "largest integral value not greater than x" },
            "fp" => Ok(OperationElt::Operator(Operator::Fp)), // "floating point part of x"                },
            "gcd" => Ok(OperationElt::Operator(Operator::Gcd)), // "greatest common divisor"                 },
            "help" => Ok(OperationElt::Operator(Operator::Help)), // "show all known functions and commands"   },
            "hex" => Ok(OperationElt::Operator(Operator::Sethex)), // "switch to hexadecimal base"              },
            "hypot" => Ok(OperationElt::Operator(Operator::Hypot)), // "Euclidean distance function: sqrt(x^2+y^2)" },
            "inv" => Ok(OperationElt::Operator(Operator::Inv)), // "inverse: 1/x"                            },
            "ip" => Ok(OperationElt::Operator(Operator::Ip)), // "largest integral value not greater than x" },
            "j0" => Ok(OperationElt::Operator(Operator::J0)), // "Bessel function of zeroth order"         },
            "j1" => Ok(OperationElt::Operator(Operator::J1)), // "Bessel function of first  order"         },
            "jn" => Ok(OperationElt::Operator(Operator::Jn)), // "Bessel function of n-th order"           },
            "ld" => Ok(OperationElt::Operator(Operator::Log2)), // "base-2 logarithm"                        },
            "ldb" => Ok(OperationElt::Operator(Operator::Logb)), // "integer part of the base-2 logarithm"    },
            "lg" => Ok(OperationElt::Operator(Operator::Log10)), // "logarithm to the base 10"                },
            "lgamma" => Ok(OperationElt::Operator(Operator::Lgamma)), // "ln|gamma(x)|"                            },
            "ln" => Ok(OperationElt::Operator(Operator::Log)), // "natural log"                             },
            "lnp1" => Ok(OperationElt::Operator(Operator::Log1p)), // "ln(1+x)"                                 },
            "log" => Ok(OperationElt::Operator(Operator::Log10)), // "logarithm to the base 10"                },
            "max" => Ok(OperationElt::Operator(Operator::Max)), // "max(x,y)"                                },
            "min" => Ok(OperationElt::Operator(Operator::Min)), // "min(x,y)"                                },
            "mod" => Ok(OperationElt::Operator(Operator::Modulo)), // "modulo function"                         },
            "neg" => Ok(OperationElt::Operator(Operator::Chs)), // "change sign"                             },
            "not" => Ok(OperationElt::Operator(Operator::Not)), // "logical not"                             },
            "oct" => Ok(OperationElt::Operator(Operator::Setoct)), // "switch to octal base"                    },
            "or" => Ok(OperationElt::Operator(Operator::Or)), // "logical or"                              },
            "over" => Ok(OperationElt::Operator(Operator::Over)), // "push second last element on stack"       },
            "pick" => Ok(OperationElt::Operator(Operator::Pick)), // "duplicate a element from the stack"      },
            "pop" => Ok(OperationElt::Operator(Operator::Pop)), // "drop last element from stack"            },
            "prec" => Ok(OperationElt::Operator(Operator::Prec)), // "set precision"                           },
            "prod" => Ok(OperationElt::Operator(Operator::Prod)), // "product of all elements on stack"        },
            "push" => Ok(OperationElt::Operator(Operator::Push)), // "push a new element on the stack"         },
            "quit" => Ok(OperationElt::Operator(Operator::Quit)), // "exit rpncalc"                            },
            "rand" => Ok(OperationElt::Operator(Operator::Rnd)), // "random value [0, 1["                     },
            "rdz" => Ok(OperationElt::Operator(Operator::Rdz)), // "set seed for random value generator"     },
            "rint" => Ok(OperationElt::Operator(Operator::Rint)), // "round to closest integer"                },
            "rnd" => Ok(OperationElt::Operator(Operator::Rint)), // "round to closest integer"                },
            "roll" => Ok(OperationElt::Operator(Operator::Roll)), // "roll the stack"                          },
            "rolld" => Ok(OperationElt::Operator(Operator::Rolld)), // "roll the stack down"                     },
            "rot" => Ok(OperationElt::Operator(Operator::Rot)), // "rotate the stack"                        },
            "shl" => Ok(OperationElt::Operator(Operator::Sl)), // "shift left"                              },
            "show" => Ok(OperationElt::Operator(Operator::Showstack)), // "redisplay stack"                         },
            "shr" => Ok(OperationElt::Operator(Operator::Sr)), // "shift right"                             },
            "sign" => Ok(OperationElt::Operator(Operator::Sign)), // "sign of x"                               },
            "sin" => Ok(OperationElt::Operator(Operator::Sin)), // "sine"                                    },
            "sinh" => Ok(OperationElt::Operator(Operator::Sinh)), // "hyperbolic sine"                         },
            "sl" => Ok(OperationElt::Operator(Operator::Sl)), // "shift left"                              },
            "slb" => Ok(OperationElt::Operator(Operator::Slb)), // "shift left 1 byte"                       },
            "sq" => Ok(OperationElt::Operator(Operator::Sqr)), // "square: x^2"                             },
            "sqr" => Ok(OperationElt::Operator(Operator::Sqr)), // "square: x^2"                             },
            "sqrt" => Ok(OperationElt::Operator(Operator::Sqrt)), // "square root"                             },
            "sr" => Ok(OperationElt::Operator(Operator::Sr)), // "shift right"                             },
            "srb" => Ok(OperationElt::Operator(Operator::Srb)), // "shift right 1 byte"                      },
            "sum" => Ok(OperationElt::Operator(Operator::Sum)), // "sum of all elements on stack"            },
            "swap" => Ok(OperationElt::Operator(Operator::Swap)), // "swap last two elements"                  },
            "tan" => Ok(OperationElt::Operator(Operator::Tan)), // "tangent"                                 },
            "tanh" => Ok(OperationElt::Operator(Operator::Tanh)), // "hyperbolic tangent"                      },
            "warranty" => Ok(OperationElt::Operator(Operator::Warranty)), // "show absence of warranty"                },
            "xor" => Ok(OperationElt::Operator(Operator::Xor)), // "logical xor"                             },
            "y0" => Ok(OperationElt::Operator(Operator::Y0)), // "Bessel function of the second kind of zeroth order" },
            "y1" => Ok(OperationElt::Operator(Operator::Y1)), // "Bessel function of the second kind of first order"  },
            "yn" => Ok(OperationElt::Operator(Operator::Yn)), // "Bessel function of the second kind of n-th order"   },
            "|" => Ok(OperationElt::Operator(Operator::Or)), // "logical or"                              },
            "~" => Ok(OperationElt::Operator(Operator::Not)), // "logical not"                             }
            operand => match operand.parse::<f32>() {
                Ok(val) => Ok(OperationElt::Operand(val)),
                Err(_) => Err(format!("Cannot parse operand \"{}\"", operand)),
            },
        })
        .into_iter()
        .collect()
}

/// Evaluates an RPL expression.
///
/// # Examples
/// ````
/// extern crate rpn;
/// use rpn::OperationElt;
///
/// let mut stack: Vec<OperationElt> = Vec::new();
/// let result = rpn::evaluate(&mut stack, "5 2 +").unwrap();
/// assert_eq!(
///     match result.pop().unwrap() {
///         OperationElt::Operand(val) => Ok(val),
///         _ => Err("No operand on stack"),
///     }.unwrap(),
///     7.0
/// );
/// ````
///
/// # Errors
/// This function will return an error in case of bad expression:
///
/// - if it includes an unrecognized operator (recognized ones are +, -, * and /
/// - if it misses an operand (i.e. value)
/// - if it misses an operator
pub fn evaluate<'a>(
    stack: &'a mut Vec<OperationElt>,
    expr: &str,
) -> Result<&'a mut Vec<OperationElt>, String> {
    return match tokenizer(expr) {
        Ok(tokens) => {
            // Before a new stack was created, now just use the existing stack
            //let mut stack: Vec<f32> = Vec::new();
            for token in tokens {
                match token {
                    OperationElt::Operator(operator) => {
                        // Parse the operation to use
                        if stack.len() >= 2 {
                            // Operators with 2 operands
                            let operand1 = match stack.pop().unwrap() {
                                OperationElt::Operand(val) => Ok(val),
                                _ => Err("Not a suitable number on the stack"),
                            }.unwrap();
                            let operand2 = match stack.pop().unwrap() {
                                OperationElt::Operand(val) => Ok(val),
                                _ => Err("Not a suitable number on the stack"),
                            }.unwrap();
                            let result = match operator {
                                Operator::Plus => operand1 + operand2,
                                Operator::Minus => operand2 - operand1,
                                Operator::Multiply => operand1 * operand2,
                                Operator::Divide => operand2 / operand1,
                                Operator::Modulo => operand2 % operand1,
                                Operator::Pow => operand2.powf(operand1),
                                Operator::Gcd => {
                                    // Euclids Algorithm
                                    let mut x = operand1 as i32;
                                    let mut y = operand2 as i32;
                                    while y != 0 {
                                        let t = y;
                                        y = x % y;
                                        x = t;
                                    }
                                    x as f32
                                }
                                Operator::Hypot => {
                                    (operand1.powf(2_f32) + operand2.powf(2_f32)).sqrt()
                                }
                                Operator::Max => operand1.max(operand2),
                                Operator::Min => operand1.min(operand2),
                                Operator::Or => (operand1.to_bits() | operand2.to_bits()) as f32,
                                Operator::Xor => (operand1.to_bits() ^ operand2.to_bits()) as f32,
                                _ => operand1 + operand2,
                            };
                            stack.push(OperationElt::Operand(result));
                        } else if stack.len() == 1 {
                            let operand1 = match stack.pop().unwrap() {
                                OperationElt::Operand(val) => Ok(val),
                                _ => Err("Not a suitable number on the stack"),
                            }.unwrap();
                            // Operators with 1 operand
                            match operator {
                                // Operators not returning any value
                                Operator::Dropn => {
                                    for _x in 0..(operand1 as usize) {
                                        let _thrash = stack.pop();
                                    }
                                }
                                Operator::Dupn => {
                                    let mut stack_copy = Vec::<OperationElt>::new();
                                    for _x in 0..(operand1 as usize) {
                                        stack_copy.push(stack.pop().unwrap());
                                    }
                                    // Push what we got twice onto the stack
                                    for _x in 0..(operand1 as usize) {
                                        stack.push(stack_copy.pop().unwrap());
                                    }
                                }
                                Operator::Push => {
                                    stack.push(OperationElt::Operand(operand1));
                                }
                                Operator::Rolld => {
                                    let stack_top =
                                        stack.pop().expect("No value found on this stack level");
                                    stack.insert(operand1 as usize - 1, stack_top);
                                }
                                _ => {}
                            }

                            let result = match operator {
                                // Operators returning a value
                                Operator::Fact => {
                                    permutohedron::factorial(operand1 as usize) as f32
                                }
                                Operator::Fabs => operand1.abs(),
                                Operator::Acos => operand1.acos(),
                                Operator::Acosh => operand1.acosh(),
                                Operator::Asin => operand1.asin(),
                                Operator::Asinh => operand1.asinh(),
                                Operator::Tan => operand1.tan(),
                                Operator::Tanh => operand1.tanh(),
                                Operator::Atan => operand1.atan(),
                                Operator::Atanh => operand1.atanh(),
                                Operator::Exp10 => 10_f32.powf(operand1),
                                Operator::Cbrt => operand1.cbrt(),
                                Operator::Ceil => operand1.ceil(),
                                Operator::Chs => -operand1,
                                Operator::Cos => operand1.cos(),
                                Operator::Cosh => operand1.cosh(),
                                Operator::Exp => operand1.exp(),
                                Operator::Expm1 => operand1.exp_m1(),
                                Operator::Floor => operand1.floor(),
                                Operator::Fp => operand1.fract(),
                                Operator::Inv => operand1.recip(),
                                Operator::Ip => operand1.trunc(),
                                Operator::Log2 => operand1.log2(),
                                Operator::Logb => operand1.log2().trunc(),
                                Operator::Log10 => operand1.log10(),
                                Operator::Log => operand1.ln(),
                                Operator::Log1p => (operand1 + 1_f32).ln(),
                                Operator::Not => !operand1.to_bits() as f32,
                                Operator::Rint => operand1.round(),
                                Operator::Roll => match stack.remove(operand1 as usize - 1) {
                                    OperationElt::Operand(val) => Ok(val),
                                    _ => Err("Not a suitable number on the stack"),
                                }.unwrap(),
                                Operator::Sign => {
                                    let sign;
                                    if operand1 < 0_f32 {
                                        sign = -1.0_f32;
                                    } else if operand1 == 0_f32 {
                                        sign = 0.0_f32;
                                    } else {
                                        sign = 1.0_f32;
                                    }
                                    sign
                                }
                                Operator::Sin => operand1.sin(),
                                Operator::Sinh => operand1.sinh(),
                                Operator::Sqr => operand1 * operand1,
                                Operator::Sqrt => operand1.sqrt(),
                                _ => 666_f32,
                            };
                            if result.is_nan() {
                            } else {
                                stack.push(OperationElt::Operand(result));
                            }
                        }
                        // Operators without operands
                        //if stack.len() == 0 {
                        match operator {
                            Operator::Setbin => {}
                            Operator::Setchar => {}
                            Operator::Clear => stack.clear(),
                            Operator::Setdec => {}
                            Operator::Sethex => {}
                            Operator::Setoct => {}
                            Operator::Help => {}
                            Operator::Depth => {
                                let length = stack.len();
                                stack.push(OperationElt::Operand(length as f32))
                            }
                            Operator::Rot => {
                                // Grab the last element of the stack, and put in the top of
                                // stack
                                let length = stack.len();
                                let stack_back = stack.remove(length - 1);
                                stack.push(stack_back);
                            }
                            Operator::Pop => {
                                let _thrash = stack.pop();
                            }
                            Operator::Drop => {
                                let _thrash = stack.pop();
                            }
                            Operator::Drop2 => {
                                let mut _thrash = stack.pop();
                                _thrash = stack.pop();
                            }
                            Operator::Dupel => {
                                let stack_top = stack.pop().unwrap();
                                stack.push(stack_top.clone());
                                stack.push(stack_top);
                            }
                            Operator::Dupel2 => {
                                let stack_top =
                                    stack.pop().expect("Expected OperationElt on stack");
                                let stack_second =
                                    stack.pop().expect("Expected OperationElt on stack");
                                // Push what we got twice onto the stack
                                stack.push(stack_second.clone());
                                stack.push(stack_top.clone());
                                stack.push(stack_second);
                                stack.push(stack_top);
                            }
                            Operator::Prod => {
                                let mut product = match stack.pop().unwrap() {
                                    OperationElt::Operand(val) => Ok(val),
                                    _ => Err(println!("Error, can not multiply operators")),
                                }.unwrap();
                                for _x in 0..(stack.len() - 1) {
                                    product = product * match stack.pop().unwrap() {
                                        OperationElt::Operand(val) => Ok(val),
                                        _ => Err(println!("Error, can not multiply operators")),
                                    }.unwrap();
                                }
                                stack.push(OperationElt::Operand(product as f32));
                            }
                            Operator::Sum => {
                                let mut sum = match stack.pop().unwrap() {
                                    OperationElt::Operand(val) => Ok(val),
                                    _ => Err(println!("Error, can not add operators")),
                                }.unwrap();
                                for _x in 0..(stack.len() - 1) {
                                    sum = sum + match stack.pop().unwrap() {
                                        OperationElt::Operand(val) => Ok(val),
                                        _ => Err(println!("Error, can not add operators")),
                                    }.unwrap();
                                }
                                stack.push(OperationElt::Operand(sum as f32));
                            }
                            Operator::Rnd => {
                                let mut rng = rand::thread_rng();
                                stack.push(OperationElt::Operand(rng.gen_range(0.0, 1.0)));
                            }
                            Operator::Swap => {
                                let stack_first =
                                    stack.pop().expect("expected f32 values in stack");
                                let stack_second =
                                    stack.pop().expect("expected f32 values in stack");
                                stack.push(stack_first);
                                stack.push(stack_second);
                            }
                            _ => {}
                        };
                    }
                    OperationElt::Operand(val) => stack.push(OperationElt::Operand(val)),
                    //OperationElt::Text() => println!("Text case!"),
                }
            }
            //if stack.len() != 1 {
            //return Err("Remaining untreated operands. Probably missing operator.".to_string());
            //}
            return Ok(stack);
        }
        Err(err) => Err(err),
    };
}

#[test]
fn it_adds() {
    let mut stack: Vec<OperationElt> = Vec::new();
    let result = evaluate(&mut stack, "1 2 +").unwrap();
    assert_eq!(
        match result.pop().unwrap() {
            OperationElt::Operand(val) => Ok(val),
            _ => Err("No operand on stack"),
        }.unwrap(),
        3.0
    );
}

#[test]
fn it_substracts() {
    let mut stack: Vec<OperationElt> = Vec::new();
    let result = evaluate(&mut stack, "1 2 -").unwrap();
    assert_eq!(
        match result.pop().unwrap() {
            OperationElt::Operand(val) => Ok(val),
            _ => Err("No operand on stack"),
        }.unwrap(),
        -1.0
    );
}

#[test]
fn it_multiplies() {
    let mut stack: Vec<OperationElt> = Vec::new();
    let result = evaluate(&mut stack, "6 7 *").unwrap();
    assert_eq!(
        match result.pop().unwrap() {
            OperationElt::Operand(val) => Ok(val),
            _ => Err("No operand on stack"),
        }.unwrap(),
        42.0
    );
}

#[test]
fn it_divides() {
    let mut stack: Vec<OperationElt> = Vec::new();
    let result = evaluate(&mut stack, "1 2 /").unwrap();
    assert_eq!(
        match result.pop().unwrap() {
            OperationElt::Operand(val) => Ok(val),
            _ => Err("No operand on stack"),
        }.unwrap(),
        0.5
    );
}

#[test]
fn it_modulos() {
    let mut stack: Vec<OperationElt> = Vec::new();
    let result = evaluate(&mut stack, "4 2 mod").unwrap();
    assert_eq!(
        match result.pop().unwrap() {
            OperationElt::Operand(val) => Ok(val),
            _ => Err("No operand on stack"),
        }.unwrap(),
        0.0
    );
}

#[test]
fn it_evaluates_complex_expressions() {
    // ((1+2) * 8 / (5-1) - 4 % 3) / 2
    let mut stack: Vec<OperationElt> = Vec::new();
    let result = evaluate(&mut stack, "1 2 + 8 * 5 1 - / 4 3 mod - 2 /").unwrap();
    assert_eq!(
        match result.pop().unwrap() {
            OperationElt::Operand(val) => Ok(val),
            _ => Err("No operand on stack"),
        }.unwrap(),
        2.5
    );
}

#[test]
fn it_allows_multiple_whitespaces() {
    let mut stack: Vec<OperationElt> = Vec::new();
    let result = evaluate(&mut stack, "1  2 +\t3 -").unwrap();
    assert_eq!(
        match result.pop().unwrap() {
            OperationElt::Operand(val) => Ok(val),
            _ => Err("No operand on stack"),
        }.unwrap(),
        0.0
    );
}

#[test]
fn it_fails_for_unsupported_characters() {
    let mut stack: Vec<OperationElt> = Vec::new();
    let result = evaluate(&mut stack, "1 2 t");
    assert_eq!(result.unwrap_err(), "Cannot parse operand \"t\"");
}

/*
#[test]
fn it_fails_for_insufficient_operands() {
    let mut stack: Vec<OperationElt> = Vec::new();
    let result = evaluate(&mut stack, "1 +");
    assert_eq!(result.unwrap_err(), "Insufficient operands before operator");
}
*/

#[test]
fn it_fails_for_insufficient_operators() {
    let mut stack: Vec<OperationElt> = Vec::new();
    let result = evaluate(&mut stack, "1 2 3 +");
    assert_eq!(result.unwrap().len(), 2);
}
