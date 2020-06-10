extern crate rand;

use rand::Rng;
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
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

/*
impl Operator {
    pub fn iterator() -> Iter<'static, Operator> {
        static OPERATIONS: [Operator; 101] = [
            "Acos",
            "Acosh",
            "And",
            "Asin",
            "Asinh",
            "Atan",
            "Atan2",
            "Atanh",
            "Cbrt",
            "Ceil",
            "Chs",
            "Clear",
            "Cos",
            "Cosh",
            "Deltapercent",
            "Depth",
            "Divide",
            "Drop",
            "Drop2",
            "Dropn",
            "Dupel",
            "Dupel2",
            "Dupn",
            "Erf",
            "Erfc",
            "Exp",
            "Exp10",
            "Expm1",
            "Fabs",
            "Fact",
            "Floor",
            "Fp",
            "Gcd",
            "Help",
            "Hypot",
            "Idiv",
            "Inv",
            "Ip",
            "J0",
            "J1",
            "Jn",
            "Lgamma",
            "Log",
            "Log10",
            "Log1p",
            "Log2",
            "Logb",
            "Max",
            "Min",
            "Minus",
            "Modulo",
            "Multiply",
            "Not",
            "Or",
            "Over",
            "Percent",
            "Percenttotal",
            "Pick",
            "Plus",
            "Pop",
            "Pow",
            "Prec",
            "Prod",
            "Push",
            "Quit",
            "Rdz",
            "Rint",
            "Rnd",
            "Roll",
            "Rolld",
            "Rot",
            "Setbin",
            "Setchar",
            "Setdec",
            "Sethex",
            "Setoct",
            "Showstack",
            "Sign",
            "Sin",
            "Sinh",
            "Sl",
            "Slb",
            "Sqr",
            "Sqrt",
            "Sr",
            "Srb",
            "Sum",
            "Swap",
            "Tan",
            "Tanh",
            "Warranty",
            "Xor",
            "Y0",
            "Y1",
            "Yn",
        ];
        OPERATIONS.into_iter()
    }
}
*/

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Operator::Acos => write!(f, ""),
            Operator::Acosh => write!(f, ""),
            Operator::And => write!(f, ""),
            Operator::Asin => write!(f, ""),
            Operator::Asinh => write!(f, ""),
            Operator::Atan => write!(f, ""),
            Operator::Atan2 => write!(f, ""),
            Operator::Atanh => write!(f, ""),
            Operator::Cbrt => write!(f, ""),
            Operator::Ceil => write!(f, ""),
            Operator::Chs => write!(f, ""),
            Operator::Clear => write!(f, ""),
            Operator::Cos => write!(f, ""),
            Operator::Cosh => write!(f, ""),
            Operator::Deltapercent => write!(f, ""),
            Operator::Depth => write!(f, ""),
            Operator::Divide => write!(f, ""),
            Operator::Drop => write!(f, ""),
            Operator::Drop2 => write!(f, ""),
            Operator::Dropn => write!(f, ""),
            Operator::Dupel => write!(f, ""),
            Operator::Dupel2 => write!(f, ""),
            Operator::Dupn => write!(f, ""),
            Operator::Erf => write!(f, ""),
            Operator::Erfc => write!(f, ""),
            Operator::Exp => write!(f, ""),
            Operator::Exp10 => write!(f, ""),
            Operator::Expm1 => write!(f, ""),
            Operator::Fabs => write!(f, ""),
            Operator::Fact => write!(f, ""),
            Operator::Floor => write!(f, ""),
            Operator::Fp => write!(f, ""),
            Operator::Gcd => write!(f, ""),
            Operator::Help => write!(f, ""),
            Operator::Hypot => write!(f, ""),
            Operator::Idiv => write!(f, ""),
            Operator::Inv => write!(f, ""),
            Operator::Ip => write!(f, ""),
            Operator::J0 => write!(f, ""),
            Operator::J1 => write!(f, ""),
            Operator::Jn => write!(f, ""),
            Operator::Lgamma => write!(f, ""),
            Operator::Log => write!(f, ""),
            Operator::Log10 => write!(f, ""),
            Operator::Log1p => write!(f, ""),
            Operator::Log2 => write!(f, ""),
            Operator::Logb => write!(f, ""),
            Operator::Max => write!(f, ""),
            Operator::Min => write!(f, ""),
            Operator::Minus => write!(f, ""),
            Operator::Modulo => write!(f, ""),
            Operator::Multiply => write!(f, ""),
            Operator::Not => write!(f, ""),
            Operator::Or => write!(f, ""),
            Operator::Over => write!(f, ""),
            Operator::Percent => write!(f, ""),
            Operator::Percenttotal => write!(f, ""),
            Operator::Pick => write!(f, ""),
            Operator::Plus => write!(f, ""),
            Operator::Pop => write!(f, ""),
            Operator::Pow => write!(f, ""),
            Operator::Prec => write!(f, ""),
            Operator::Prod => write!(f, ""),
            Operator::Push => write!(f, ""),
            Operator::Quit => write!(f, ""),
            Operator::Rdz => write!(f, ""),
            Operator::Rint => write!(f, ""),
            Operator::Rnd => write!(f, ""),
            Operator::Roll => write!(f, ""),
            Operator::Rolld => write!(f, ""),
            Operator::Rot => write!(f, ""),
            Operator::Setbin => write!(f, ""),
            Operator::Setchar => write!(f, ""),
            Operator::Setdec => write!(f, ""),
            Operator::Sethex => write!(f, ""),
            Operator::Setoct => write!(f, ""),
            Operator::Showstack => write!(f, ""),
            Operator::Sign => write!(f, ""),
            Operator::Sin => write!(f, ""),
            Operator::Sinh => write!(f, ""),
            Operator::Sl => write!(f, ""),
            Operator::Slb => write!(f, ""),
            Operator::Sqr => write!(f, ""),
            Operator::Sqrt => write!(f, ""),
            Operator::Sr => write!(f, ""),
            Operator::Srb => write!(f, ""),
            Operator::Sum => write!(f, ""),
            Operator::Swap => write!(f, ""),
            Operator::Tan => write!(f, ""),
            Operator::Tanh => write!(f, ""),
            Operator::Warranty => write!(f, ""),
            Operator::Xor => write!(f, ""),
            Operator::Y0 => write!(f, ""),
            Operator::Y1 => write!(f, ""),
            Operator::Yn => write!(f, ""),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Stack {
    Operator(Operator),
    Operand(f32),
    //Text(String),
}

