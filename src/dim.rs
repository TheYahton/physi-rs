use crate::SIDimension;

impl std::ops::Add for SIDimension {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            l: self.l + rhs.l,
            m: self.m + rhs.m,
            t: self.t + rhs.t,
            i: self.i + rhs.i,
            o: self.o + rhs.o,
            n: self.n + rhs.n,
            j: self.j + rhs.j,
        }
    }
}

impl std::ops::Sub for SIDimension {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            l: self.l - rhs.l,
            m: self.m - rhs.m,
            t: self.t - rhs.t,
            i: self.i - rhs.i,
            o: self.o - rhs.o,
            n: self.n - rhs.n,
            j: self.j - rhs.j,
        }
    }
}

// TODO: refactor this shit, please.
impl std::fmt::Display for SIDimension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut left: Vec<String> = Vec::new();
        let mut right: Vec<String> = Vec::new();

        if self.l != 0 {
            if self.l > 0 {
                if self.l == 1 {
                    left.push("m".to_string())
                } else {
                    left.push(format!("m^{}", self.l))
                }
            } else {
                if self.l == -1 {
                    right.push("m".to_string())
                } else {
                    right.push(format!("m^{}", self.l.abs()))
                }
            }
        }

        if self.m != 0 {
            if self.m > 0 {
                if self.m == 1 {
                    left.push("kg".to_string())
                } else {
                    left.push(format!("kg^{}", self.m))
                }
            } else {
                if self.m == -1 {
                    right.push("kg".to_string())
                } else {
                    right.push(format!("kg^{}", self.m.abs()))
                }
            }
        }

        if self.t != 0 {
            if self.t > 0 {
                if self.t == 1 {
                    left.push("s".to_string())
                } else {
                    left.push(format!("s^{}", self.t))
                }
            } else {
                if self.t == -1 {
                    right.push("s".to_string())
                } else {
                    right.push(format!("s^{}", self.t.abs()))
                }
            }
        }

        if self.i != 0 {
            if self.i > 0 {
                if self.i == 1 {
                    left.push("A".to_string())
                } else {
                    left.push(format!("A^{}", self.i))
                }
            } else {
                if self.i == -1 {
                    right.push("A".to_string())
                } else {
                    right.push(format!("A^{}", self.i.abs()))
                }
            }
        }

        if self.o != 0 {
            if self.o > 0 {
                if self.o == 1 {
                    left.push("K".to_string())
                } else {
                    left.push(format!("K^{}", self.o))
                }
            } else {
                if self.o == -1 {
                    right.push("K".to_string())
                } else {
                    right.push(format!("K^{}", self.o.abs()))
                }
            }
        }

        if self.n != 0 {
            if self.n > 0 {
                if self.n == 1 {
                    left.push("mol".to_string())
                } else {
                    left.push(format!("mol^{}", self.n))
                }
            } else {
                if self.n == -1 {
                    right.push("mol".to_string())
                } else {
                    right.push(format!("mol^{}", self.n.abs()))
                }
            }
        }

        if self.j != 0 {
            if self.j > 0 {
                if self.j == 1 {
                    left.push("cd".to_string())
                } else {
                    left.push(format!("cd^{}", self.j))
                }
            } else {
                if self.j == -1 {
                    right.push("cd".to_string())
                } else {
                    right.push(format!("cd^{}", self.j.abs()))
                }
            }
        }

        if right.is_empty() {
            write!(f, "{}", left.join("*"))
        } else {
            write!(f, "{}/{}", left.join("*"), right.join("*"))
        }
    }
}