fn tokenizer(expr: &str) -> Result<Vec<Stack>, String> {
    expr.split_whitespace()
        .map(|el| match el {
            "!" => Ok(Stack::Operator(Operator::Fact)), // "factorial"                               },
            "%" => Ok(Stack::Operator(Operator::Percent)), // "xy/100"                                  },
            "%ch" => Ok(Stack::Operator(Operator::Deltapercent)), // "100(y-x)/x"                           },
            "%t" => Ok(Stack::Operator(Operator::Percenttotal)), // "100y/x"                               },
            "&" => Ok(Stack::Operator(Operator::And)), // "logical and"                             },
            "*" => Ok(Stack::Operator(Operator::Multiply)), // "multiplication"                          },
            "+" => Ok(Stack::Operator(Operator::Plus)), // "addition"                                },
            "-" => Ok(Stack::Operator(Operator::Minus)), // "subtraction"                             },
            "/" => Ok(Stack::Operator(Operator::Divide)), // "division"                                },
            "?" => Ok(Stack::Operator(Operator::Help)), // "show all known functions and commands"   },
            "^" => Ok(Stack::Operator(Operator::Pow)), // "exponentiation"                          },
            "abs" => Ok(Stack::Operator(Operator::Fabs)), // "absolute value: |x|"                     },
            "acos" => Ok(Stack::Operator(Operator::Acos)), // "arc cosine"                              },
            "acosh" => Ok(Stack::Operator(Operator::Acosh)), // "inverse hyperbolic cosine"               },
            "alog" => Ok(Stack::Operator(Operator::Exp10)), // "exponentiation to the base 10: 10^x"     },
            "and" => Ok(Stack::Operator(Operator::And)), // "logical and"                             },
            "arg" => Ok(Stack::Operator(Operator::Atan2)), // "arc tangent of 2 variables"              },
            "asin" => Ok(Stack::Operator(Operator::Asin)), // "arc sine"                                },
            "asinh" => Ok(Stack::Operator(Operator::Asinh)), // "inverse hyperbolic sine"                 },
            "atan" => Ok(Stack::Operator(Operator::Atan)), // "arc tangent"                             },
            "atan2" => Ok(Stack::Operator(Operator::Atan2)), // "arc tangent of 2 variables"              },
            "atanh" => Ok(Stack::Operator(Operator::Atanh)), // "inverse hyperbolic tangent"              },
            "bin" => Ok(Stack::Operator(Operator::Setbin)), // "set binary"                              },
            "cbrt" => Ok(Stack::Operator(Operator::Cbrt)), // "cube root: x^1/3"                        },
            "ceil" => Ok(Stack::Operator(Operator::Ceil)), // "smallest integral value not less than x" },
            "char" => Ok(Stack::Operator(Operator::Setchar)), // "display ASCII values"                    },
            "chs" => Ok(Stack::Operator(Operator::Chs)), // "change sign"                             },
            "clear" => Ok(Stack::Operator(Operator::Clear)), // "clear stack"                             },
            "cos" => Ok(Stack::Operator(Operator::Cos)), // "cosine"                                  },
            "cosh" => Ok(Stack::Operator(Operator::Cosh)), // "arcus cosine"                            },
            "dec" => Ok(Stack::Operator(Operator::Setdec)), // "switch to decimal base"                  },
            "depth" => Ok(Stack::Operator(Operator::Depth)), // "show depth of stack"                     },
            "div" => Ok(Stack::Operator(Operator::Idiv)), // "divide"                                  },
            "drop" => Ok(Stack::Operator(Operator::Drop)), // "drop last stack element"                 },
            "drop2" => Ok(Stack::Operator(Operator::Drop2)), // "drop last 2 stack elements"              },
            "dropn" => Ok(Stack::Operator(Operator::Dropn)), // "drop last n stack elements"              },
            "dup" => Ok(Stack::Operator(Operator::Dupel)), // "duplicate last stack element"            },
            "dup2" => Ok(Stack::Operator(Operator::Dupel2)), // "duplicate last 2 stack elements"         },
            "dupn" => Ok(Stack::Operator(Operator::Dupn)), // "duplicate last n stack elements"         },
            "erf" => Ok(Stack::Operator(Operator::Erf)), // "error function"                          },
            "erfc" => Ok(Stack::Operator(Operator::Erfc)), // "complementary error function: 1-erf(x)"  },
            "exit" => Ok(Stack::Operator(Operator::Quit)), // "exit rpncalc"                            },
            "exp" => Ok(Stack::Operator(Operator::Exp)), // "exponential function: e^x"               },
            "expm" => Ok(Stack::Operator(Operator::Expm1)), // "e^x-1"                                   },
            "fact" => Ok(Stack::Operator(Operator::Fact)), // "factorial"                               },
            "floor" => Ok(Stack::Operator(Operator::Floor)), // "largest integral value not greater than x" },
            "fp" => Ok(Stack::Operator(Operator::Fp)), // "floating point part of x"                },
            "gcd" => Ok(Stack::Operator(Operator::Gcd)), // "greatest common divisor"                 },
            "help" => Ok(Stack::Operator(Operator::Help)), // "show all known functions and commands"   },
            "hex" => Ok(Stack::Operator(Operator::Sethex)), // "switch to hexadecimal base"              },
            "hypot" => Ok(Stack::Operator(Operator::Hypot)), // "Euclidean distance function: sqrt(x^2+y^2)" },
            "inv" => Ok(Stack::Operator(Operator::Inv)), // "inverse: 1/x"                            },
            "ip" => Ok(Stack::Operator(Operator::Ip)), // "largest integral value not greater than x" },
            "j0" => Ok(Stack::Operator(Operator::J0)), // "Bessel function of zeroth order"         },
            "j1" => Ok(Stack::Operator(Operator::J1)), // "Bessel function of first  order"         },
            "jn" => Ok(Stack::Operator(Operator::Jn)), // "Bessel function of n-th order"           },
            "ld" => Ok(Stack::Operator(Operator::Log2)), // "base-2 logarithm"                        },
            "ldb" => Ok(Stack::Operator(Operator::Logb)), // "integer part of the base-2 logarithm"    },
            "lg" => Ok(Stack::Operator(Operator::Log10)), // "logarithm to the base 10"                },
            "lgamma" => Ok(Stack::Operator(Operator::Lgamma)), // "ln|gamma(x)|"                            },
            "ln" => Ok(Stack::Operator(Operator::Log)), // "natural log"                             },
            "lnp1" => Ok(Stack::Operator(Operator::Log1p)), // "ln(1+x)"                                 },
            "log" => Ok(Stack::Operator(Operator::Log10)), // "logarithm to the base 10"                },
            "max" => Ok(Stack::Operator(Operator::Max)), // "max(x,y)"                                },
            "min" => Ok(Stack::Operator(Operator::Min)), // "min(x,y)"                                },
            "mod" => Ok(Stack::Operator(Operator::Modulo)), // "modulo function"                         },
            "neg" => Ok(Stack::Operator(Operator::Chs)), // "change sign"                             },
            "not" => Ok(Stack::Operator(Operator::Not)), // "logical not"                             },
            "oct" => Ok(Stack::Operator(Operator::Setoct)), // "switch to octal base"                    },
            "or" => Ok(Stack::Operator(Operator::Or)), // "logical or"                              },
            "over" => Ok(Stack::Operator(Operator::Over)), // "push second last element on stack"       },
            "pick" => Ok(Stack::Operator(Operator::Pick)), // "duplicate a element from the stack"      },
            "pop" => Ok(Stack::Operator(Operator::Pop)), // "drop last element from stack"            },
            "prec" => Ok(Stack::Operator(Operator::Prec)), // "set precision"                           },
            "prod" => Ok(Stack::Operator(Operator::Prod)), // "product of all elements on stack"        },
            "push" => Ok(Stack::Operator(Operator::Push)), // "push a new element on the stack"         },
            "quit" => Ok(Stack::Operator(Operator::Quit)), // "exit rpncalc"                            },
            "rand" => Ok(Stack::Operator(Operator::Rnd)), // "random value [0, 1["                     },
            "rdz" => Ok(Stack::Operator(Operator::Rdz)), // "set seed for random value generator"     },
            "rint" => Ok(Stack::Operator(Operator::Rint)), // "round to closest integer"                },
            "rnd" => Ok(Stack::Operator(Operator::Rint)), // "round to closest integer"                },
            "roll" => Ok(Stack::Operator(Operator::Roll)), // "roll the stack"                          },
            "rolld" => Ok(Stack::Operator(Operator::Rolld)), // "roll the stack down"                     },
            "rot" => Ok(Stack::Operator(Operator::Rot)), // "rotate the stack"                        },
            "shl" => Ok(Stack::Operator(Operator::Sl)), // "shift left"                              },
            "show" => Ok(Stack::Operator(Operator::Showstack)), // "redisplay stack"                         },
            "shr" => Ok(Stack::Operator(Operator::Sr)), // "shift right"                             },
            "sign" => Ok(Stack::Operator(Operator::Sign)), // "sign of x"                               },
            "sin" => Ok(Stack::Operator(Operator::Sin)), // "sine"                                    },
            "sinh" => Ok(Stack::Operator(Operator::Sinh)), // "hyperbolic sine"                         },
            "sl" => Ok(Stack::Operator(Operator::Sl)), // "shift left"                              },
            "slb" => Ok(Stack::Operator(Operator::Slb)), // "shift left 1 byte"                       },
            "sq" => Ok(Stack::Operator(Operator::Sqr)), // "square: x^2"                             },
            "sqr" => Ok(Stack::Operator(Operator::Sqr)), // "square: x^2"                             },
            "sqrt" => Ok(Stack::Operator(Operator::Sqrt)), // "square root"                             },
            "sr" => Ok(Stack::Operator(Operator::Sr)), // "shift right"                             },
            "srb" => Ok(Stack::Operator(Operator::Srb)), // "shift right 1 byte"                      },
            "sum" => Ok(Stack::Operator(Operator::Sum)), // "sum of all elements on stack"            },
            "swap" => Ok(Stack::Operator(Operator::Swap)), // "swap last two elements"                  },
            "tan" => Ok(Stack::Operator(Operator::Tan)), // "tangent"                                 },
            "tanh" => Ok(Stack::Operator(Operator::Tanh)), // "hyperbolic tangent"                      },
            "warranty" => Ok(Stack::Operator(Operator::Warranty)), // "show absence of warranty"                },
            "xor" => Ok(Stack::Operator(Operator::Xor)), // "logical xor"                             },
            "y0" => Ok(Stack::Operator(Operator::Y0)), // "Bessel function of the second kind of zeroth order" },
            "y1" => Ok(Stack::Operator(Operator::Y1)), // "Bessel function of the second kind of first order"  },
            "yn" => Ok(Stack::Operator(Operator::Yn)), // "Bessel function of the second kind of n-th order"   },
            "|" => Ok(Stack::Operator(Operator::Or)), // "logical or"                              },
            "~" => Ok(Stack::Operator(Operator::Not)), // "logical not"                             }
            operand => match operand.parse::<f32>() {
                Ok(val) => Ok(Stack::Operand(val)),
                Err(_) => Err(format!("Cannot parse operand \"{}\"", operand)),
            },
        })
        .into_iter()
        .collect()
}

/// Pop an operand from the stack returning the inner f32
///
///
fn stack_pop<'a>(stack: &'a mut Vec<Stack>) -> Result<f32, String> {
    match stack.pop().unwrap() {
        Stack::Operand(val) => Ok(val),
        _ => Err("Not a suitable number on the stack".to_string()),
    }
}

/// Evaluates an RPL expression.
///
/// # Examples
/// ````
/// extern crate rpn;
/// use rpn::Stack;
///
/// let mut stack: Vec<Stack> = Vec::new();
/// let result = rpn::evaluate(&mut stack, "5 2 +").unwrap();
/// assert_eq!(
///     match result.pop().unwrap() {
///         Stack::Operand(val) => Ok(val),
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
pub fn evaluate<'a>(stack: &'a mut Vec<Stack>, expr: &str) -> Result<&'a mut Vec<Stack>, String> {
    return match tokenizer(expr) {
        Ok(tokens) => {
            for token in tokens {
                match token {
                    Stack::Operator(operator) => {
                        // Parse the operation to use

                        //let (stack, result) = evaluate_token(&mut stack, operator);
                        // Operators with 2 operands
                        match operator {
                            Operator::Plus => {
                                let operand1 = stack_pop(stack)?;
                                let operand2 = stack_pop(stack)?;

                                stack.push(Stack::Operand(operand1 + operand2))
                            }
                            Operator::Minus => {
                                let operand1 = stack_pop(stack)?;
                                let operand2 = stack_pop(stack)?;

                                stack.push(Stack::Operand(operand2 - operand1))
                            }
                            Operator::Multiply => {
                                let operand1 = stack_pop(stack)?;
                                let operand2 = stack_pop(stack)?;

                                stack.push(Stack::Operand(operand1 * operand2))
                            }
                            Operator::Divide => {
                                let operand1 = stack_pop(stack)?;
                                let operand2 = stack_pop(stack)?;

                                stack.push(Stack::Operand(operand2 / operand1))
                            }
                            Operator::Modulo => {
                                let operand1 = stack_pop(stack)?;
                                let operand2 = stack_pop(stack)?;

                                stack.push(Stack::Operand(operand2 % operand1))
                            }
                            Operator::Pow => {
                                let operand1 = stack_pop(stack)?;
                                let operand2 = stack_pop(stack)?;

                                stack.push(Stack::Operand(operand2.powf(operand1)))
                            }
                            Operator::Gcd => {
                                let operand1 = stack_pop(stack)?;
                                let operand2 = stack_pop(stack)?;

                                // Euclids Algorithm
                                let mut x = operand1 as i32;
                                let mut y = operand2 as i32;
                                while y != 0 {
                                    let t = y;
                                    y = x % y;
                                    x = t;
                                }
                                stack.push(Stack::Operand(x as f32))
                            }
                            Operator::Hypot => {
                                let operand1 = stack_pop(stack)?;
                                let operand2 = stack_pop(stack)?;

                                stack.push(Stack::Operand(
                                    (operand1.powf(2_f32) + operand2.powf(2_f32)).sqrt(),
                                ))
                            }
                            Operator::Max => {
                                let operand1 = stack_pop(stack)?;
                                let operand2 = stack_pop(stack)?;

                                stack.push(Stack::Operand(operand1.max(operand2)))
                            }
                            Operator::Min => {
                                let operand1 = stack_pop(stack)?;
                                let operand2 = stack_pop(stack)?;
                                stack.push(Stack::Operand(operand1.min(operand2)))
                            }
                            Operator::Or => {
                                let operand1 = stack_pop(stack)?;
                                let operand2 = stack_pop(stack)?;

                                stack.push(Stack::Operand(
                                    (operand1.to_bits() | operand2.to_bits()) as f32,
                                ))
                            }
                            Operator::Xor => {
                                let operand1 = stack_pop(stack)?;
                                let operand2 = stack_pop(stack)?;

                                stack.push(Stack::Operand(
                                    (operand1.to_bits() ^ operand2.to_bits()) as f32,
                                ))
                            }
                            // Operators with 1 operand
                            Operator::Fact => {
                                let operand1 = stack_pop(stack)?;
                                stack.push(Stack::Operand(permutohedron::factorial(
                                    operand1 as usize,
                                ) as f32))
                            }
                            Operator::Fabs => {
                                let operand1 = stack_pop(stack)?;
                                stack.push(Stack::Operand(operand1.abs()))
                            }
                            Operator::Acos => {
                                let operand1 = stack_pop(stack)?;
                                stack.push(Stack::Operand(operand1.acos()))
                            }
                            Operator::Acosh => {
                                let operand1 = stack_pop(stack)?;
                                stack.push(Stack::Operand(operand1.acosh()))
                            }
                            Operator::Asin => {
                                let operand1 = stack_pop(stack)?;
                                stack.push(Stack::Operand(operand1.asin()))
                            }
                            Operator::Asinh => {
                                let operand1 = stack_pop(stack)?;
                                stack.push(Stack::Operand(operand1.asinh()))
                            }
                            Operator::Tan => {
                                let operand1 = stack_pop(stack)?;
                                stack.push(Stack::Operand(operand1.tan()))
                            }
                            Operator::Tanh => {
                                let operand1 = stack_pop(stack)?;
                                stack.push(Stack::Operand(operand1.tanh()))
                            }
                            Operator::Atan => {
                                let operand1 = stack_pop(stack)?;
                                stack.push(Stack::Operand(operand1.atan()))
                            }
                            Operator::Atanh => {
                                let operand1 = stack_pop(stack)?;
                                stack.push(Stack::Operand(operand1.atanh()))
                            }
                            Operator::Exp10 => {
                                let operand1 = stack_pop(stack)?;
                                stack.push(Stack::Operand(10_f32.powf(operand1)))
                            }
                            Operator::Cbrt => {
                                let operand1 = stack_pop(stack)?;
                                stack.push(Stack::Operand(operand1.cbrt()))
                            }
                            Operator::Ceil => {
                                let operand1 = stack_pop(stack)?;
                                stack.push(Stack::Operand(operand1.ceil()))
                            }
                            Operator::Chs => {
                                let operand1 = stack_pop(stack)?;
                                stack.push(Stack::Operand(-operand1))
                            }
                            Operator::Cos => {
                                let operand1 = stack_pop(stack)?;
                                stack.push(Stack::Operand(operand1.cos()))
                            }
                            Operator::Cosh => {
                                let operand1 = stack_pop(stack)?;
                                stack.push(Stack::Operand(operand1.cosh()))
                            }
                            Operator::Exp => {
                                let operand1 = stack_pop(stack)?;
                                stack.push(Stack::Operand(operand1.exp()))
                            }
                            Operator::Expm1 => {
                                let operand1 = stack_pop(stack)?;
                                stack.push(Stack::Operand(operand1.exp_m1()))
                            }
                            Operator::Floor => {
                                let operand1 = stack_pop(stack)?;
                                stack.push(Stack::Operand(operand1.floor()))
                            }
                            Operator::Fp => {
                                let operand1 = stack_pop(stack)?;
                                stack.push(Stack::Operand(operand1.fract()))
                            }
                            Operator::Inv => {
                                let operand1 = stack_pop(stack)?;
                                stack.push(Stack::Operand(operand1.recip()))
                            }
                            Operator::Ip => {
                                let operand1 = stack_pop(stack)?;
                                stack.push(Stack::Operand(operand1.trunc()))
                            }
                            Operator::Log2 => {
                                let operand1 = stack_pop(stack)?;
                                stack.push(Stack::Operand(operand1.log2()))
                            }
                            Operator::Logb => {
                                let operand1 = stack_pop(stack)?;
                                stack.push(Stack::Operand(operand1.log2().trunc()))
                            }
                            Operator::Log10 => {
                                let operand1 = stack_pop(stack)?;
                                stack.push(Stack::Operand(operand1.log10()))
                            }
                            Operator::Log => {
                                let operand1 = stack_pop(stack)?;
                                stack.push(Stack::Operand(operand1.ln()))
                            }
                            Operator::Log1p => {
                                let operand1 = stack_pop(stack)?;
                                stack.push(Stack::Operand((operand1 + 1_f32).ln()))
                            }
                            Operator::Not => {
                                let operand1 = stack_pop(stack)?;
                                stack.push(Stack::Operand(!operand1.to_bits() as f32))
                            }
                            Operator::Rint => {
                                let operand1 = stack_pop(stack)?;
                                stack.push(Stack::Operand(operand1.round()))
                            }
                            Operator::Roll => {
                                // Get the given index to push to the stack top
                                let operand1 = stack_pop(stack)?;
                                let result = match stack.remove(stack.len() - operand1 as usize) {
                                    Stack::Operand(val) => Ok(Stack::Operand(val)),
                                    _ => Err("Not a suitable number on the stack"),
                                }?;
                                stack.push(result)
                            }
                            Operator::Sign => {
                                let operand1 = stack_pop(stack)?;
                                let sign;
                                if operand1 < 0_f32 {
                                    sign = -1.0_f32;
                                } else if operand1 == 0_f32 {
                                    sign = 0.0_f32;
                                } else {
                                    sign = 1.0_f32;
                                }
                                stack.push(Stack::Operand(sign))
                            }
                            Operator::Sin => {
                                let operand1 = stack_pop(stack)?;
                                stack.push(Stack::Operand(operand1.sin()))
                            }
                            Operator::Sinh => {
                                let operand1 = stack_pop(stack)?;
                                stack.push(Stack::Operand(operand1.sinh()))
                            }
                            Operator::Sqr => {
                                let operand1 = stack_pop(stack)?;
                                stack.push(Stack::Operand(operand1 * operand1))
                            }
                            Operator::Sqrt => {
                                let operand1 = stack_pop(stack)?;
                                stack.push(Stack::Operand(operand1.sqrt()))
                            }
                            // Operators not returning any value
                            Operator::Dropn => {
                                let operand1 = stack_pop(stack)?;

                                for _x in 0..(operand1 as usize) {
                                    let _thrash = stack.pop();
                                }
                            }
                            Operator::Dupn => {
                                let operand1 = stack_pop(stack)?;
                                let mut stack_copy = Vec::<Stack>::new();
                                for _x in 0..(operand1 as usize) {
                                    stack_copy.push(stack.pop().unwrap());
                                }
                                // Push what we got twice onto the stack
                                for _x in 0..(operand1 as usize) {
                                    stack.push(stack_copy.pop().unwrap());
                                }
                            }
                            Operator::Push => {
                                let operand1 = stack_pop(stack)?;
                                stack.push(Stack::Operand(operand1));
                            }
                            Operator::Rolld => {
                                let operand1 = stack_pop(stack)?;
                                let stack_top =
                                    stack.pop().expect("No value found on this stack level");
                                stack.insert(stack.len() - operand1 as usize, stack_top);
                            }

                            // Operators without operands
                            Operator::Setbin => {}
                            Operator::Setchar => {}
                            Operator::Clear => stack.clear(),
                            Operator::Setdec => {}
                            Operator::Sethex => {}
                            Operator::Setoct => {}
                            Operator::Help => {}
                            Operator::Depth => {
                                let length = stack.len();
                                stack.push(Stack::Operand(length as f32))
                            }
                            Operator::Rot => {
                                // Move the last element of the stack to the top of the stack
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
                                let stack_top = stack.pop().expect("Expected Stack on stack");
                                let stack_second = stack.pop().expect("Expected Stack on stack");
                                // Push what we got twice onto the stack
                                stack.push(stack_second.clone());
                                stack.push(stack_top.clone());
                                stack.push(stack_second);
                                stack.push(stack_top);
                            }
                            Operator::Prod => {
                                let mut product = match stack.pop().unwrap() {
                                    Stack::Operand(val) => Ok(val),
                                    _ => Err("Error, can not multiply operators".to_string()),
                                }?;
                                for _x in 0..(stack.len() - 1) {
                                    product = product * match stack.pop().unwrap() {
                                        Stack::Operand(val) => Ok(val),
                                        _ => Err("Error, can not multiply operators".to_string()),
                                    }?;
                                }
                                stack.push(Stack::Operand(product as f32));
                            }
                            Operator::Sum => {
                                let mut sum = match stack.pop().unwrap() {
                                    Stack::Operand(val) => Ok(val),
                                    _ => Err("Error, can not add operators".to_string()),
                                }?;
                                for _x in 0..(stack.len() - 1) {
                                    sum = sum + match stack.pop().unwrap() {
                                        Stack::Operand(val) => Ok(val),
                                        _ => Err("Error, can not add operators".to_string()),
                                    }?;
                                }
                                stack.push(Stack::Operand(sum as f32));
                            }
                            Operator::Rnd => {
                                let mut rng = rand::thread_rng();
                                stack.push(Stack::Operand(rng.gen_range(0.0, 1.0)));
                            }
                            Operator::Swap => {
                                let stack_first =
                                    stack.pop().expect("expected f32 values in stack");
                                let stack_second =
                                    stack.pop().expect("expected f32 values in stack");
                                stack.push(stack_first);
                                stack.push(stack_second);
                            }
                            Operator::And => {}
                            Operator::Atan2 => {}
                            Operator::Deltapercent => {}
                            Operator::Erf => {}
                            Operator::Erfc => {}
                            Operator::Idiv => {}
                            Operator::J0 => {}
                            Operator::J1 => {}
                            Operator::Jn => {}
                            Operator::Lgamma => {}
                            Operator::Over => {}
                            Operator::Percent => {}
                            Operator::Percenttotal => {}
                            Operator::Pick => {}
                            Operator::Prec => {}
                            Operator::Quit => {}
                            Operator::Rdz => {}
                            Operator::Showstack => {}
                            Operator::Sl => {}
                            Operator::Slb => {}
                            Operator::Sr => {}
                            Operator::Srb => {}
                            Operator::Warranty => {}
                            Operator::Y0 => {}
                            Operator::Y1 => {}
                            Operator::Yn => {}
                        };
                    }
                    Stack::Operand(val) => stack.push(Stack::Operand(val)),
                    //Stack::Text() => println!("Text case!"),
                }
            }
            return Ok(stack);
        }
        Err(err) => Err(err),
    };
}

#[test]
fn it_adds() {
    let mut stack: Vec<Stack> = Vec::new();
    let result = evaluate(&mut stack, "1 2 +").unwrap();
    assert_eq!(
        match result.pop().unwrap() {
            Stack::Operand(val) => Ok(val),
            _ => Err("No operand on stack".to_string()),
        }.unwrap(),
        3.0
    );
}

#[test]
fn it_substracts() {
    let mut stack: Vec<Stack> = Vec::new();
    let result = evaluate(&mut stack, "1 2 -").unwrap();
    assert_eq!(
        match result.pop().unwrap() {
            Stack::Operand(val) => Ok(val),
            _ => Err("No operand on stack".to_string()),
        }.unwrap(),
        -1.0
    );
}

#[test]
fn it_multiplies() {
    let mut stack: Vec<Stack> = Vec::new();
    let result = evaluate(&mut stack, "6 7 *").unwrap();
    assert_eq!(
        match result.pop().unwrap() {
            Stack::Operand(val) => Ok(val),
            _ => Err("No operand on stack".to_string()),
        }.unwrap(),
        42.0
    );
}

#[test]
fn it_divides() {
    let mut stack: Vec<Stack> = Vec::new();
    let result = evaluate(&mut stack, "1 2 /").unwrap();
    assert_eq!(
        match result.pop().unwrap() {
            Stack::Operand(val) => Ok(val),
            _ => Err("No operand on stack".to_string()),
        }.unwrap(),
        0.5
    );
}

#[test]
fn it_modulos() {
    let mut stack: Vec<Stack> = Vec::new();
    let result = evaluate(&mut stack, "4 2 mod").unwrap();
    assert_eq!(
        match result.pop().unwrap() {
            Stack::Operand(val) => Ok(val),
            _ => Err("No operand on stack".to_string()),
        }.unwrap(),
        0.0
    );
}

#[test]
fn it_square_root() {
    let mut stack: Vec<Stack> = Vec::new();
    let result = evaluate(&mut stack, "9 sqrt").unwrap();
    assert_eq!(
        match result.pop().unwrap() {
            Stack::Operand(val) => Ok(val),
            _ => Err("No operand on stack".to_string()),
        }.unwrap(),
        3.0
    );
}

#[test]
fn it_roll() {
    let mut stack: Vec<Stack> = Vec::new();
    let result = evaluate(&mut stack, "1 2 3 4 5 2 roll").unwrap();
    let expected = &mut vec![
        Stack::Operand(1_f32),
        Stack::Operand(2_f32),
        Stack::Operand(3_f32),
        Stack::Operand(5_f32),
        Stack::Operand(4_f32),
    ];
    assert_eq!(result, expected);
}

#[test]
fn it_rolld() {
    let mut stack: Vec<Stack> = Vec::new();
    let result = evaluate(&mut stack, "1 2 3 4 5 2 rolld").unwrap();
    let expected = &mut vec![
        Stack::Operand(1_f32),
        Stack::Operand(2_f32),
        Stack::Operand(5_f32),
        Stack::Operand(3_f32),
        Stack::Operand(4_f32),
    ];
    assert_eq!(result, expected);
}

#[test]
fn it_evaluates_complex_expressions() {
    // ((1+2) * 8 / (5-1) - 4 % 3) / 2
    let mut stack: Vec<Stack> = Vec::new();
    let result = evaluate(&mut stack, "1 2 + 8 * 5 1 - / 4 3 mod - 2 /").unwrap();
    assert_eq!(
        match result.pop().unwrap() {
            Stack::Operand(val) => Ok(val),
            _ => Err("No operand on stack".to_string()),
        }.unwrap(),
        2.5
    );
}

#[test]
fn it_allows_multiple_whitespaces() {
    let mut stack: Vec<Stack> = Vec::new();
    let result = evaluate(&mut stack, "1  2 +\t3 -").unwrap();
    assert_eq!(
        match result.pop().unwrap() {
            Stack::Operand(val) => Ok(val),
            _ => Err("No operand on stack".to_string()),
        }.unwrap(),
        0.0
    );
}

#[test]
fn it_fails_for_unsupported_characters() {
    let mut stack: Vec<Stack> = Vec::new();
    let result = evaluate(&mut stack, "1 2 t");
    assert_eq!(result.unwrap_err(), "Cannot parse operand \"t\"");
}

/*
#[test]
fn it_fails_for_insufficient_operands() {
    let mut stack: Vec<Stack> = Vec::new();
    let result = evaluate(&mut stack, "1 +");
    assert_eq!(result.unwrap_err(), "Insufficient operands before operator");
}
*/

#[test]
fn it_fails_for_insufficient_operators() {
    let mut stack: Vec<Stack> = Vec::new();
    let result = evaluate(&mut stack, "1 2 3 +");
    assert_eq!(result.unwrap().len(), 2);
}
